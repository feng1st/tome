//! Keeps the gameplay camera centered on its target.

use bevy::prelude::*;

use crate::graphic::camera::components::camera_target::CameraTarget;
use crate::graphic::camera::components::main_camera::MainCamera;

/// The followed entity is always centered; the void beyond the map shows as
/// background, matching the original game.
pub fn follow_target(
    target: Query<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let (Ok(target_transform), Ok(mut camera_transform)) = (target.single(), camera.single_mut())
    else {
        return;
    };
    camera_transform.translation.x = target_transform.translation.x;
    camera_transform.translation.y = target_transform.translation.y;
}
