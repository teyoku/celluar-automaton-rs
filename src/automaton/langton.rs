use crate::{
    automaton::Automaton,
    cell::{
        Cell,
        CellState::{self},
    },
    grid::Grid,
};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

pub struct LangtonAnt {
    x: usize,
    y: usize,
    direction: Direction,
}

impl LangtonAnt {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            x: width / 2,
            y: height / 2,
            direction: Direction::Up,
        }
    }

    fn wrap(value: usize, delta: isize, limit: usize) -> usize {
        (value as isize + delta + limit as isize) as usize % limit
    }
}

impl Automaton for LangtonAnt {
    fn step(&mut self, current: &Grid<Cell>, next: &mut Grid<Cell>, width: usize, height: usize) {
        next.copy_from_grid(current);

        if let Some(cell) = current.get(self.x, self.y) {
            match cell.state {
                CellState::Alive => {
                    self.direction = self.direction.turn_left();
                    next.set(self.x, self.y, Cell::dead());
                }
                CellState::Dead => {
                    self.direction = self.direction.turn_right();
                    next.set(self.x, self.y, Cell::alive());
                }
            }
        }

        let (dx, dy) = self.direction.offset();
        self.x = Self::wrap(self.x, dx, width);
        self.y = Self::wrap(self.y, dy, height);
    }

    fn cursor(&self) -> Option<(usize, usize)> {
        Some((self.x, self.y))
    }
}
