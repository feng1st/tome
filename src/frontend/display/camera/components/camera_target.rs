// TODO: pending cleanup review — remove once stabilized
//! Marker for the entity the camera follows.

use bevy::prelude::*;

/// Attached to the entity the camera follows (the hero today; anything
/// later). Camera code depends on this marker, not on the hero domain.
#[derive(Component)]
pub struct CameraTarget;
