use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NextTurn {
    // Pixel Dungeon, Actor.time
    pub at: i64,
}
