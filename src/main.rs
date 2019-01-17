mod parser;

#[macro_use]
extern crate nom;
#[macro_use]
extern crate glium;
use glium::{Display, index, glutin, Surface, VertexBuffer};
use glium::backend::{Facade};
use glium::glutin::{Event, ElementState, MouseButton, WindowEvent, VirtualKeyCode};
use glium::uniforms::UniformBuffer;
#[macro_use]
extern crate imgui;
use imgui::{ImGui, ImGuiCond, ImString};
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
    plot_str: ImString,
    dark_plot: bool
}

impl Gui {
    fn init<F: Facade>(facade: &F) -> RendererResult<Self> {
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);
        imgui.set_font_global_scale(1.2);
        let input = InputState::empty();
        // This should be sufficient for all the text we have
        let plot_str = ImString::with_capacity(256);
        Renderer::init(&mut imgui, facade).map(|renderer| {
            Gui { imgui, renderer, input, plot_str, dark_plot: false }
        })
    }

    fn handle_event(&mut self, we: &WindowEvent) {
        self.input.handle_event(we);
        match we {
            WindowEvent::KeyboardInput{ input, .. } => {
                if input.state == ElementState::Pressed {
                    let chr = match input.virtual_keycode {
                        Some(VirtualKeyCode::A) => Some('a'),
                        Some(VirtualKeyCode::B) => Some('b'),
                        Some(VirtualKeyCode::C) => Some('c'),
                        Some(VirtualKeyCode::D) => Some('d'),
                        Some(VirtualKeyCode::E) => Some('e'),
                        Some(VirtualKeyCode::F) => Some('f'),
                        Some(VirtualKeyCode::G) => Some('g'),
                        Some(VirtualKeyCode::H) => Some('h'),
                        Some(VirtualKeyCode::I) => Some('i'),
                        Some(VirtualKeyCode::J) => Some('j'),
                        Some(VirtualKeyCode::K) => Some('k'),
                        Some(VirtualKeyCode::L) => Some('l'),
                        Some(VirtualKeyCode::M) => Some('m'),
                        Some(VirtualKeyCode::N) => Some('n'),
                        Some(VirtualKeyCode::O) => Some('o'),
                        Some(VirtualKeyCode::P) => Some('p'),
                        Some(VirtualKeyCode::Q) => Some('q'),
                        Some(VirtualKeyCode::R) => Some('r'),
                        Some(VirtualKeyCode::S) => Some('s'),
                        Some(VirtualKeyCode::T) => Some('t'),
                        Some(VirtualKeyCode::U) => Some('u'),
                        Some(VirtualKeyCode::V) => Some('v'),
                        Some(VirtualKeyCode::W) => Some('w'),
                        Some(VirtualKeyCode::X) => Some('x'),
                        Some(VirtualKeyCode::Y) => Some('y'),
                        Some(VirtualKeyCode::Z) => Some('z'),
                        Some(VirtualKeyCode::LBracket) => Some('('),
                        Some(VirtualKeyCode::RBracket) => Some(')'),
                        Some(VirtualKeyCode::Back) => {
                            self.plot_str.clear();
                            None
                        }
                        _ => None
                    };
                    if let Some(c) = chr {
                        self.plot_str.push(c);
                    }
                }
            }
            _ => {}
        }
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
        let mut plot_str = self.plot_str.clone();
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
                ui.input_text(im_str!("Expr"), &mut plot_str)
                    .build();
            });
        self.dark_plot = dark_plot;
        self.plot_str = plot_str;
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

        let tokens: [i32; 10] = [1, 7, 0, 0, 0, 0, 0, 0, 0, 0];
        let token_buf = UniformBuffer::new(&display, tokens).unwrap();
        let floats: [f32; 10] = [2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let float_buf = UniformBuffer::new(&display, floats).unwrap();
        let uniforms = uniform! {
            u_dark_plot: gui.is_dark_plot(),
            Tokens: { &token_buf },
            Floats: { &float_buf }
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
