//! The terrain animation value type.

use bevy::prelude::*;

/// One terrain animation: a scrolling texture layer and how it moves.
/// Static definition data; the runtime instance carries a
/// `TerrainAnimState` component. The kind binding lives at the use site
/// (today: `WATER_ANIM` in `constants/terrain_anims`).
///
/// Rendering: the chunks draw this terrain's cells as shoreline variants
/// (open water fully transparent), and a map-sized REPEAT quad beneath
/// the chunks scrolls this texture, showing through the transparent
/// parts. The scroll is written into `ColorMaterial`'s `uv_transform`
/// (see `systems/animate.rs`).
///
/// Known limit: one map-sized quad per kind works only while animated
/// kinds don't coexist
/// on one map — overlapping quads at the same z can't discriminate
/// per-cell. Coexisting kinds (a water pool and a lava pool) need
/// per-cell meshes with world-space UVs instead; deferred until a second
/// animated kind lands.
pub struct TerrainAnim {
    pub texture: &'static str,
    /// Texture size in pixels; the quad's UV scale repeats the texture
    /// every `texture_size` world pixels.
    pub texture_size: UVec2,
    /// UV scroll velocity (uv units per second; -5/32 uv/s is -5 px/s on
    /// a 32px texture).
    pub velocity: Vec2,
}
