//! Motion: presentation positions and their glide. Display-side only —
//! real time lives here, never in core.

pub mod components;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the motion domain.
pub fn register(app: &mut App) {
    app.add_systems(Update, systems::slide::slide.in_set(DisplayPhase::Motion));
}
