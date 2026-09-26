// TODO: pending cleanup review — remove once stabilized
//! Frame animation for sprite entities: clips, states, timers.

pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the animation domain.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::animate::animate.in_set(DisplayPhase::Animate),
    );
}
