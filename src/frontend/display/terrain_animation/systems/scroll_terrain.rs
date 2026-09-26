// TODO: pending cleanup review — remove once stabilized
//! Drives every scrolling terrain layer: the UV offset advances with
//! elapsed time, written into the material's `uv_transform` — the engine's
//! own color-material shader applies it, no custom shader needed.

use bevy::math::Affine2;
use bevy::prelude::*;

use crate::frontend::display::terrain_animation::components::terrain_scroll::TerrainScroll;

/// PD parity: `GameScene.update()` scrolls its water block by a fixed
/// pixel velocity each frame; writing the `uv_transform` translation does
/// the same thing on the material uniform.
pub fn scroll_terrain(
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&TerrainScroll, &MeshMaterial2d<ColorMaterial>)>,
) {
    let t = time.elapsed_secs();
    for (scroll, material) in &query {
        let Some(mut material) = materials.get_mut(material.id()) else {
            continue;
        };
        // Scale first (texture repeats across the quad), then translate
        // (scroll) — the translation is in texture-uv units either way.
        material.uv_transform =
            Affine2::from_mat2_translation(Mat2::from_diagonal(scroll.scale), scroll.scroll * t);
    }
}
