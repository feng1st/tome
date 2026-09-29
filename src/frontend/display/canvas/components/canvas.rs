//! The canvas marker: the fullscreen presentation of the canvas texture.

use bevy::prelude::*;

/// Marks the sprite that presents the canvas texture. Kept at the
/// canvas's natural size; the follow rig pans it by the sub-pixel
/// remainder for smooth scrolling, and the screen camera's fixed
/// projection does the scaling and view cropping.
#[derive(Component, Debug)]
pub struct Canvas;
