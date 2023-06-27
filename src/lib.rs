// runs the program and for now handles event loop, later event loop will be in screen
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, WindowEvent},
};

use crate::config::Config;

mod config;
mod display;
mod layout;
mod render;
mod screen;
mod terminal;

pub async fn run() {
    tracing_subscriber::fmt::init();

    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Nyat")
        .with_min_inner_size(PhysicalSize::new(50, 20))
        .build(&event_loop)
        .unwrap();
    let config = Config::default();
    let mut screen = screen::Screen::new(window, config).await;
    screen.color_background();

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
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                screen.resize(*size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // TODO: seperate scale setting?
                screen.resize(**new_inner_size);
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match virtual_keycode {
                Some(keycode) => {
                    screen.key_pressed(keycode);
                }
                None => {}
            },
            _ => (),
        },
        Event::RedrawRequested(_) => {
            screen.render();
        }
        _ => (),
    });
}
