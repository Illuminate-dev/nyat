// Pieces the renderer and the terminal together

use crate::{render::Renderer, terminal::Terminal};

pub struct Screen {
    renderer: Renderer,
    pub terminal: Terminal,
    background_color: wgpu::Color,
    scale: f32,
    font_size: f32,
}

impl Screen {
    pub async fn new(window: winit::window::Window, config: crate::Config) -> Self {
        let size = window.inner_size();

        let text_width = ((size.width as f32 / config.scale) / (config.font_size / 2.0)) * 0.96;
        let text_height = (size.height as f32 / config.scale) / config.font_size;

        println!("height: {}, width: {}", text_height, text_width);

        Self {
            renderer: Renderer::new(window, config.scale, config.font_size).await,
            terminal: Terminal::new(text_width as u32, text_height as u32),
            background_color: config.background_color,
            scale: config.scale,
            font_size: config.font_size,
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
    }

    pub fn render(&mut self) {
        self.renderer.draw_text(&self.terminal.grid);
        self.renderer.render();
    }
}
