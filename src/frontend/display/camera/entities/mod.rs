//! Camera spawning.

pub mod camera;

use bevy::prelude::*;

/// Register camera spawning: the camera is display infrastructure, spawned
/// in `Startup`.
pub fn register(app: &mut App) {
    app.add_systems(Startup, camera::spawn_camera);
}
