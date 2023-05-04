use wgpu::Color;
// renderer has a height, width, and scale. It uses these values to render text to screen
use wgpu_glyph::{ab_glyph::FontArc, OwnedText, Section, Text};

use crate::layout::{AnsiChar, Grid};

mod state;

pub struct Renderer {
    state: state::State,
    brush: wgpu_glyph::GlyphBrush<()>,
    scale: f32,
    font_size: f32,
}

impl Renderer {
    pub async fn new(window: winit::window::Window, scale: f32, font_size: f32) -> Self {
        let state = state::State::new(window).await;

        let font =
            FontArc::try_from_slice(include_bytes!("FantasqueSansMono-Regular.ttf")).unwrap();

        let brush = wgpu_glyph::GlyphBrushBuilder::using_font(font)
            .build(&state.device, state.config.format);

        Self {
            state,
            brush,
            scale,
            font_size,
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.state.window()
    }

    fn render_full(&mut self, load_op: wgpu::LoadOp<Color>) {
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
                    load: load_op,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.brush
            .draw_queued(
                &self.state.device,
                &mut self.state.staging_belt,
                &mut encoder,
                view,
                self.state.size.width,
                self.state.size.height,
            )
            .expect("Draw queued");

        self.state.staging_belt.finish();
        self.state.queue.submit(Some(encoder.finish()));
        frame.present();
        self.state.staging_belt.recall();
    }

    pub fn render(&mut self) {
        self.render_full(wgpu::LoadOp::Load);
    }

    pub fn color_background(&mut self, color: wgpu::Color) {
        self.render_full(wgpu::LoadOp::Clear(color));
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.state.resize(new_size);
    }

    pub fn draw_text(&mut self, grid: &Grid) {
        for i in 0..(grid.size.1 as usize) {
            let mut texts: Vec<Text> = vec![];
            for ansichar in grid[i].iter() {
                texts.push(ansichar.text(self.font_size));
            }

            // TODO: calculate different heights, etc.

            self.brush.queue(Section {
                screen_position: (0.0, (i * 20) as f32),
                bounds: (self.state.size.width as f32, self.state.size.height as f32),
                text: texts,
                layout: wgpu_glyph::Layout::default_single_line(),
            })
        }
    }
}
