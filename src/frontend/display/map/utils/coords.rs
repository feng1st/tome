// TODO: pending cleanup review — remove once stabilized
//! Cell <-> world-pixel conversion. The core counts in continuous cell
//! coordinates and knows nothing about pixels; these functions are the
//! display side's only bridge between the two spaces.

use bevy::prelude::*;

use crate::core::map::types::cell_coord::CellCoord;
use crate::core::movement::components::position::Position;
use crate::frontend::display::map::constants::layout::TILE_SIZE;

/// World-pixel position of a cell-space position. Integer cell coordinates
/// are cell centers, hence the half-cell offset. Map row 0 is the top row
/// while world Y points up, so world y is negative.
pub fn cell_to_world(pos: Position) -> Vec2 {
    Vec2::new(
        (pos.x + 0.5) * TILE_SIZE as f32,
        -(pos.y + 0.5) * TILE_SIZE as f32,
    )
}

/// Cell containing a world-pixel position.
pub fn world_to_cell(world: Vec2) -> CellCoord {
    CellCoord::new(
        (world.x / TILE_SIZE as f32).floor() as i32,
        (-world.y / TILE_SIZE as f32).floor() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_world_roundtrip() {
        let cell = CellCoord::new(30, 20);
        assert_eq!(world_to_cell(cell_to_world(Position::from(cell))), cell);
    }
}
