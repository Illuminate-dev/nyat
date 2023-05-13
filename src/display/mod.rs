use crate::{
    config::ColorPallete,
    layout::{AnsiChar, Grid},
};

use self::enums::AnsiSequence;

mod enums;
mod parsers;

#[derive(Debug)]
enum AnsiMode {
    Print,
    Title,
}

#[derive(Debug)]
struct Setting {
    pub color: [f32; 4],
    pub mode: AnsiMode,
    pub pallete: ColorPallete,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0, 1.0],
            mode: AnsiMode::Print,
            pallete: ColorPallete::default(),
        }
    }
}

impl Setting {
    pub fn update(&mut self, code: u8) {
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
            _ => self.color = self.pallete.white,
        }
    }
}

pub fn display_ansi_text(grid: &mut Grid, text: String) {
    // TODO: take config input
    let mut setting = Setting::default();

    let mut x = 0;
    let mut y = 0;

    let (input, ansichars) = parsers::parse(&text).unwrap();

    for c in ansichars {
        match c {
            AnsiSequence::Character(c) => match c {
                '\n' => {
                    y += 1;
                    x = 0;
                }
                '\r' => {
                    x = 0;
                }
                _ => {
                    if y < grid.size.1 && x < grid.size.0 {
                        grid[y as usize][x as usize] =
                            AnsiChar::new(c, setting.color, [0.0, 0.0, 0.0, 1.0]);
                        x += 1;
                    } else if y < grid.size.0 {
                        y += 1;
                        x = 0;
                    }
                }
            },
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
