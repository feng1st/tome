//! Movement systems.

pub mod follow_path;

use bevy::prelude::*;

use crate::core::app_state::InGameState;
use crate::core::core_phase::CorePhase;

/// Register movement systems: path following advances in the Act phase,
/// gated on `InGameState::LocalMap` (world-map travel moves differently).
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        follow_path::follow_path
            .in_set(CorePhase::Act)
            .run_if(in_state(InGameState::LocalMap)),
    );
}
