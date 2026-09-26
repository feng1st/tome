//! The terrain animation value type.

use bevy::prelude::*;

/// One terrain animation: a scrolling texture layer and how it moves.
/// Static definition data; the runtime instance carries a
/// `TerrainAnimState` component. The kind binding lives at the use site
/// (today: `WATER_ANIM` in `constants/terrain_anims`).
///
/// Rendering follows PD's water (`GameScene.java` + `SkinnedBlock.java`):
/// the chunks draw this terrain's cells as shoreline variants (open water
/// fully transparent), and a map-sized REPEAT quad beneath the chunks
/// scrolls this texture, showing through the transparent parts. PD scrolls
/// by offsetting UVs on the CPU each frame (`water.offset(0, -5 *
/// elapsed)`); we write `ColorMaterial`'s `uv_transform` instead (see
/// `systems/animate.rs`).
///
/// Known limit (inherited from PD, which animates only water): one
/// map-sized quad per kind works only while animated kinds don't coexist
/// on one map — overlapping quads at the same z can't discriminate
/// per-cell. Coexisting kinds (a water pool and a lava pool) need
/// per-cell meshes with world-space UVs instead; deferred until a second
/// animated kind lands.
pub struct TerrainAnim {
    pub texture: &'static str,
    /// Texture size in pixels; the quad's UV scale repeats the texture
    /// every `texture_size` world pixels (PD `SkinnedBlock`:
    /// `u1 = u0 + width / texture.width`).
    pub texture_size: UVec2,
    /// UV scroll velocity (uv units per second). PD scrolls its 32px water
    /// texture at -5 px/s, i.e. -5/32 uv/s.
    pub velocity: Vec2,
}
