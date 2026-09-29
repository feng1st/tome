//! Marker for the canvas camera: it renders the world into the canvas.

use bevy::prelude::*;

/// Marks the camera that renders the world into the canvas. Click
/// handling resolves through it to convert canvas coordinates to world
/// coordinates; the screen camera presents the result.
#[derive(Component)]
pub struct CanvasCamera;
