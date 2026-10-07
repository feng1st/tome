//! Frame animation for sprite entities: anim tables per figure,
//! playback intent derived and one-shots switched in Sync, pure
//! playback in Animate. A one-shot's end announces itself as a
//! message — the completion callback, broadcast.

pub mod components;
pub mod constants;
pub mod messages;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use self::messages::anim_finished::AnimFinished;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the animation domain.
pub fn register(app: &mut App) {
    app.add_message::<AnimFinished>().add_systems(
        Update,
        (
            systems::sync_animation::sync_animation.in_set(DisplayPhase::Sync),
            systems::animate::animate.in_set(DisplayPhase::Animate),
        ),
    );
}
