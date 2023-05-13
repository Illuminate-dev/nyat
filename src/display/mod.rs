use crate::{
    config::ColorPallete,
    layout::{AnsiChar, Grid},
};

use self::enums::AnsiSequence;

mod enums;
mod parsers;

#[derive(Debug, PartialEq)]
enum AnsiMode {
    Print,
    Title,
}

#[derive(Debug)]
struct Setting {
    pub color: [f32; 4],
    pub bg_color: [f32; 4],
    pub mode: AnsiMode,
    pub pallete: ColorPallete,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0, 1.0],
            bg_color: [0.0, 0.0, 0.0, 1.0],
            mode: AnsiMode::Print,
            pallete: ColorPallete::default(),
        }
    }
}

impl Setting {
    pub fn set_graphics_mode(&mut self, code: Vec<u8>) {
        match code.len() {
            0 => self.color = self.pallete.white,
            x => {
                for i in 0..x {
                    self.set_graphics_mode_1(code[i]);
                }
            }
        }
    }

    fn set_graphics_mode_1(&mut self, code: u8) {
        match code {
            0 => self.color = self.pallete.white,
            30 => self.color = self.pallete.black,
            31 => self.color = self.pallete.red,
            32 => self.color = self.pallete.green,
            33 => self.color = self.pallete.yellow,
            34 => self.color = self.pallete.blue,
            35 => self.color = self.pallete.magenta,
            36 => self.color = self.pallete.cyan,
            37 => self.color = self.pallete.white,
            40 => self.bg_color = self.pallete.black,
            41 => self.bg_color = self.pallete.red,
            42 => self.bg_color = self.pallete.green,
            43 => self.bg_color = self.pallete.yellow,
            44 => self.bg_color = self.pallete.blue,
            45 => self.bg_color = self.pallete.magenta,
            46 => self.bg_color = self.pallete.cyan,
            47 => self.bg_color = self.pallete.white,
            _ => {}
        }
    }
}

pub fn display_ansi_text(grid: &mut Grid, text: String) {
    // TODO: take config input
    let mut setting = Setting::default();

    let mut x = 0;
    let mut y = 0;

    let (_input, ansichars) = parsers::parse(&text).unwrap();

    for c in ansichars {
        match c {
            AnsiSequence::Character(c) if setting.mode == AnsiMode::Title => {}
            AnsiSequence::Character(c) => match c {
                '\n' => {
                    y += 1;
                    x = 0;
                }
                '\r' => {
                    x = 0;
                }
                _ => {
                    if c == 't' {
                        println!("x: {}, y: {}, {:?}", x, y, setting.mode);
                    }
                    if y < grid.size.1 && x < grid.size.0 {
                        grid[y as usize][x as usize] =
                            AnsiChar::new(c, setting.color, setting.bg_color);
                        x += 1;
                    } else if y < grid.size.0 {
                        y += 1;
                        x = 0;
                        grid[y as usize][x as usize] =
                            AnsiChar::new(c, setting.color, setting.bg_color);
                        x += 1;
                    }
                }
            },
            AnsiSequence::SetGraphicsMode(codes) => {
                setting.set_graphics_mode(codes);
            }
            AnsiSequence::SetTitleMode => {
                setting.mode = AnsiMode::Title;
            }
            AnsiSequence::Bell => {
                setting.mode = AnsiMode::Print;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::layout::Row;

    use super::*;

    #[test]
    fn test_base() {
        let mut grid = Grid::new(5, 10);

        display_ansi_text(&mut grid, "t".to_string());

        let pallete = ColorPallete::default();

        let expected_row = Row::new(vec![
            AnsiChar::new('t', pallete.white, [0.0, 0.0, 0.0, 1.0]),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
        ]);

        assert_eq!(grid[0], expected_row);
    }

    #[test]
    fn test_csi_red() {
        let mut grid = Grid::new(5, 10);

        let pallete = ColorPallete::default();

        display_ansi_text(&mut grid, "\x1b[31mt".to_string());

        let expected_row = Row::new(vec![
            AnsiChar::new('t', pallete.red, [0.0, 0.0, 0.0, 1.0]),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
        ]);

        assert_eq!(grid[0], expected_row);
    }

    #[test]
    fn test_csi_reset() {
        let mut grid = Grid::new(5, 10);

        let pallete = ColorPallete::default();

        display_ansi_text(&mut grid, "\x1b[31mt\x1b[0mt".to_string());

        let expected_row = Row::new(vec![
            AnsiChar::new('t', pallete.red, [0.0, 0.0, 0.0, 1.0]),
            AnsiChar::new('t', pallete.white, [0.0, 0.0, 0.0, 1.0]),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
        ]);

        assert_eq!(grid[0], expected_row);
    }
}
