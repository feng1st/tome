//! Motion: a picture's current position and its move toward the
//! logical cell, plus the `IsMoving` flag the core reads. One mechanism
//! serves every moving action — a step today, a lunge or a knockback
//! tomorrow; actions that play in place (pickup, eat) belong to sprite
//! animation instead. Display-side only — real time lives here, never
//! in core.

pub mod components;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the motion domain.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::move_sprite::move_sprite.in_set(DisplayPhase::Motion),
    );
}
