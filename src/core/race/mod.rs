pub mod components;
pub mod resources;
pub mod types;

use bevy::prelude::*;

use self::resources::race_registry::RaceRegistry;

pub fn register(app: &mut App) {
    app.init_resource::<RaceRegistry>();
}
