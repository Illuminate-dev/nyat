// runs the program and for now handles event loop, later event loop will be in screen
use layout::AnsiChar;
use winit::event::{Event, KeyboardInput, WindowEvent};

mod layout;
mod render;
mod screen;
mod terminal;

pub struct Config {
    pub background_color: wgpu::Color,
    pub scale: f32,
    pub font_size: f32,
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
        scale: 1.0,
        font_size: 16.0,
    };
    let mut screen = screen::Screen::new(window, config).await;
    screen.color_background();

    screen.terminal.grid[0][0].character = "0".to_string();
    for i in 1..screen.terminal.height {
        screen.terminal.grid[i as usize][0].character = i.to_string();
        screen.terminal.grid[i as usize][0].foreground[0] = 1.0;
    }

    for i in 1..screen.terminal.width {
        screen.terminal.grid[0][i as usize].character = i.to_string()[0..1].to_string();
        screen.terminal.grid[0][i as usize].foreground[0] = 1.0;
    }

    println!(
        "height: {}, width: {}",
        screen.terminal.height, screen.terminal.width
    );

    println!(
        "window height: {}, width: {}",
        screen.window().inner_size().height,
        screen.window().inner_size().width
    );

    println!("window scale factor: {}", screen.window().scale_factor());
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
                screen.window().request_redraw();
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // TODO: seperate scale setting?
                screen.resize(**new_inner_size);
                screen.window().request_redraw();
            }
            _ => (),
        },
        Event::RedrawRequested(_) => {
            screen.render();
        }
        _ => (),
    });
}
