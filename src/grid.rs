pub struct Grid<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, initial: T) -> Self {
        Self {
            width,
            height,
            data: vec![initial; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let idx = y * self.width + x;
        self.data.get(idx)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let idx = y * self.width + x;
        self.data.get_mut(idx)
    }

    pub fn set(&mut self, x: usize, y: usize, new: T) {
        if let Some(val) = self.get_mut(x, y) {
            *val = new;
        }
    }

    pub fn copy_from_grid(&mut self, grid: &Grid<T>) {
        self.width = grid.width;
        self.height = grid.height;
        self.data = grid.data.clone();
    }
}
