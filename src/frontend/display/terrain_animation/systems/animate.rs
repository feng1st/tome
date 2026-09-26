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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::math::Mat2;

    use super::*;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .init_resource::<Assets<ColorMaterial>>()
            .add_systems(Update, animate);
        app
    }

    #[test]
    fn uv_offset_advances_with_time() {
        let mut app = app();
        let material = app
            .world_mut()
            .resource_mut::<Assets<ColorMaterial>>()
            .add(ColorMaterial::default());
        app.world_mut().spawn((
            TerrainAnimState {
                scale: Vec2::new(2.0, 4.0),
                velocity: Vec2::new(0.0, -0.5),
            },
            MeshMaterial2d(material.clone()),
        ));
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs(2));
        app.update();
        let uv = app
            .world()
            .resource::<Assets<ColorMaterial>>()
            .get(&material)
            .expect("material registered")
            .uv_transform;
        // Scale repeats the texture across the quad; the translation is the
        // scroll velocity integrated over elapsed virtual time.
        assert_eq!(uv.matrix2, Mat2::from_diagonal(Vec2::new(2.0, 4.0)));
        assert_eq!(uv.translation, Vec2::new(0.0, -1.0));
    }
}
