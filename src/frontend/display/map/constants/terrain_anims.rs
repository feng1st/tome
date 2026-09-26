//! Animated terrain definitions: which tile kinds get a scrolling layer
//! and how it moves. Display data layered on top of the core's tile kinds.

use bevy::prelude::*;

use crate::core::map::constants::tile_kind::TileKind;

/// One animated terrain type: which tile kind it animates and how its
/// scrolling texture layer moves. Adding lava, poison gas, etc. means
/// adding an entry here plus its texture — no structural change.
///
/// Rendering follows PD's water (`GameScene.java` + `SkinnedBlock.java`):
/// the chunks draw this terrain's cells as shoreline variants (open water
/// fully transparent), and a map-sized REPEAT quad beneath the chunks
/// scrolls this texture, showing through the transparent parts. PD scrolls by offsetting UVs on the CPU each frame
/// (`water.offset(0, -5 * elapsed)`); we write `ColorMaterial`'s
/// `uv_transform` instead (see `systems/scroll_terrain.rs`).
pub struct TerrainAnimSpec {
    /// The terrain this layer animates; the quad spawns only when the map
    /// actually contains this kind.
    pub kind: TileKind,
    pub texture: &'static str,
    /// Square texture size in pixels; the quad's UV scale repeats the
    /// texture every `texture_size` world pixels (PD `SkinnedBlock`:
    /// `u1 = u0 + width / texture.width`).
    pub texture_size: u32,
    /// UV scroll velocity (uv units per second). PD scrolls its 32px water
    /// texture at -5 px/s, i.e. -5/32 uv/s.
    pub scroll: Vec2,
}

pub const TERRAIN_ANIMS: &[TerrainAnimSpec] = &[TerrainAnimSpec {
    kind: TileKind::Water,
    texture: "water0.png",
    texture_size: 32,
    scroll: Vec2::new(0.0, -5.0 / 32.0),
}];
