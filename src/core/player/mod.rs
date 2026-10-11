pub mod commands;
pub mod components;
pub mod entities;
pub mod systems;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use self::commands::target_cell::TargetCell;
use crate::core::app_state::AppState;
use crate::core::core_phase::CorePhase;

pub fn register(app: &mut App) {
    app.add_message::<TargetCell>()
        .add_systems(OnEnter(AppState::Game), entities::player::spawn_player)
        .add_systems(
            Update,
            systems::commands::target_cell::target_cell.in_set(CorePhase::Command),
        )
        .add_systems(
            Update,
            systems::interrupt_order::interrupt_order
                .after(crate::core::turn::systems::act_turns::act_turns)
                .in_set(CorePhase::Turn),
        );
}
