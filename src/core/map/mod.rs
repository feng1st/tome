// TODO: pending cleanup review — remove once stabilized
//! The map domain's game-data side: grid model, terrain kinds, pathfinding.
//! Rendering (chunks, textures) lives in `frontend::display`.

pub mod constants;
pub mod resources;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use resources::current_map::CurrentMap;
use types::grid_map::GridMap;

/// Register the map domain: the current map is game data. Rendering the
/// map (chunks, textures) is the frontend's job.
pub fn register(app: &mut App) {
    app.insert_resource(CurrentMap::new(GridMap::demo_room()));
}
