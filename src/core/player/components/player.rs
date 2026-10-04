//! The player marker component.

use bevy::prelude::*;

/// Marker for the player character. Spawning, command execution, and the
/// camera attach find the player through it.
#[derive(Component)]
pub struct Player;
