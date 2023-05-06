use std::ops::{Index, IndexMut};

// this is the way the text is stored
use wgpu_glyph::{ab_glyph::PxScale, Text};

#[derive(Debug)]
pub struct Grid {
    rows: Vec<Row<AnsiChar>>,
    pub size: (u32, u32),
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            rows: vec![Row::new(vec![AnsiChar::default(); width as usize]); height as usize],
            size: (width, height),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.rows.resize(
            height as usize,
            Row::new(vec![AnsiChar::default(); width as usize]),
        );
        for row in self.rows.iter_mut() {
            row.length = width;
            row.row.resize(width as usize, AnsiChar::default());
        }
        self.size = (width, height);
    }

    pub fn write(&mut self, buffer: Vec<u8>) {
        let text = String::from_utf8(buffer).unwrap();
        let mut iter = text.chars();

        let (mut x, mut y) = (0, 0);

        while let Some(c) = iter.next() {
            match c {
                '\n' => {
                    x = 0;
                    y += 1;
                }
                '\r' => {
                    x = 0;
                }
                _ => {
                    if x < self.size.0 as usize && y < self.size.1 as usize {
                        self.rows[y][x] = AnsiChar::new(
                            c.to_string(),
                            [1.0, 1.0, 1.0, 1.0],
                            [0.0, 0.0, 0.0, 1.0],
                        );
                        x += 1;
                    }
                }
            }
        }
    }
}

impl Index<usize> for Grid {
    type Output = Row<AnsiChar>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

#[derive(Debug, Clone)]
pub struct Row<T> {
    pub length: u32,
    row: Vec<T>,
}

impl<T> Row<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self {
            length: v.len() as u32,
            row: v,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.row.iter()
    }
}

impl<T> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.row[index]
    }
}

impl<T> IndexMut<usize> for Row<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.row[index]
    }
}

#[derive(Clone, Debug)]
pub struct AnsiChar {
    pub character: String,
    pub foreground: [f32; 4],
    pub background: [f32; 4],
}

impl AnsiChar {
    pub fn new(character: String, foreground: [f32; 4], background: [f32; 4]) -> Self {
        Self {
            character,
            foreground,
            background,
        }
    }

    pub fn text<'a>(&'a self, scale: f32) -> Text<'a> {
        Text::new(&self.character)
            .with_color(self.foreground)
            .with_scale(PxScale::from(scale))
    }
}

impl Default for AnsiChar {
    fn default() -> Self {
        Self {
            character: " ".to_string(),
            foreground: [0.0, 0.0, 0.0, 1.0],
            background: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

#[derive(Debug)]
pub struct Layout {
    pub scale: f32,
    pub font_size: f32,
    pub px_height: f32,
    pub px_width: f32,
}

impl Layout {
    pub fn new(scale: f32, font_size: f32, px_height: f32, px_width: f32) -> Self {
        Self {
            scale,
            font_size,
            px_height,
            px_width,
        }
    }

    pub fn calculate(&self) -> (u32, u32) {
        let text_width = ((self.px_width / self.scale) / (self.font_size / 2.0)) * 0.96;
        let text_height = (self.px_height as f32 / self.scale) / self.font_size;
        (text_width as u32, text_height as u32)
    }
}
