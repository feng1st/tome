//! Component marking a scrolling terrain layer quad.

use bevy::prelude::*;

/// Scroll state of one scrolling terrain layer. Drives the material's
/// `uv_transform` in `systems/scroll_terrain.rs`.
#[derive(Component)]
pub struct TerrainScroll {
    /// UV repetitions across the quad (map pixels / texture pixels).
    pub scale: Vec2,
    /// UV scroll velocity (uv units per second).
    pub scroll: Vec2,
}
