//! Animated terrain definitions. Display data layered on top of the
//! core's tile kinds.

use bevy::prelude::*;

use crate::frontend::display::terrain_animation::types::terrain_anim::TerrainAnim;

/// The water terrain animation — the only animated kind for now (the
/// kind binding lives in the constant name; multi-kind support returns
/// as a table plus a map-built presence mask).
pub const WATER_ANIM: TerrainAnim = TerrainAnim {
    texture: "water0.png",
    texture_size: UVec2::splat(32),
    velocity: Vec2::new(0.0, -5.0 / 32.0),
};
