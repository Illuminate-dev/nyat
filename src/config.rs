pub struct Config {
    pub color_pallete: ColorPallete,
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
