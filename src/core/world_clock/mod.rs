//! The world clock: logical time, persistent turn slots, and the
//! moving flag it waits on.

pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;

/// Register the world clock domain: members only; ordering lives in the
/// core root.
pub fn register(app: &mut App) {
    app.init_resource::<resources::world_clock::WorldClock>()
        .add_systems(Update, systems::advance::advance.in_set(CorePhase::Advance));
}
