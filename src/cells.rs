pub struct Cells {
    pub width: usize,
    pub height: usize,
    grid: Vec<Vec<bool>>,
}

impl Cells {
    // Inicializa una nueva matriz con todas las células muertas
    pub fn new(width: usize, height: usize) -> Self {
        Cells {
            width,
            height,
            grid: vec![vec![false; width]; height],
        }
    }

    // Establece una célula como viva
    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.grid[y][x] = true;
        }
    }

    // Establece una célula como muerta
    pub fn set_dead(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.grid[y][x] = false;
        }
    }

    // Verifica si una célula está viva
    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.grid[y][x]
        } else {
            false
        }
    }

    // Limpia la matriz (todas las células muertas)
    pub fn clear(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = false;
            }
        }
    }
}
