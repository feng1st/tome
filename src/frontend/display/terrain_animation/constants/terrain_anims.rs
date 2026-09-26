//! Animated terrain definitions: which tile kinds get a scrolling layer
//! and how it moves. Display data layered on top of the core's tile kinds.

use bevy::prelude::*;

use crate::core::map::constants::tile_kind::TileKind;
use crate::frontend::display::terrain_animation::types::terrain_anim_spec::TerrainAnimSpec;

/// The animated terrains.
pub const TERRAIN_ANIMS: &[TerrainAnimSpec] = &[TerrainAnimSpec {
    kind: TileKind::Water,
    texture: "water0.png",
    texture_size: UVec2::splat(32),
    velocity: Vec2::new(0.0, -5.0 / 32.0),
}];
