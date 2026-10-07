//! Move: arm the glide whenever a creature's logical cell lands.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::frontend::display::motion::components::curr_position::CurrPosition;
use crate::frontend::display::motion::constants::move_interval::MOVE_INTERVAL;
use crate::frontend::display::tween::components::pos_tween::PosTween;

/// Arm the glide for every freshly landed logical cell: the picture's
/// position tweens toward the cell over one move interval. A step that
/// lands while a glide is still running re-arms from where the picture
/// stands — pictures may overlap; only the tick ledger is serial. The
/// spawn cell needs no glide (the picture stands on it already).
pub fn move_cells(
    mut commands: Commands,
    creatures: Query<(Entity, &CellCoord, &CurrPosition), Changed<CellCoord>>,
) {
    for (entity, cell, curr_position) in &creatures {
        let to = Vec2::new(cell.x as f32, cell.y as f32);
        let from = Vec2::new(curr_position.x, curr_position.y);
        if from.distance_squared(to) < 1e-12 {
            // Already standing there (a spawn, or a no-op rewrite).
            continue;
        }
        commands.entity(entity).insert(PosTween {
            from,
            to,
            elapsed: 0.0,
            interval: MOVE_INTERVAL,
        });
    }
}
