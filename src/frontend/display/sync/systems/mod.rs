//! Position sync systems.

pub mod sync_position;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register sync systems into the Sync phase.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        sync_position::sync_position.in_set(DisplayPhase::Sync),
    );
}
