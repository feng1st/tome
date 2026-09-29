//! Keeps the camera rig on the pixel grid while presenting smoothly:
//! the canvas camera snaps to integer pixels, the presented canvas pans
//! by the sub-pixel remainder, and the terrain layers track the world.

use bevy::prelude::*;

use crate::frontend::display::camera::components::camera_target::CameraTarget;
use crate::frontend::display::camera::components::canvas_camera::CanvasCamera;
use crate::frontend::display::canvas::components::canvas::Canvas;
use crate::frontend::display::terrain_animation::components::terrain_anim_state::TerrainAnimState;

type TargetQuery<'w, 's> = Query<'w, 's, &'static Transform, With<CameraTarget>>;
type CameraQuery<'w, 's> = Query<'w, 's, &'static mut Transform, With<CanvasCamera>>;
type CanvasQuery<'w, 's> = Query<'w, 's, &'static mut Transform, With<Canvas>>;
type TerrainAnimQuery<'w, 's> = Query<'w, 's, (&'static TerrainAnimState, &'static mut Transform)>;

/// Snap the canvas camera to integer pixels, pan the canvas by the
/// sub-pixel remainder, and re-base the animated terrain layers on the
/// target — one computation driving the follow policy. The canvas
/// content stays grid-aligned (stable texel pattern, whole-texel
/// steps); the pan cancels the snap on screen, so the target sits
/// exactly centered and motion scrolls at screen-pixel granularity.
/// The margin covers the remainder; these transforms are derived
/// display state.
///
/// A terrain layer shows through the canvas's holes, so it must track
/// the world exactly: anchored at the map center, its position is
/// `anchor − target`, the same law a fixed world point follows.
pub fn follow_target(
    mut queries: ParamSet<(TargetQuery, CameraQuery, CanvasQuery, TerrainAnimQuery)>,
) {
    let target_position = {
        let target_query = queries.p0();
        let Ok(target_transform) = target_query.single() else {
            return;
        };
        target_transform.translation.truncate()
    };
    let snapped = target_position.round();
    let remainder = snapped - target_position;
    {
        let mut camera_query = queries.p1();
        let Ok(mut camera_transform) = camera_query.single_mut() else {
            return;
        };
        camera_transform.translation.x = snapped.x;
        camera_transform.translation.y = snapped.y;
    }
    {
        let mut canvas_query = queries.p2();
        let Ok(mut canvas_transform) = canvas_query.single_mut() else {
            return;
        };
        canvas_transform.translation.x = remainder.x;
        canvas_transform.translation.y = remainder.y;
    }
    {
        let mut terrain_anim_query = queries.p3();
        for (terrain_anim, mut terrain_anim_transform) in &mut terrain_anim_query {
            terrain_anim_transform.translation.x = terrain_anim.world_anchor.x - target_position.x;
            terrain_anim_transform.translation.y = terrain_anim.world_anchor.y - target_position.y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn follow_after(target_xy: Vec2) -> (Vec2, Vec2, Vec2) {
        let mut app = App::new();
        app.add_systems(Update, follow_target);
        app.world_mut().spawn((
            Transform::from_xyz(target_xy.x, target_xy.y, 0.0),
            CameraTarget,
        ));
        let camera = app
            .world_mut()
            .spawn((Transform::default(), CanvasCamera))
            .id();
        let canvas = app.world_mut().spawn((Transform::default(), Canvas)).id();
        let terrain_anim_anchor = Vec2::new(384.0, -256.0);
        let terrain_anim = app
            .world_mut()
            .spawn((
                Transform::default(),
                TerrainAnimState {
                    world_anchor: terrain_anim_anchor,
                    velocity: Vec2::ZERO,
                },
            ))
            .id();
        app.update();
        let camera_position = app
            .world()
            .get::<Transform>(camera)
            .unwrap()
            .translation
            .truncate();
        let canvas_offset = app
            .world()
            .get::<Transform>(canvas)
            .unwrap()
            .translation
            .truncate();
        let terrain_anim_position = app
            .world()
            .get::<Transform>(terrain_anim)
            .unwrap()
            .translation
            .truncate();
        (camera_position, canvas_offset, terrain_anim_position)
    }

    #[test]
    fn grid_target_needs_no_canvas_pan() {
        let (camera_position, canvas_offset, terrain_anim_position) =
            follow_after(Vec2::new(96.0, -64.0));
        assert_eq!(camera_position, Vec2::new(96.0, -64.0));
        assert_eq!(canvas_offset, Vec2::ZERO);
        // The layer tracks the world: anchor − target.
        assert_eq!(
            terrain_anim_position,
            Vec2::new(384.0 - 96.0, -256.0 + 64.0)
        );
    }

    #[test]
    fn off_grid_target_snaps_camera_and_pans_canvas_by_the_remainder() {
        let (camera_position, canvas_offset, terrain_anim_position) =
            follow_after(Vec2::new(100.4, -50.6));
        assert_eq!(camera_position, Vec2::new(100.0, -51.0));
        assert!((canvas_offset - Vec2::new(-0.4, -0.4)).length() < 1e-4);
        assert!((terrain_anim_position - Vec2::new(384.0 - 100.4, -256.0 + 50.6)).length() < 1e-4);
        // Half-way ties round away from zero, in both signs.
        let (camera_position, _, _) = follow_after(Vec2::new(100.5, -100.5));
        assert_eq!(camera_position, Vec2::new(101.0, -101.0));
    }
}
