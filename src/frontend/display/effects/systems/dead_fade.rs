//! The death fade: a claimed body fades out, then releases itself to
//! the departure sweep.

use bevy::prelude::*;

use crate::core::display_protocol::components::is_disappearing::IsDisappearing;
use crate::frontend::display::effects::constants::feedback::FADE_TIME;
use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::messages::anim_finished::AnimFinished;
use crate::frontend::display::tween::components::alpha_tween::AlphaTween;
use crate::frontend::display::tween::messages::tween_finished::TweenFinished;

/// The die animation ran to its end on a claimed body: the fade opens.
/// An `AlphaTween` carries the body's alpha to nothing over the fade
/// time and ends itself — the completion callback as a message.
pub fn start_dead_fade(
    mut anims_finished_reader: MessageReader<AnimFinished>,
    claimed: Query<(), With<IsDisappearing>>,
    mut commands: Commands,
) {
    for anim_finished in anims_finished_reader.read() {
        if anim_finished.anim_kind != AnimKind::Die {
            continue;
        }
        if claimed.get(anim_finished.entity).is_ok() {
            commands.entity(anim_finished.entity).insert(AlphaTween {
                from: 1.0,
                to: 0.0,
                elapsed: 0.0,
                interval: FADE_TIME,
            });
        }
    }
}

/// The fade ran to its end on a claimed body: the claim releases, and
/// the departure sweep may take the body on its next pass.
pub fn release_dead_fade(
    mut tweens_finished_reader: MessageReader<TweenFinished>,
    claimed: Query<(), With<IsDisappearing>>,
    mut commands: Commands,
) {
    for tween_finished in tweens_finished_reader.read() {
        if claimed.get(tween_finished.entity).is_ok() {
            commands
                .entity(tween_finished.entity)
                .remove::<IsDisappearing>();
        }
    }
}
