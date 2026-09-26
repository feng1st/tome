//! Pure playback (`DisplayPhase::Animate`): every animated terrain
//! layer's UV offset advances with elapsed time, written into the
//! material's `uv_transform` — the engine's own color-material shader
//! applies it, no custom shader needed. Stateless: the offset is a pure
//! function of global time, same philosophy as `sprite_animation`.

use bevy::math::Affine2;
use bevy::prelude::*;

use crate::frontend::display::terrain_animation::components::terrain_anim_state::TerrainAnimState;

/// The layer scrolls at a fixed velocity: writing the `uv_transform`
/// translation advances the offset on the material uniform.
pub fn animate(
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&TerrainAnimState, &MeshMaterial2d<ColorMaterial>)>,
) {
    let t = time.elapsed_secs();
    for (anim, material) in &query {
        let Some(mut material) = materials.get_mut(material.id()) else {
            continue;
        };
        // Scale first (texture repeats across the quad), then translate
        // (scroll) — the translation is in texture-uv units either way.
        material.uv_transform =
            Affine2::from_mat2_translation(Mat2::from_diagonal(anim.scale), anim.velocity * t);
    }
}
