//! The combat domain: player-side combat numbers — the stat bonus
//! tables, the combat-bonus component derived from them, and the
//! composite formulas built on both.

pub mod components;
pub mod constants;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::core::core_phase::CorePhase;

/// Register the combat domain: the derive system keeps the
/// combat-bonus component fresh in the Derive phase, before any turn
/// logic reads it.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::derive_combat_bonuses::derive_combat_bonuses.in_set(CorePhase::Derive),
    );
}
