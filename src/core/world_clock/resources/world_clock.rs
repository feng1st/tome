use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct WorldClock {
    // Pixel Dungeon, Actor.now
    pub now: i64,
    // Pixel Dungeon, Actor.current
    pub solo_performer: Option<Entity>,
}
