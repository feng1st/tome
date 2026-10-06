//! Tile-step movement on the grid: the step action and logical
//! positions. Route computation is the planning side's business.
//! Pure game logic — no rendering types, no real time involved.

pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;

/// Register the movement domain.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        (
            systems::act_move::act_move.in_set(CorePhase::PlayerAct),
            systems::act_move::act_move.in_set(CorePhase::WorldAct),
        ),
    );
}
