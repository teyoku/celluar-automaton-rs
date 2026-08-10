mod automaton;
mod automaton_selector;
mod cell;
mod cli;
mod config;
mod error;
mod grid;
mod renderer;
mod utils;

use crate::{automaton_selector::AnyAutomaton, renderer::Renderer};
use minifb::{Window, WindowOptions};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match cli::parse_args(args) {
        Ok(config) => {
            let window_width = config.width * config.scale;
            let window_height = config.height * config.scale;

            let window = Window::new(
                "celluar-automaton-rs",
                window_width,
                window_height,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| panic!("{e}"));

            let automaton = AnyAutomaton::from_kind(config.automaton, config.width, config.height);
            let mut renderer = Renderer::new(
                window,
                automaton,
                config.width,
                config.height,
                config.scale,
                config.fps,
            );
            renderer.render();
        }
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    }
}
