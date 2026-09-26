//! Position sync: derives render transforms from the core's authoritative
//! cell-space positions.

pub mod systems;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the movement domain: render transforms sync from the core's
/// positions in the Sync phase.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::sync_position::sync_position.in_set(DisplayPhase::Sync),
    );
}
