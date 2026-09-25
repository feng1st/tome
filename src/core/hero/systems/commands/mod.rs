//! Command executors: one file per command, each exposing `execute`.

pub mod move_to_cell;

use bevy::prelude::*;

use crate::core::app_state::InGameState;
use crate::core::core_phase::CorePhase;

/// Register executors into the Act phase. Command execution is a local-map
/// mechanic, so it gates itself on `InGameState::LocalMap`.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        move_to_cell::execute
            .in_set(CorePhase::Act)
            .run_if(in_state(InGameState::LocalMap)),
    );
}
