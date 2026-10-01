//! The terrain animation value type.

use bevy::prelude::*;

/// One terrain animation: a texture layer and how it flows. Static
/// definition data; the runtime instance carries a `TerrainAnimState`
/// component. The kind binding lives at the use site (today:
/// `WATER_ANIM` in `constants/terrain_anims`).
///
/// Rendering: the layer is a map-sized rectangle in the world, anchored at
/// the map center beneath the terrain mesh (z below the ground plane).
/// The chunks draw this terrain's cells as transparent or shoreline
/// variants, so the layer shows exactly through the mesh's holes for
/// these cells; the flow rides `ColorMaterial`'s `uv_transform` (see
/// `systems/animate.rs`).
///
/// Known limit: one map-sized layer per kind works only while animated
/// kinds don't coexist on one map — stacked layers can't discriminate
/// the holes per kind. Coexisting kinds (a water pool and a lava pool)
/// need per-cell union meshes instead; deferred until a second
/// animated kind lands.
pub struct TerrainAnim {
    pub texture: &'static str,
    /// Texture size in pixels; the layer's baked UVs repeat the texture
    /// every `texture_size` world pixels.
    pub texture_size: UVec2,
    /// Scroll velocity (world pixels per second; one world pixel
    /// presents as `ZOOM` window pixels).
    pub velocity: Vec2,
}
