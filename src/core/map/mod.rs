//! The map domain's game-data side: local map model, terrain registry,
//! pathfinding. Rendering (chunks, textures) lives in `frontend::display`.

pub mod components;
pub mod constants;
pub mod resources;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use self::resources::current_map::CurrentMap;
use self::resources::terrain_registry::TerrainRegistry;

/// Register the map domain: the terrain registry and the current map
/// build at app build time (`FromWorld` — the map pulls the registry
/// into existence, so registration order never matters; data errors
/// panic before the window opens). Rendering the map (chunks, textures)
/// is the frontend's job.
pub fn register(app: &mut App) {
    app.init_resource::<TerrainRegistry>()
        .init_resource::<CurrentMap>();
}
