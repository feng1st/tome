//! Component driving one animated terrain layer quad.

use bevy::prelude::*;

/// Animation state of one animated terrain layer. Drives the material's
/// `uv_transform` in `systems/animate.rs`.
#[derive(Component)]
pub struct TerrainAnim {
    /// UV repetitions across the quad (map pixels / texture pixels).
    pub scale: Vec2,
    /// UV scroll velocity (uv units per second).
    pub velocity: Vec2,
}
