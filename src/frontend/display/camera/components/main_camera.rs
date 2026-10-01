//! Marker for the main camera: the single camera rendering the world
//! straight to the window.

use bevy::prelude::*;

/// Marks the camera that renders the world to the window. Click
/// handling resolves through it to convert window coordinates to world
/// coordinates.
#[derive(Component)]
pub struct MainCamera;
