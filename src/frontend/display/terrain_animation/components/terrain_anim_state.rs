//! Component driving one animated terrain layer.

use bevy::prelude::*;

/// Per-instance state of one animated terrain layer: the world anchor
/// the follow rig locks on (the map's center in world pixels) and the
/// scroll velocity in repeat units per second that
/// `systems/animate.rs` integrates into the UV offset.
#[derive(Component)]
pub struct TerrainAnimState {
    pub world_anchor: Vec2,
    pub velocity: Vec2,
}
