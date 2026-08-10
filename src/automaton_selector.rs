use crate::{
    automaton::{Automaton, ConwayAutomaton, LangtonAnt}, cell::Cell, config::AutomatonKind, grid::Grid,
};

pub enum AnyAutomaton {
    Conway(ConwayAutomaton),
    Langton(LangtonAnt),
}

impl AnyAutomaton {
    pub fn from_kind(kind: AutomatonKind, width: usize, height: usize) -> Self {
        match kind {
            AutomatonKind::Conway => Self::Conway(ConwayAutomaton),
            AutomatonKind::Langton => Self::Langton(LangtonAnt::new(width, height)),
        }
    }
}

impl Automaton for AnyAutomaton {
    fn step(
        &mut self,
        current: &Grid<Cell>,
        next: &mut Grid<Cell>,
        width: usize,
        height: usize,
    ) {
        match self {
            Self::Conway(conway) => conway.step(current, next, width, height),
            Self::Langton(langton) => langton.step(current, next, width, height),
        }
    }

    fn cursor(&self) -> Option<(usize, usize)> {
        match self {
            Self::Conway(conway) => conway.cursor(),
            Self::Langton(langton) => langton.cursor(),
        }
    }
}
