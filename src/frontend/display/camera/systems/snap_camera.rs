//! Snaps the camera onto the screen grid.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::frontend::display::camera::components::camera_target::CameraTarget;
use crate::frontend::display::camera::components::main_camera::MainCamera;
use crate::frontend::display::camera::utils::screen_grid::world_scale_factor;
use crate::frontend::display::constants::layout::ZOOM;

/// The camera follows its target in screen-pixel steps and renders at
/// the effective integer magnification. The projection scale is the
/// window's scale factor over the (integer) world-to-screen factor, so
/// one world pixel always covers an integer number of screen pixels;
/// the camera position is the target's position rounded to the screen
/// grid, so the presented view scrolls at a quantum of one screen pixel
/// and the target sits within half a screen pixel of the view center.
/// Both derive live from the window's scale factor. Integer-gridded
/// content (terrain chunks, anchored layers) shares the lattice and
/// needs no further snapping.
pub fn snap_camera(
    target: Query<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    let (Ok(target_transform), Ok((mut camera_transform, mut projection))) =
        (target.single(), camera.single_mut())
    else {
        return;
    };
    let world_scale_factor = windows.single().map(world_scale_factor).unwrap_or(ZOOM);
    let window_scale_factor = windows.single().map(Window::scale_factor).unwrap_or(1.0);
    let Projection::Orthographic(orthographic) = projection.as_mut() else {
        return;
    };
    orthographic.scale = window_scale_factor / world_scale_factor;
    let target_position = target_transform.translation;
    camera_transform.translation.x =
        (target_position.x * world_scale_factor).round() / world_scale_factor;
    camera_transform.translation.y =
        (target_position.y * world_scale_factor).round() / world_scale_factor;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        let mut app = App::new();
        app.add_systems(Update, snap_camera);
        app
    }

    fn rig(app: &mut App, target_xy: Vec2) -> (Entity, Entity) {
        let target = app
            .world_mut()
            .spawn((
                Transform::from_xyz(target_xy.x, target_xy.y, 0.0),
                CameraTarget,
            ))
            .id();
        let camera = app
            .world_mut()
            .spawn((
                Transform::default(),
                Projection::Orthographic(OrthographicProjection::default_2d()),
                MainCamera,
            ))
            .id();
        (target, camera)
    }

    #[test]
    fn camera_snaps_to_the_screen_grid() {
        let mut app = app();
        // No window in a bare world: the factor falls back to `ZOOM`
        // (as if the window scale factor were 1.0).
        let (_, camera) = rig(&mut app, Vec2::new(100.3, -50.27));
        app.update();
        let position = app
            .world()
            .get::<Transform>(camera)
            .unwrap()
            .translation
            .truncate();
        let world_scale_factor = ZOOM;
        assert_eq!(
            position * world_scale_factor,
            (position * world_scale_factor).round()
        );
        // The snapped position stays within half a screen pixel.
        let error = position - Vec2::new(100.3, -50.27);
        assert!(error.abs().max_element() <= 0.5 / world_scale_factor + 1e-4);
    }

    #[test]
    fn integer_target_needs_no_correction() {
        let mut app = app();
        let (_, camera) = rig(&mut app, Vec2::new(96.0, -64.0));
        app.update();
        let position = app
            .world()
            .get::<Transform>(camera)
            .unwrap()
            .translation
            .truncate();
        assert_eq!(position, Vec2::new(96.0, -64.0));
    }

    #[test]
    fn window_scale_factor_drives_the_grid() {
        let mut app = app();
        // A window reporting 150% display scaling: the grid is one
        // third of a world pixel, and the projection stays at the
        // nominal zoom (1.5 / 3 = 0.5).
        let mut window = Window::default();
        window.resolution.set_scale_factor_override(Some(1.5));
        app.world_mut().spawn((window, PrimaryWindow));
        let (_, camera) = rig(&mut app, Vec2::new(100.3, -50.27));
        app.update();
        let position = app
            .world()
            .get::<Transform>(camera)
            .unwrap()
            .translation
            .truncate();
        let world_scale_factor = ZOOM * 1.5;
        assert_eq!(
            position * world_scale_factor,
            (position * world_scale_factor).round()
        );
        let Projection::Orthographic(orthographic) = app.world().get::<Projection>(camera).unwrap()
        else {
            panic!("2d projection is orthographic");
        };
        assert_eq!(orthographic.scale, 0.5);
    }

    #[test]
    fn fractional_display_scale_rounds_to_integer_magnification() {
        let mut app = app();
        // A window reporting 175% display scaling: the effective
        // magnification rounds to the integer 4, so the grid is a
        // quarter world pixel and the projection deviates from the
        // nominal zoom to keep every world pixel on an integer number
        // of screen pixels.
        let mut window = Window::default();
        window.resolution.set_scale_factor_override(Some(1.75));
        app.world_mut().spawn((window, PrimaryWindow));
        let (_, camera) = rig(&mut app, Vec2::new(100.3, -50.27));
        app.update();
        let position = app
            .world()
            .get::<Transform>(camera)
            .unwrap()
            .translation
            .truncate();
        assert_eq!(position * 4.0, (position * 4.0).round());
        let Projection::Orthographic(orthographic) = app.world().get::<Projection>(camera).unwrap()
        else {
            panic!("2d projection is orthographic");
        };
        assert!((orthographic.scale - 1.75 / 4.0).abs() < 1e-6);
    }
}
