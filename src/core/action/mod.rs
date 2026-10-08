//! The action domain: the transient turn actions every species shares
//! — the step and the attack — and the executors that consume them in
//! the same frame they were planned. Deciding an action (routes,
//! targets, species policy) is the planning side's business; this
//! domain only executes. Pure game logic — no rendering types, no real
//! time involved.

pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;

/// Register the action domain: both executors run in the two act
/// phases — the player's and the world's — consuming exactly what the
/// plan phases produced this frame.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        (
            systems::act_move::act_move.in_set(CorePhase::PlayerAct),
            systems::act_move::act_move.in_set(CorePhase::WorldAct),
            systems::act_attack::act_attack.in_set(CorePhase::PlayerAct),
            systems::act_attack::act_attack.in_set(CorePhase::WorldAct),
        ),
    );
}
