pub mod resources;

use bevy::prelude::*;

use self::resources::game_rng::GameRng;

pub fn register(app: &mut App) {
    app.init_resource::<GameRng>();
}
