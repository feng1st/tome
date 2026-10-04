//! Marker for the entity the camera follows.

use bevy::prelude::*;

/// Attached to the entity the camera follows (the player today; anything
/// later). Camera code depends on this marker, not on the player domain.
#[derive(Component)]
pub struct CameraTarget;
