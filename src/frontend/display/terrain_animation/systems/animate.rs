//! Pure playback (`DisplayPhase::Animate`): every animated terrain
//! layer's UV offset advances with elapsed time, written into the
//! material's `uv_transform` — the engine's own color-material shader
//! applies it. Stateless: the offset is a pure function of global
//! time, same philosophy as `sprite_animation`. The layer's position is
//! fixed at spawn (map-anchored world content); this system only adds
//! the flow.

use bevy::math::Affine2;
use bevy::prelude::*;

use crate::frontend::display::terrain_animation::components::terrain_anim_state::TerrainAnimState;

/// A layer scrolls at its fixed velocity: the translation is the
/// velocity integrated over elapsed virtual time, in repeat units
/// (REPEAT addressing wraps it). The repeat scale is baked into the
/// mesh UVs at spawn.
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
        material.uv_transform = Affine2::from_translation(anim.velocity * t);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn uv_offset_advances_with_virtual_time() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .init_resource::<Assets<ColorMaterial>>()
            .add_systems(Update, animate);
        let material = app
            .world_mut()
            .resource_mut::<Assets<ColorMaterial>>()
            .add(ColorMaterial::default());
        app.world_mut().spawn((
            MeshMaterial2d(material.clone()),
            TerrainAnimState {
                velocity: Vec2::new(0.0, -0.5),
            },
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
        // The translation is the velocity integrated over elapsed
        // virtual time.
        assert_eq!(uv.translation, Vec2::new(0.0, -1.0));
    }
}
