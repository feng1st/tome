use bevy::prelude::*;

/// Marker for the one gameplay camera; click handling resolves it to
/// convert screen coordinates to world coordinates.
#[derive(Component)]
pub struct MainCamera;
