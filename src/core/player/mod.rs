//! The player domain's game-data side: marker, standing orders,
//! spawning, and command execution. Appearance lives in the frontend.

pub mod commands;
pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

use self::commands::target_cell::TargetCell;
use crate::core::app_state::AppState;
use crate::core::core_phase::CorePhase;

/// Register the player domain: command messages, executors, and spawning.
pub fn register(app: &mut App) {
    app.add_message::<TargetCell>()
        .add_systems(OnEnter(AppState::Game), entities::player::spawn_player)
        .add_systems(
            Update,
            systems::commands::target_cell::execute.in_set(CorePhase::Command),
        )
        .add_systems(
            Update,
            systems::plan_action::plan_action.in_set(CorePhase::PlayerPlan),
        );
}
