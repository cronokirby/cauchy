#[macro_use]
extern crate glium;
use glium::{Display, index, glutin, Surface, VertexBuffer};
use glium::glutin::{Event, WindowEvent, VirtualKeyCode};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2]
}
implement_vertex!(Vertex, position);


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Cauchy".to_string())
        .with_dimensions((800, 800).into());
    let context = glutin::ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();

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
            u_dark_plot: false
        };

        target.draw(
            &vertex_buffer,
            &index::NoIndices(index::PrimitiveType::TrianglesList),
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();
        target.finish().unwrap();

        let mut should_return = false;
        events_loop.poll_events(|e| match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => should_return = true,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        should_return = true;
                    }
                }
                _ => {}
            }
            _ => {}
        });
        if should_return {
            return;
        }
    }
}
