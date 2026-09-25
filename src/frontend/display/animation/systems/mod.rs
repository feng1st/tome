//! Animation systems.

pub mod animate;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register animation systems into the Animate phase.
pub fn register(app: &mut App) {
    app.add_systems(Update, animate::animate.in_set(DisplayPhase::Animate));
}
