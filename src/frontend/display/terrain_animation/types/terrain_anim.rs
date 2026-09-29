//! The terrain animation value type.

use bevy::prelude::*;

/// One terrain animation: a texture layer and how it flows. Static
/// definition data; the runtime instance carries a `TerrainAnimState`
/// component. The kind binding lives at the use site (today:
/// `WATER_ANIM` in `constants/terrain_anims`).
///
/// Rendering: the layer is a map-sized quad beneath the canvas sprite
/// in the screen pass, world-locked by the follow rig (position =
/// anchor − target). The chunks draw this terrain's cells as
/// transparent or shoreline variants, so the layer shows exactly
/// through the canvas's holes for these cells; the flow rides
/// `ColorMaterial`'s `uv_transform` (see `systems/animate.rs`).
///
/// Known limit: one map-sized quad per kind works only while animated
/// kinds don't coexist on one map — stacked quads can't discriminate
/// the holes per kind. Coexisting kinds (a water pool and a lava pool)
/// need per-cell union meshes instead; deferred until a second
/// animated kind lands.
pub struct TerrainAnim {
    pub texture: &'static str,
    /// Texture size in pixels; the quad's baked UVs repeat the texture
    /// every `texture_size` screen units.
    pub texture_size: UVec2,
    /// Scroll velocity (screen units per second; 5 units = 10 window
    /// pixels at the fixed upscale).
    pub velocity: Vec2,
}
