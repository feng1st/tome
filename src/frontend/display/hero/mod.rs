// TODO: pending cleanup review — remove once stabilized
//! The hero domain's display side: sprite assets, frame tables, and
//! attaching appearance to heroes spawned by the core.

pub mod constants;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the hero display domain: sprite loads are issued on entering
/// `Game`; appearance attaches in the Sync phase — handles need no pixel
/// readiness, the renderer waits (brief pop-in accepted).
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), systems::loading::begin_load)
        .add_systems(
            Update,
            systems::attach_appearance::attach_appearance.in_set(DisplayPhase::Sync),
        );
}
