use bevy::prelude::*;

/// Attached to the entity the camera follows (the hero today; anything later).
#[derive(Component)]
pub struct CameraTarget;
