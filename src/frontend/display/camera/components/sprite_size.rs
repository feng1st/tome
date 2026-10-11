use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct SpriteSize {
    size: Vec2,
}

impl SpriteSize {
    pub fn new(size: Vec2) -> Self {
        SpriteSize { size }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }
}
