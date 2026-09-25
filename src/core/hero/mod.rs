//! The hero domain's game-data side: marker, spawning, and command
//! execution. Appearance lives in the frontend.

pub mod commands;
pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

/// Register the hero domain: command messages, executors, and spawning.
pub fn register(app: &mut App) {
    commands::register(app);
    systems::register(app);
    entities::register(app);
}
