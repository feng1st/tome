//! The hero domain's display side: sprite assets, frame tables, and
//! attaching appearance to heroes spawned by the core.

pub mod constants;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

/// Register the hero display domain.
pub fn register(app: &mut App) {
    systems::register(app);
}
