use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct HitPoints {
    pub current: i32,
    pub max: i32,
}

impl HitPoints {
    // Pixel Dungeon, Char.isAlive
    pub fn is_dead(&self) -> bool {
        self.current < 0
    }
}
