use crate::{config::ColorPallete, layout::AnsiChar, terminal::Terminal};

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

pub fn display_ansi_text(terminal: &mut Terminal, text: String) {
    // TODO: take config input
    let mut setting = Setting::default();

    let grid = &mut terminal.visible_grid;

    let (_input, ansichars) = parsers::parse(&text).unwrap();

    for c in ansichars {
        match c {
            AnsiSequence::Character(c) if setting.mode == AnsiMode::Title => {}
            AnsiSequence::Character(c) => match c {
                '\n' => {
                    terminal.cursor.1 += 1;
                    terminal.cursor.0 = 0;
                }
                '\r' => {
                    terminal.cursor.0 = 0;
                }
                _ => {
                    if c == 't' {
                        println!(
                            "x: {}, y: {}, {:?}",
                            terminal.cursor.0, terminal.cursor.1, setting.mode
                        );
                    }
                    if terminal.cursor.1 < grid.size.1 && terminal.cursor.0 < grid.size.0 {
                        grid[terminal.cursor.1 as usize][terminal.cursor.0 as usize] =
                            AnsiChar::new(c, setting.color, setting.bg_color);
                        terminal.cursor.0 += 1;
                    } else if terminal.cursor.1 < grid.size.0 {
                        terminal.cursor.1 += 1;
                        terminal.cursor.0 = 0;
                        grid[terminal.cursor.1 as usize][terminal.cursor.0 as usize] =
                            AnsiChar::new(c, setting.color, setting.bg_color);
                        terminal.cursor.0 += 1;
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
            AnsiSequence::CursorUp(n) => {
                if terminal.cursor.1 >= n as u32 {
                    terminal.cursor.1 -= n as u32;
                } else {
                    terminal.cursor.1 = 0;
                }
            }
            AnsiSequence::CursorDown(n) => {
                if terminal.cursor.1 + (n as u32) < grid.size.1 {
                    terminal.cursor.1 += n as u32;
                } else {
                    terminal.cursor.1 = grid.size.1 - 1;
                }
            }
            AnsiSequence::CursorForward(n) => {
                if terminal.cursor.0 + (n as u32) < grid.size.0 {
                    terminal.cursor.0 += n as u32;
                } else {
                    terminal.cursor.0 = grid.size.0 - 1;
                }
            }
            AnsiSequence::CursorBackward(n) => {
                if terminal.cursor.0 >= n as u32 {
                    terminal.cursor.0 -= n as u32;
                } else {
                    terminal.cursor.0 = 0;
                }
            }
            AnsiSequence::Escape => {}
            AnsiSequence::CursorPos(x, y) => {
                if x > grid.size.0 as u16 {
                    terminal.cursor.0 = grid.size.0 - 1;
                } else {
                    terminal.cursor.0 = x as u32;
                }
                if y > grid.size.1 as u16 {
                    terminal.cursor.1 = grid.size.1 - 1;
                } else {
                    terminal.cursor.1 = y as u32;
                }
            }
            AnsiSequence::ShowCursor => terminal.visible_cursor = true,
            AnsiSequence::HideCursor => terminal.visible_cursor = false,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::layout::{Layout, Row};

    use super::*;

    #[test]
    fn test_base() {
        let mut terminal =
            Terminal::new(Layout::new(1.0, 16.0, (16 * 5) as f32, (8 * 5 + 5) as f32));

        display_ansi_text(&mut terminal, "t".to_string());

        let pallete = ColorPallete::default();

        let expected_row = Row::new(vec![
            AnsiChar::new('t', pallete.white, [0.0, 0.0, 0.0, 1.0]),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
        ]);

        assert_eq!(terminal.visible_grid[0], expected_row);
    }

    #[test]
    fn test_csi_red() {
        let mut terminal =
            Terminal::new(Layout::new(1.0, 16.0, (16 * 5) as f32, (8 * 5 + 5) as f32));

        let pallete = ColorPallete::default();

        display_ansi_text(&mut terminal, "\x1b[31mt".to_string());

        let expected_row = Row::new(vec![
            AnsiChar::new('t', pallete.red, [0.0, 0.0, 0.0, 1.0]),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
        ]);

        assert_eq!(terminal.visible_grid[0], expected_row);
    }

    #[test]
    fn test_csi_reset() {
        let mut terminal =
            Terminal::new(Layout::new(1.0, 16.0, (16 * 5) as f32, (8 * 5 + 5) as f32));

        let pallete = ColorPallete::default();

        display_ansi_text(&mut terminal, "\x1b[31mt\x1b[0mt".to_string());

        let expected_row = Row::new(vec![
            AnsiChar::new('t', pallete.red, [0.0, 0.0, 0.0, 1.0]),
            AnsiChar::new('t', pallete.white, [0.0, 0.0, 0.0, 1.0]),
            AnsiChar::default(),
            AnsiChar::default(),
            AnsiChar::default(),
        ]);

        assert_eq!(terminal.visible_grid[0], expected_row);
    }
}
