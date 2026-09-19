use bevy::prelude::*;

/// Seconds per tile step (diagonal steps take TILE_TIME * sqrt(2)).
const TILE_TIME: f32 = 0.15;

/// Diagonal steps cost sqrt(2) times a straight step, so speed measured in
/// pixels per second stays constant in all 8 directions.
pub fn step_duration(from_cell: IVec2, to_cell: IVec2) -> f32 {
    let diagonal = from_cell.x != to_cell.x && from_cell.y != to_cell.y;
    TILE_TIME
        * if diagonal {
            std::f32::consts::SQRT_2
        } else {
            1.0
        }
}
