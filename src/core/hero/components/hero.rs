// TODO: pending cleanup review — remove once stabilized
//! The hero marker component.

use bevy::prelude::*;

/// Marker for the player character. Input handling and spawn logic find the
/// hero through it.
#[derive(Component)]
pub struct Hero;
