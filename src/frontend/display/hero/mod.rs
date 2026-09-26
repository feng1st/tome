//! The hero domain's display side: sprite assets, frame tables, and
//! attaching appearance to heroes spawned by the core.

pub mod constants;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::frontend::display::display_phase::DisplayPhase;
use crate::frontend::display::loading::assets_ready;

/// Register the hero display domain: sprite loads are signed on entering
/// `Game`; appearance attaches only once every guarded load has finished
/// (no pop-in), in the Sync phase.
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), systems::loading::begin_load)
        .add_systems(
            Update,
            systems::attach_appearance::attach_appearance
                .run_if(assets_ready)
                .in_set(DisplayPhase::Sync),
        );
}
