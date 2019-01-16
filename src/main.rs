#[macro_use]
extern crate glium;
use glium::{Display, index, glutin, Surface, VertexBuffer};
use glium::backend::{Facade};
use glium::glutin::{Event, ElementState, MouseButton, WindowEvent, VirtualKeyCode};
#[macro_use]
extern crate imgui;
use imgui::{ImGui, ImGuiCond};
use imgui_glium_renderer::{Renderer, RendererResult};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2]
}
implement_vertex!(Vertex, position);


#[derive(Debug)]
struct InputState {
    mouse_x: f32,
    mouse_y: f32,
    mouse_left: bool,
    mouse_mid: bool,
    mouse_right: bool
}

impl InputState {
    fn empty() -> Self {
        InputState { 
            mouse_x: 0.0, mouse_y: 0.0, 
            mouse_left: false, 
            mouse_mid: false,
            mouse_right: false 
        }
    }

    fn handle_event(&mut self, we: &WindowEvent) {
        match we {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_x = position.x as f32;
                self.mouse_y = position.y as f32;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                match button {
                    MouseButton::Left => self.mouse_left = is_pressed,
                    MouseButton::Middle => self.mouse_mid = is_pressed,
                    MouseButton::Right => self.mouse_right = is_pressed,
                    MouseButton::Other(_) => {}
                }
            }
            _ => {}
        }
    }
}


struct Gui {
    imgui: ImGui,
    renderer: Renderer,
    input: InputState,
    dark_plot: bool
}

impl Gui {
    fn init<F: Facade>(facade: &F) -> RendererResult<Self> {
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);
        imgui.set_font_global_scale(1.2);
        let input = InputState::empty();
        Renderer::init(&mut imgui, facade).map(|renderer| {
            Gui { imgui, renderer, input, dark_plot: false }
        })
    }

    fn handle_event(&mut self, we: &WindowEvent) {
        self.input.handle_event(we);
    }

    fn update_input(&mut self) {
        self.imgui.set_mouse_pos(self.input.mouse_x, self.input.mouse_y);
        let buttons = [self.input.mouse_left, self.input.mouse_mid, self.input.mouse_right, false, false];
        self.imgui.set_mouse_down(buttons);
    }

    fn draw_ui<S: glium::Surface>(&mut self, target: &mut S, display: &glium::Display) {
        self.update_input();
        let (w, h) = display.gl_window().get_inner_size().unwrap().into();
        let hidpi = display.gl_window().get_hidpi_factor();
        let ui = self.imgui.frame(imgui::FrameSize::new(w, h, hidpi), 0.1);
        let mut dark_plot = self.dark_plot;
        ui.window(im_str!("Controls"))
            .size((200.0, 100.0), ImGuiCond::Always)
            .position((w as f32 - 220.0, 20.0), ImGuiCond::Always)
            .build(|| {
                let text = if dark_plot { 
                    im_str!("Light") 
                } else { 
                    im_str!("Dark")
                };
                if ui.small_button(text) {
                    dark_plot = !dark_plot;
                };
            });
        self.dark_plot = dark_plot;
        self.renderer.render(target, ui)
            .expect("Failed to draw UI");
    }

    fn is_dark_plot(&self) -> bool {
        self.dark_plot
    }
}


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Cauchy".to_string())
        .with_dimensions((600, 600).into());
    let context = glutin::ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();

    let mut gui = Gui::init(&display).unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("cauchy.glslv"),
        include_str!("cauchy.glslf"),
        None
    ).unwrap();

    let vertices = [
        Vertex{ position: [-1.0,  1.0]},
        Vertex{ position: [ 1.0,  1.0]},
        Vertex{ position: [-1.0, -1.0]},

        Vertex{ position: [-1.0, -1.0]},
        Vertex{ position: [ 1.0,  1.0]},
        Vertex{ position: [ 1.0, -1.0]}
    ];

    let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();

    loop {
        let mut target = display.draw();

        let uniforms = uniform! {
            u_dark_plot: gui.is_dark_plot()
        };
        target.draw(
            &vertex_buffer,
            &index::NoIndices(index::PrimitiveType::TrianglesList),
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();

        gui.draw_ui(&mut target, &display);

        target.finish().unwrap();

        let mut should_return = false;
        events_loop.poll_events(|e| match e {
            Event::WindowEvent { event, .. } => {
                gui.handle_event(&event);
                match event {
                    WindowEvent::CloseRequested => should_return = true,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                            should_return = true;
                        }
                    }
                    _ => {}
                }
           }
            _ => {}
        });
        if should_return {
            return;
        }
    }
}
