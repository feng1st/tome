pub mod components;
pub mod constants;
pub mod resources;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use self::resources::current_map::CurrentMap;
use self::resources::terrain_registry::TerrainRegistry;

pub fn register(app: &mut App) {
    app.init_resource::<TerrainRegistry>()
        .init_resource::<CurrentMap>();
}
