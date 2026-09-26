//! Terrain animation: time-driven terrain visuals (today: one scrolling
//! UV layer per animated terrain kind, PD's `SkinnedBlock` approach).
//! Mechanism-owned — frame-table creature animation lives in
//! `sprite_animation`; both feed `DisplayPhase::Animate`.

pub mod components;
pub mod constants;
pub mod entities;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the terrain animation domain: animated layers spawn on
/// entering `Game` (the same path a later map switch takes); playback
/// advances in the Animate phase.
pub fn register(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::Game),
        entities::anim_layers::spawn_anim_layers,
    )
    .add_systems(
        Update,
        systems::animate::animate.in_set(DisplayPhase::Animate),
    );
}
