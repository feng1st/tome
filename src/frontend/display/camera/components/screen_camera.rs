//! Marker for the screen camera: it presents the canvas to the window.

use bevy::prelude::*;

/// Marks the camera that renders the canvas sprite to the window. The
/// canvas camera renders the world into the canvas; this camera only
/// ever sees the screen layer.
#[derive(Component, Debug)]
pub struct ScreenCamera;
