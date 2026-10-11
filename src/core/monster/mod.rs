pub mod components;
pub mod entities;
pub mod resources;
pub mod types;

use bevy::prelude::*;

use self::resources::monster_registry::MonsterRegistry;
use crate::core::app_state::AppState;

pub fn register(app: &mut App) {
    app.init_resource::<MonsterRegistry>()
        .add_systems(OnEnter(AppState::Game), entities::monsters::spawn_monsters);
}
