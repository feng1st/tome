// TODO: pending cleanup review — remove once stabilized
//! The map domain's display side: tileset textures, chunk rendering,
//! scrolling terrain layers, and cell/pixel conversion.

pub mod components;
pub mod constants;
pub mod entities;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the map display domain: chunks and scrolling layers spawn on
/// entering `Game` (the same path a later map switch takes); the scroll
/// offset advances in the Animate phase.
pub fn register(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::Game),
        (
            entities::chunks::spawn_chunks,
            entities::scroll_layers::spawn_scroll_layers,
        ),
    )
    .add_systems(
        Update,
        systems::scroll_terrain::scroll_terrain.in_set(DisplayPhase::Animate),
    );
}
