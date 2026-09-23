//! Copies the core's authoritative `Position` into the render `Transform`.

use bevy::prelude::*;

use crate::core::movement::components::position::Position;
use crate::frontend::graphic::map::utils::coords::cell_to_world;

/// Write the authoritative position into the transform every frame. The
/// transform's x/y are derived display state, never a source of truth; z is
/// presentation layering owned by the display side and preserved here. No
/// pixel snapping: continuous pixel motion matches the original game's feel.
pub fn sync_position(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        let world = cell_to_world(pos.0);
        transform.translation.x = world.x;
        transform.translation.y = world.y;
    }
}
