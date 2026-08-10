use crate::{
    automaton::Automaton,
    cell::{Cell, CellState},
    grid::Grid,
    utils::{count_neighbors, grid_coords},
};

pub struct ConwayAutomaton;

impl Automaton for ConwayAutomaton {
    fn step(&mut self, current: &Grid<Cell>, next: &mut Grid<Cell>, width: usize, height: usize) {
        for (x, y) in grid_coords(width, height) {
            // Count the number of live neighbors around the cell
            let neighbors = count_neighbors(current, x, y, width, height);

            if let Some(cell) = current.get(x, y) {
                // Calculate the cell's next-generation state
                let next_state = next_state(cell.state, neighbors);

                // Create a new cell with this state
                let next_cell = Cell::from_state(next_state);

                // Add the cell to the next generation grid
                next.set(x, y, next_cell);
            }
        }
    }

    fn cursor(&self) -> Option<(usize, usize)> {
        None
    }
}

fn next_state(current: CellState, neighbors: u8) -> CellState {
    match (current, neighbors) {
        (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
        (CellState::Dead, 3) => CellState::Alive,
        _ => CellState::Dead,
    }
}
