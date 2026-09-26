//! Animated terrain definitions: which tile kinds get a scrolling layer
//! and how it moves. Display data layered on top of the core's tile kinds.

use bevy::prelude::*;

use crate::core::map::constants::tile_kind::TileKind;
use crate::frontend::display::terrain_animation::types::terrain_anim_spec::TerrainAnimSpec;

/// The animated terrains. Adding lava, poison gas, etc. means adding an
/// entry here plus its texture — no structural change.
pub const TERRAIN_ANIMS: &[TerrainAnimSpec] = &[TerrainAnimSpec {
    kind: TileKind::Water,
    texture: "water0.png",
    texture_size: 32,
    velocity: Vec2::new(0.0, -5.0 / 32.0),
}];
