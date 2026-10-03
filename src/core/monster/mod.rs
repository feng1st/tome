//! The monster domain's game-data side: the kind vocabulary, its
//! registry, and spawning. Appearance lives in the frontend.

pub mod components;
pub mod entities;
pub mod resources;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use self::resources::monster_registry::MonsterRegistry;
use crate::core::app_state::AppState;
use crate::core::core_phase::CorePhase;

/// Register the monster domain: the vocabulary registry builds at app
/// build time (`FromWorld`), monsters spawn on entering the game, and
/// `plan_wander` plans in the Plan phase.
pub fn register(app: &mut App) {
    app.init_resource::<MonsterRegistry>()
        .add_systems(OnEnter(AppState::Game), entities::monsters::spawn_monsters)
        .add_systems(
            Update,
            systems::plan_wander::plan_wander.in_set(CorePhase::WorldPlan),
        );
}
