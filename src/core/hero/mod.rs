//! The hero domain's game-data side: marker, spawning, and command
//! execution. Appearance lives in the frontend.

pub mod commands;
pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::core::core_phase::CorePhase;

use commands::move_to_cell::MoveToCell;

/// Register the hero domain: command messages, executors, and spawning.
pub fn register(app: &mut App) {
    app.add_message::<MoveToCell>()
        .add_systems(OnEnter(AppState::Game), entities::hero::spawn_hero)
        .add_systems(
            Update,
            systems::commands::move_to_cell::execute.in_set(CorePhase::Plan),
        );
}
