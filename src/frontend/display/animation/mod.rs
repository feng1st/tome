//! Frame animation for sprite entities: clip tables per creature template,
//! playback instructions derived in Sync, pure playback in Animate.

pub mod components;
pub mod constants;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the animation domain.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        (
            systems::sync_animation::sync_animation.in_set(DisplayPhase::Sync),
            systems::animate::animate.in_set(DisplayPhase::Animate),
        ),
    );
}
