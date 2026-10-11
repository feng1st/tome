pub mod systems;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;

pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::act_turns::act_turns.in_set(CorePhase::Turn),
    );
}
