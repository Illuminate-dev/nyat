// holds grid and later on will hold the cursor position
// Also will hold psuedo terminal
use crate::layout::Grid;

#[derive(Debug)]
pub struct Terminal {
    pub grid: Grid,
    pub width: u32,
    pub height: u32,
}

impl Terminal {
    pub fn new(width: u32, height: u32) -> Self {
        let grid = Grid::new(width, height);

        Self {
            grid,
            width,
            height,
        }
    }
}
