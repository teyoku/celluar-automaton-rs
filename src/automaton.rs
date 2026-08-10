mod conway;
mod langton;

use crate::{cell::Cell, grid::Grid};
pub use conway::ConwayAutomaton;
pub use langton::LangtonAnt;

pub trait Automaton {
    fn step(&mut self, current: &Grid<Cell>, next: &mut Grid<Cell>, width: usize, height: usize);

    fn cursor(&self) -> Option<(usize, usize)>;
}
