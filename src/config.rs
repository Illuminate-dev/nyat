pub struct Config {
    pub color_pallete: ColorPallete,
    pub background_color: wgpu::Color,
    pub scale: f32,
    pub font_size: f32,
    pub cursor: char,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            color_pallete: ColorPallete::default(),
            background_color: wgpu::Color::BLACK,
            scale: 1.0,
            font_size: 16.0,
            cursor: '█',
        }
    }
}

#[derive(Debug)]
pub struct ColorPallete {
    pub black: [f32; 4],
    pub red: [f32; 4],
    pub green: [f32; 4],
    pub yellow: [f32; 4],
    pub blue: [f32; 4],
    pub magenta: [f32; 4],
    pub cyan: [f32; 4],
    pub white: [f32; 4],
}

impl Default for ColorPallete {
    fn default() -> Self {
        Self {
            black: [0.0, 0.0, 0.0, 1.0],
            red: [1.0, 0.0, 0.0, 1.0],
            green: [0.0, 1.0, 0.0, 1.0],
            yellow: [1.0, 1.0, 0.0, 1.0],
            blue: [0.0, 0.0, 1.0, 1.0],
            magenta: [1.0, 0.0, 1.0, 1.0],
            cyan: [0.0, 1.0, 1.0, 1.0],
            white: [1.0, 1.0, 1.0, 1.0],
        }
    }
}
