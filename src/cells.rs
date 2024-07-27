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
        if x >= self.width || y >= self.height {
            false
        } else {
            self.grid[y * self.width + x]
        }
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = true;
        }
    }

    pub fn set_dead(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = false;
        }
    }

    pub fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [-1, 0, 1].iter().cloned() {
            for dx in [-1, 0, 1].iter().cloned() {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if nx < self.width && ny < self.height && self.is_alive(nx, ny) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn update(&mut self) {
        let mut new_grid = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                if self.is_alive(x, y) {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        new_grid[y * self.width + x] = false; // underpopulation or overpopulation
                    }
                } else {
                    if alive_neighbors == 3 {
                        new_grid[y * self.width + x] = true; // reproduction
                    }
                }
            }
        }
        self.grid = new_grid;
    }
}
