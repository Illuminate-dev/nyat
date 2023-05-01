mod state;

pub struct Renderer {
    state: state::State,
}

impl Renderer {
    pub async fn new(window: winit::window::Window) -> Self {
        let state = state::State::new(window).await;

        Self { state }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.state.window()
    }

    pub fn color_background(&mut self, color: wgpu::Color) {
        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Background Color Encoder"),
                });

        let frame = self
            .state
            .surface
            .get_current_texture()
            .expect("Unable to get texture");

        let view = &frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Background Color Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.state.staging_belt.finish();
        self.state.queue.submit(Some(encoder.finish()));
        frame.present();
        self.state.staging_belt.recall();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.state.resize(new_size);
    }

    pub fn render(&mut self) {
        let frame = self
            .state
            .surface
            .get_current_texture()
            .expect("Unable to get texture");

        let view = &frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.state.staging_belt.finish();
        self.state.queue.submit(Some(encoder.finish()));
        frame.present();
        self.state.staging_belt.recall();
    }
}
