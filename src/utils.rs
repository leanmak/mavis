use crate::algorithm::Coord;

pub fn abs_to_grid(abs_position: Coord, grid_start: Coord) -> Coord {
    (abs_position.0 - grid_start.0, abs_position.1 - grid_start.1)
}