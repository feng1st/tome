// TODO: pending cleanup review — remove once stabilized
//! Per-step durations for tile-by-tile movement.

use crate::core::map::types::cell_coord::CellCoord;

/// Seconds per tile step (diagonal steps take TILE_TIME * sqrt(2)).
const TILE_TIME: f32 = 0.15;

/// Diagonal steps cost sqrt(2) times a straight step, so speed measured in
/// pixels per second stays constant in all 8 directions.
pub fn step_duration(from_cell: CellCoord, to_cell: CellCoord) -> f32 {
    let diagonal = from_cell.x != to_cell.x && from_cell.y != to_cell.y;
    TILE_TIME
        * if diagonal {
            std::f32::consts::SQRT_2
        } else {
            1.0
        }
}
