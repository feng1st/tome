pub mod components;
pub mod constants;
pub mod entities;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use resources::grid_map::GridMap;
use resources::pending_chunks::PendingChunks;
use systems::loading::{begin_load, finish_chunks};

/// Register map-domain systems. The cross-domain movement chain is assembled
/// in main.rs.
pub fn register(app: &mut App) {
    app.insert_resource(GridMap::demo_room())
        .add_systems(Startup, begin_load)
        .add_systems(Update, systems::terrain_anim::animate_terrain)
        .add_systems(
            Update,
            finish_chunks.run_if(resource_exists::<PendingChunks>),
        );
}
