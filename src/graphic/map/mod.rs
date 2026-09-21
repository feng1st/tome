//! The map domain's display side: tileset textures, chunk rendering,
//! animated terrain overlays, and cell/pixel conversion.

pub mod components;
pub mod constants;
pub mod entities;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use resources::pending_chunks::PendingChunks;

/// Register map rendering: two-phase texture loading and terrain animation.
/// Reads the core's `GridMap` resource; owns all pixel concepts.
pub fn register(app: &mut App) {
    app.add_systems(Startup, systems::loading::begin_load)
        .add_systems(Update, systems::terrain_anim::animate_terrain)
        .add_systems(
            Update,
            systems::loading::finish_chunks.run_if(resource_exists::<PendingChunks>),
        );
}
