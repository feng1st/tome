//! Spawns the main camera: the single direct-to-window camera.

use bevy::prelude::*;

use crate::frontend::display::camera::components::main_camera::MainCamera;
use crate::frontend::display::constants::layout::ZOOM;

/// Startup system: spawn the camera that renders the world straight to
/// the window. The initial projection scale is the nominal `1 / ZOOM`;
/// `snap_camera` re-derives it every frame as the window's scale factor
/// over the effective integer magnification, so one world pixel always
/// covers an integer number of screen pixels. The view tracks the
/// window (a resized window shows more or less of the world). MSAA
/// stays off: multisample resolve blends shape edges with the
/// background at fractional phases, which shows as stray lines at
/// sprite and tile borders. Clear color stays the global default: the
/// void beyond the map shows it.
pub fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0 / ZOOM,
            ..OrthographicProjection::default_2d()
        }),
        Msaa::Off,
        MainCamera,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_camera_spawns_with_msaa_off() {
        let mut app = App::new();
        app.add_systems(Startup, spawn_main_camera);
        app.update();
        let mut cameras = app
            .world_mut()
            .query_filtered::<(&Projection, &Msaa), With<MainCamera>>();
        let (projection, msaa) = cameras.single(app.world()).expect("one main camera");
        let Projection::Orthographic(orthographic) = projection else {
            panic!("2d projection is orthographic");
        };
        assert_eq!(orthographic.scale, 1.0 / ZOOM);
        assert!(matches!(msaa, Msaa::Off));
    }
}
