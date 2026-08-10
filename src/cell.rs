const COLOR_ALIVE: u32 = 0x00FF_FFFF;
const COLOR_DEAD: u32 = 0x0012_1212;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub state: CellState,
    pub color: u32,
}

impl Cell {
    pub fn alive() -> Self {
        Self {
            state: CellState::Alive,
            color: COLOR_ALIVE,
        }
    }

    pub fn dead() -> Self {
        Self {
            state: CellState::Dead,
            color: COLOR_DEAD,
        }
    }

    pub fn from_state(state: CellState) -> Cell {
        match state {
            CellState::Alive => Self::alive(),
            CellState::Dead => Self::dead(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }
}
