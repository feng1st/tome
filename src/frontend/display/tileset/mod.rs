//! The tileset domain: grid texture atlases as a general display
//! mechanism — definition files, texture loading, and the registry.
//! Consumers (map chunks, and later others) look textures up here.

pub mod resources;
pub mod types;

use bevy::prelude::*;

use crate::frontend::display::tileset::resources::tileset_registry::TilesetRegistry;

/// Register the tileset domain: the registry builds at app build time
/// (`FromWorld` — data errors panic before the window opens).
pub fn register(app: &mut App) {
    app.init_resource::<TilesetRegistry>();
}
