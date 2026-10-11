use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::frontend::display::constants::layout::TILE_SIZE;
use crate::frontend::display::motion::components::curr_position::CellPosition;

pub fn cell_to_world_position(cell_position: CellPosition) -> Vec2 {
    Vec2::new(
        (cell_position.x + 0.5) * TILE_SIZE as f32,
        -(cell_position.y + 0.5) * TILE_SIZE as f32,
    )
}

pub fn world_position_to_cell_coord(world_position: Vec2) -> CellCoord {
    CellCoord::new(
        (world_position.x / TILE_SIZE as f32).floor() as i32,
        (-world_position.y / TILE_SIZE as f32).floor() as i32,
    )
}
