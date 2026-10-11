pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;
use crate::core::world_clock::resources::world_clock::WorldClock;

pub fn register(app: &mut App) {
    app.init_resource::<WorldClock>().add_systems(
        Update,
        systems::advance_world::advance_world.in_set(CorePhase::WorldAdvance),
    );
}
