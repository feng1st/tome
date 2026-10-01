//! The presented sprite's size: what the snap system needs to land the
//! sprite's edges on the screen grid.

use bevy::prelude::*;

/// The presented sprite's size in world pixels, attached at spawn by
/// the domain that knows the frame size. The snap system reads it to
/// compute the translation-to-min-corner offset; without it the entity
/// presents unsnapped.
#[derive(Component, Clone, Copy, Debug)]
pub struct SpriteSize {
    size: Vec2,
}

impl SpriteSize {
    /// The presented sprite's size in world pixels.
    pub fn new(size: Vec2) -> Self {
        SpriteSize { size }
    }

    /// The sprite's size in world pixels.
    pub fn size(&self) -> Vec2 {
        self.size
    }
}
