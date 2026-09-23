//! Input gestures: what the pointer hit, typed by target. Frontend-internal
//! messages from modalities to the resolvers — the core never sees them
//! (the cross-side protocol is the commands in `core::hero::commands`).
//!
//! Design paradigm: one gesture type per hittable target kind. New hittable
//! things add a new gesture file plus a new resolver; existing code is
//! never modified.

pub mod primary_action_on_cell;
pub mod primary_action_on_monster;
pub mod primary_action_on_world_cell;

use bevy::prelude::*;

use primary_action_on_cell::PrimaryActionOnCell;

/// Register the gesture message types. Paradigm-marker gestures join this
/// list when a modality first emits them.
pub fn register(app: &mut App) {
    app.add_message::<PrimaryActionOnCell>();
}
