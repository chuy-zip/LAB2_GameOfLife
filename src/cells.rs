pub struct Cells {
    pub width: usize,
    pub height: usize,
    grid: Vec<bool>, // true for alive, false for dead
}

impl Cells {
    pub fn new(width: usize, height: usize) -> Self {
        Cells {
            width,
            height,
            grid: vec![false; width * height],
        }
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.grid[y * self.width + x]
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        self.grid[y * self.width + x] = true;
    }

    pub fn set_dead(&mut self, x: usize, y: usize) {
        self.grid[y * self.width + x] = false;
    }
}
