//! The tween domain: linear interpolations that advance themselves —
//! alpha fades and position glides. A tween ends by removing itself
//! and broadcasting `TweenFinished`; what a fade or glide belongs to
//! (a death, a step) is its caller's business, never this domain's.

pub mod components;
pub mod messages;
pub mod systems;

use bevy::prelude::*;

use self::messages::tween_finished::TweenFinished;
use self::systems::update_tweens::update_tweens;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the tween domain: the finished-fact message exists; the
/// advancer runs with the rest of presentation's time, in the Animate
/// phase, ahead of the camera and the snap.
pub fn register(app: &mut App) {
    app.add_message::<TweenFinished>()
        .add_systems(Update, update_tweens.in_set(DisplayPhase::Animate));
}
