use winit::event::{Event, KeyboardInput, WindowEvent};

mod render;
mod screen;

pub struct Config {
    pub background_color: wgpu::Color,
}

pub async fn run() {
    tracing_subscriber::fmt::init();

    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Nyat")
        .build(&event_loop)
        .unwrap();
    let config = Config {
        background_color: wgpu::Color::BLACK,
    };
    let mut screen = screen::Screen::new(window, config).await;
    screen.color_background();
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == screen.window().id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                screen.resize(*size);
                screen.render();
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // TODO: seperate scale setting?
                screen.resize(**new_inner_size);
                screen.render();
            }
            _ => (),
        },
        _ => (),
    });
}
