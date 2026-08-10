use minifb::{Key, Window};

use crate::{
    automaton::Automaton, automaton_selector::AnyAutomaton, cell::Cell, grid::Grid,
    utils::grid_coords,
};

#[derive(PartialEq, Eq)]
pub enum AppState {
    Editing,
    Running,
}

pub struct Renderer {
    window: Window,
    buffer: Vec<u32>,
    grid: Grid<Cell>,
    next_grid: Grid<Cell>,
    width: usize,
    height: usize,
    scale: usize,
    fps: usize,
    automaton: AnyAutomaton,
    state: AppState,
}

impl Renderer {
    pub fn new(
        window: Window,
        automaton: AnyAutomaton,
        width: usize,
        height: usize,
        scale: usize,
        fps: usize,
    ) -> Self {
        // Grids for the current and next generations of cells
        let grid = Grid::new(width, height, Cell::dead());
        let next_grid = Grid::new(width, height, Cell::dead());

        // Pixels buffer
        let buffer = vec![0u32; width * scale * height * scale];

        Self {
            window,
            buffer,
            grid,
            next_grid,
            width,
            height,
            scale,
            fps,
            automaton,
            state: AppState::Editing,
        }
    }

    pub fn render(&mut self) {
        self.window.set_target_fps(self.fps);

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Toggle the application mode when Space is pressed
            if self
                .window
                .is_key_pressed(Key::Space, minifb::KeyRepeat::No)
            {
                match self.state {
                    AppState::Running => self.state = AppState::Editing,
                    AppState::Editing => self.state = AppState::Running,
                }
            }

            // Process mouse input (only in editing mode)
            self.handle_mouse();

            // If the simulation is enabled, advance to the next generations
            if self.state == AppState::Running {
                self.update_next_grid();
                self.grid.copy_from_grid(&self.next_grid);
            }

            // Update the pixel buffer
            self.update_buffer();
            self.window
                .update_with_buffer(
                    &self.buffer,
                    self.width * self.scale,
                    self.height * self.scale,
                )
                .unwrap();
        }
    }

    fn update_next_grid(&mut self) {
        self.automaton
            .step(&self.grid, &mut self.next_grid, self.width, self.height);
    }

    fn update_buffer(&mut self) {
        let cursor = self.automaton.cursor();

        for (x, y) in grid_coords(self.width, self.height) {
            if let Some(cell) = self.grid.get(x, y) {
                for (sx, sy) in grid_coords(self.scale, self.scale) {
                    let screen_x = x * self.scale + sx;
                    let screen_y = y * self.scale + sy;
                    let screen_idx = screen_y * (self.width * self.scale) + screen_x;

                    let color = if cursor == Some((x, y)) {
                        0x00FF_0000
                    } else {
                        cell.color
                    };

                    self.buffer[screen_idx] = color;
                }
            }
        }
    }

    fn grid_coords_from_mouse(&self) -> Option<(usize, usize)> {
        if let Some((mouse_x, mouse_y)) = self.window.get_mouse_pos(minifb::MouseMode::Clamp) {
            let grid_x = (mouse_x / self.scale as f32) as usize;
            let grid_y = (mouse_y / self.scale as f32) as usize;

            if grid_x < self.width && grid_y < self.height {
                Some((grid_x, grid_y))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn handle_mouse(&mut self) {
        if let Some((grid_x, grid_y)) = self.grid_coords_from_mouse()
            && self.state == AppState::Editing
        {
            if self.window.get_mouse_down(minifb::MouseButton::Left) {
                self.grid.set(grid_x, grid_y, Cell::alive());
            } else if self.window.get_mouse_down(minifb::MouseButton::Right) {
                self.grid.set(grid_x, grid_y, Cell::dead());
            }
        }
    }
}
