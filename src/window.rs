use crate::render::Renderer;

pub struct Window {
    renderer: Renderer,
}

impl Window {
    pub async fn new(window: winit::window::Window) -> Self {
        Self {
            renderer: Renderer::new(window).await,
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        self.renderer.window()
    }
}
