//! The hero marker component.

use bevy::prelude::*;

/// Marker for the player character. Spawning, command execution, and the
/// camera attach find the hero through it.
#[derive(Component)]
pub struct Hero;
