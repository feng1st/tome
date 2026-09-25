//! Camera systems.

pub mod follow_target;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register camera systems: target following runs in the Camera phase.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        follow_target::follow_target.in_set(DisplayPhase::Camera),
    );
}
