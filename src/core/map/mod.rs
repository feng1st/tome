//! The map domain's game-data side: grid model, terrain kinds, pathfinding.
//! Rendering (chunks, textures) lives in `graphic::map`.

pub mod constants;
pub mod resources;
pub mod utils;

use bevy::prelude::*;

use resources::grid_map::GridMap;

/// Register the map domain: the map resource is game data. Rendering the
/// map (chunks, textures) is the graphic plugin's job.
pub fn register(app: &mut App) {
    app.insert_resource(GridMap::demo_room());
}
