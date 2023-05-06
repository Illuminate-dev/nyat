// Pieces the renderer and the terminal together

use crate::{render::Renderer, terminal::Terminal};

pub struct Screen {
    renderer: Renderer,
    pub terminal: Terminal,
    background_color: wgpu::Color,
}

impl Screen {
    pub async fn new(window: winit::window::Window, config: crate::Config) -> Self {
        let size = window.inner_size();

        let layout = crate::layout::Layout::new(
            config.scale,
            config.font_size,
            size.height as f32,
            size.width as f32,
        );

        let (text_width, text_height) = layout.calculate();

        println!("height: {}, width: {}", text_height, text_width);

        Self {
            renderer: Renderer::new(window, config.scale, config.font_size).await,
            terminal: Terminal::new(layout),
            background_color: config.background_color,
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        self.renderer.window()
    }

    pub fn color_background(&mut self) {
        self.renderer.color_background(self.background_color);
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(size);
        self.terminal.resize(size);
    }

    pub fn render(&mut self) {
        self.renderer.draw_text(&self.terminal.visible_grid);
        self.renderer.render();
    }

    pub fn key_pressed(&mut self, key: &winit::event::VirtualKeyCode) {
        println!("key pressed: {:?}", key);
    }
}
