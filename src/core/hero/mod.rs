//! The hero domain's game-data side: marker, spawning, and command
//! execution. Appearance lives in the frontend.

pub mod commands;
pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

/// Register hero-domain game data: command message types (delegated to the
/// commands side) and the spawn system. Command executors join the
/// core-side chain registered in `core::register`.
pub fn register(app: &mut App) {
    commands::register(app);
    app.add_systems(Startup, entities::hero::spawn_hero);
}
