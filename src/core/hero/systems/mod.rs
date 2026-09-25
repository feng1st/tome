//! Hero game-logic systems.

pub mod commands;

use bevy::prelude::*;

/// Register the hero domain's system groups.
pub fn register(app: &mut App) {
    commands::register(app);
}
