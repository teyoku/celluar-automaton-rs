use crate::{cell::Cell, grid::Grid};

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn count_neighbors(grid: &Grid<Cell>, x: usize, y: usize, width: usize, height: usize) -> u8 {
    NEIGHBOR_OFFSETS
        .iter()
        .filter_map(|(dx, dy)| {
            let nx = (x as isize + dx + width as isize) as usize % width;
            let ny = (y as isize + dy + height as isize) as usize % height;

            if let Some(cell) = grid.get(nx, ny)
                && cell.is_alive()
            {
                Some(cell)
            } else {
                None
            }
        })
        .count() as u8
}

pub fn grid_coords(width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
}
