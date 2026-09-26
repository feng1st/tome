//! The map domain's display side: static tilemap rendering — tileset
//! chunks, layout constants, and cell/pixel conversion. Time-driven
//! terrain visuals live in `terrain_animation`.

pub mod constants;
pub mod entities;
pub mod utils;

use bevy::prelude::*;

use crate::core::app_state::AppState;

/// Register the map display domain: chunks spawn on entering `Game` (the
/// same path a later map switch takes).
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), entities::chunks::spawn_chunks);
}
