pub enum AutomatonKind {
    Conway,
    Langton
}

pub struct AppConfig {
    pub automaton: AutomatonKind,
    pub width: usize,
    pub height: usize,
    pub scale: usize,
    pub fps: usize
}