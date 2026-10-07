//! The combat domain: the strike pipeline both sides share — the blows
//! and armor-class components, their player-side derivation,
//! and the executor — plus the composite formulas built on the
//! combat-bonus component and the stat bonus tables.

pub mod components;
pub mod constants;
pub mod systems;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;

/// Register the combat domain: the derive chain keeps the bonus and
/// strike components fresh in the Derive phase, before any turn logic
/// reads them, and the strike executor runs in both act phases — the
/// player's and the world's.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        (
            (
                systems::derive_combat_bonuses::derive_combat_bonuses,
                systems::derive_strike::derive_strike,
            )
                .chain()
                .in_set(CorePhase::Derive),
            systems::act_attack::act_attack.in_set(CorePhase::PlayerAct),
            systems::act_attack::act_attack.in_set(CorePhase::WorldAct),
        ),
    );
}
