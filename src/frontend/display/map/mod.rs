//! The map domain's display side: tileset textures, chunk rendering,
//! animated terrain overlays, and cell/pixel conversion.

pub mod components;
pub mod constants;
pub mod entities;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

/// Register the map display domain.
pub fn register(app: &mut App) {
    systems::register(app);
}
