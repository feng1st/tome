//! Spawns the one gameplay camera (2x zoom).

use bevy::prelude::*;

use crate::frontend::graphic::camera::components::main_camera::MainCamera;

/// Startup system: 2x zoom (view = 640x360 world units).
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
    ));
}
