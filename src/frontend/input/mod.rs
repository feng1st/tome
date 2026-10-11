pub mod systems;

use bevy::prelude::*;

use crate::core::game_loop::GameLoop;

pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::translate_mouse::translate_mouse.in_set(GameLoop::Input),
    );
}
