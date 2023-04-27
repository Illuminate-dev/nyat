use winit::event::{Event, KeyboardInput, WindowEvent};

mod render;
mod window;

pub async fn run() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Nyat")
        .build(&event_loop)
        .unwrap();
    let win = window::Window::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == win.window().id() => match event {
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
            _ => (),
        },
        _ => (),
    });
}

