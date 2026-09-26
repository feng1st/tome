//! Keeps the gameplay camera centered on its target.

use bevy::prelude::*;

use crate::frontend::display::camera::components::camera_target::CameraTarget;
use crate::frontend::display::camera::components::main_camera::MainCamera;

/// The followed entity is always centered; the void beyond the map shows
/// as background.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_centers_on_its_target() {
        let mut app = App::new();
        app.add_systems(Update, follow_target);
        app.world_mut()
            .spawn((Transform::from_xyz(100.0, -50.0, 0.0), CameraTarget));
        let camera = app
            .world_mut()
            .spawn((Transform::default(), MainCamera))
            .id();
        app.update();
        let transform = app.world().get::<Transform>(camera).unwrap();
        assert_eq!(transform.translation.truncate(), Vec2::new(100.0, -50.0));
    }
}
