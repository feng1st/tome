//! Component driving one animated terrain layer.

use bevy::prelude::*;

/// Per-instance state of one animated terrain layer: the scroll
/// velocity in repeat units per second that `systems/animate.rs`
/// integrates into the UV offset. The layer's transform is fixed at
/// spawn (map-anchored world content); playback never touches it.
#[derive(Component)]
pub struct TerrainAnimState {
    pub velocity: Vec2,
}
