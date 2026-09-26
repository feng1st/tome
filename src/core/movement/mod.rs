// TODO: pending cleanup review — remove once stabilized
//! Tile-step movement on the grid: paths, authoritative positions, and the
//! per-step tween. Pure game logic — no rendering types involved.

pub mod components;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;

/// Register the movement domain.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::follow_path::follow_path.in_set(CorePhase::Act),
    );
}
