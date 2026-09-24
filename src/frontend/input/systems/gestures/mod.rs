//! Gesture resolvers: interpret gestures into core commands, using game
//! state. This is where interface policy lives — what a primary action on
//! a given target means. One resolver per gesture, mirroring `gestures/`
//! one-to-one; modalities stay semantics-free.

pub mod primary_action_on_cell;

use bevy::prelude::*;

use crate::frontend::input::input_phase::InputPhase;

/// Register resolvers into the Resolve phase. A new gesture adds a mirrored
/// file above and a line here.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        primary_action_on_cell::resolve.in_set(InputPhase::Resolve),
    );
}
