//! Animated terrain definitions: which tile kinds get an animated overlay
//! and how it plays. Display data layered on top of the core's tile kinds.

use crate::core::map::constants::tile_kind::TileKind;
use crate::frontend::graphic::map::constants::layout::LAYER_WATER;

/// One animated terrain type: which tile kind it renders, where its texture
/// comes from, and how it animates. Adding lava, poison gas, etc. means
/// adding an entry here plus its texture — no structural change.
///
/// The animation is a flipbook of offset windows into the source texture,
/// plus a global alpha pulse (see `systems/terrain_anim.rs`).
pub struct TerrainAnimSpec {
    pub kind: TileKind,
    pub texture: &'static str,
    /// Horizontal sample offsets (in px) into the source texture, one per frame.
    pub offsets: &'static [u32],
    pub fps: f32,
    pub alpha_min: f32,
    pub alpha_max: f32,
    pub layer: f32,
}

pub const TERRAIN_ANIMS: &[TerrainAnimSpec] = &[TerrainAnimSpec {
    kind: TileKind::Water,
    texture: "water0.png",
    offsets: &[0, 8, 16, 24],
    fps: 4.0,
    alpha_min: 0.8,
    alpha_max: 1.0,
    layer: LAYER_WATER,
}];
