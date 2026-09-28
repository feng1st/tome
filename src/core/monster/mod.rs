//! The monster domain's game-data side: the kind vocabulary, its
//! registry, and spawning. Appearance lives in the frontend.

pub mod components;
pub mod entities;
pub mod resources;
pub mod types;

use bevy::prelude::*;

use self::resources::monster_registry::MonsterRegistry;
use crate::core::app_state::AppState;

/// Register the monster domain: the vocabulary registry builds at app
/// build time (`FromWorld`), and monsters spawn on entering the game.
pub fn register(app: &mut App) {
    app.init_resource::<MonsterRegistry>()
        .add_systems(OnEnter(AppState::Game), entities::monsters::spawn_monsters);
}
