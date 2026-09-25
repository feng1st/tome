//! Hero display systems.

pub mod attach_appearance;
pub mod loading;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::frontend::display::display_phase::DisplayPhase;
use crate::frontend::display::loading::assets_ready;

/// Register hero display systems: sprite loads signed on entering `InGame`;
/// appearance attaches only once every guarded load has finished (no
/// pop-in), in the Sync phase.
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), loading::begin_load)
        .add_systems(
            Update,
            attach_appearance::attach_appearance
                .run_if(assets_ready)
                .in_set(DisplayPhase::Sync),
        );
}
