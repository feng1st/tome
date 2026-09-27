//! The map domain's display side: static tilemap rendering — tileset
//! chunks, layout constants, and cell/pixel conversion. Time-driven
//! terrain visuals live in `terrain_animation`.

pub mod constants;
pub mod entities;
pub mod resources;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use self::resources::terrain_tile_registry::TerrainTileRegistry;
use crate::core::app_state::AppState;

/// Register the map display domain: the terrain tile registry builds at
/// app build time (`FromWorld`, pulling the core's terrain registry into
/// existence — data errors panic before the window opens). Chunks spawn
/// on entering `Game` (the same path a later map switch takes).
pub fn register(app: &mut App) {
    app.init_resource::<TerrainTileRegistry>()
        .add_systems(OnEnter(AppState::Game), entities::chunks::spawn_chunks);
}
