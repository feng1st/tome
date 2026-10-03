//! The world clock: logical time, action slots, and the hero-driven
//! time gate.

pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;

/// Register the time domain: members only; ordering lives in the core root.
pub fn register(app: &mut App) {
    app.init_resource::<resources::world_clock::WorldClock>()
        .add_systems(Update, systems::advance::advance.in_set(CorePhase::Advance));
}
