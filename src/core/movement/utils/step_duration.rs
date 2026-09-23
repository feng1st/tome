//! Per-step durations for tile-by-tile movement.

use crate::core::map::cell_pos::CellPos;

/// Seconds per tile step (diagonal steps take TILE_TIME * sqrt(2)).
const TILE_TIME: f32 = 0.15;

/// Diagonal steps cost sqrt(2) times a straight step, so speed measured in
/// pixels per second stays constant in all 8 directions.
pub fn step_duration(from_cell: CellPos, to_cell: CellPos) -> f32 {
    let (from, to) = (from_cell.0, to_cell.0);
    let diagonal = from.x != to.x && from.y != to.y;
    TILE_TIME
        * if diagonal {
            std::f32::consts::SQRT_2
        } else {
            1.0
        }
}
