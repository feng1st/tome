//! The random source: one seedable generator feeding every consumer
//! of chance in the game.

pub mod resources;

use bevy::prelude::*;

use self::resources::game_rng::GameRng;

/// Register the domain's members: the central random source.
pub fn register(app: &mut App) {
    app.init_resource::<GameRng>();
}
