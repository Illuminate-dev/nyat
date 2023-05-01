use crate::render::Renderer;

pub struct Screen {
    renderer: Renderer,
    background_color: wgpu::Color,
}

impl Screen {
    pub async fn new(window: winit::window::Window, config: crate::Config) -> Self {
        Self {
            renderer: Renderer::new(window).await,
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
    }

    pub fn render(&mut self) {
        self.renderer.render();
    }
}
