//! The death fade: a claimed body fades out, then releases itself to
//! the departure sweep.

use bevy::prelude::*;

use crate::core::display::components::is_disappearing::IsDisappearing;
use crate::frontend::display::effects::constants::feedback::FADE_TIME;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::messages::anim_finished::AnimFinished;
use crate::frontend::display::tween::components::alpha_tween::AlphaTween;
use crate::frontend::display::tween::messages::tween_finished::TweenFinished;

/// The die animation ran to its end on a claimed body: the fade opens.
/// An `AlphaTween` carries the body's alpha to nothing over the fade
/// time and ends itself — the completion callback as a message.
pub fn start_dead_fade(
    mut anims_finished: MessageReader<AnimFinished>,
    claimed: Query<(), With<IsDisappearing>>,
    mut commands: Commands,
) {
    for anim_finished in anims_finished.read() {
        if anim_finished.anim != AnimKind::Die {
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
    mut tweens_finished: MessageReader<TweenFinished>,
    claimed: Query<(), With<IsDisappearing>>,
    mut commands: Commands,
) {
    for tween_finished in tweens_finished.read() {
        if claimed.get(tween_finished.entity).is_ok() {
            commands
                .entity(tween_finished.entity)
                .remove::<IsDisappearing>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::display::components::is_disappearing::IsDisappearing;
    use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
    use crate::frontend::display::sprite_animation::messages::anim_finished::AnimFinished;
    use crate::frontend::display::tween::components::alpha_tween::AlphaTween;
    use crate::frontend::display::tween::messages::tween_finished::TweenFinished;

    fn app() -> App {
        let mut app = App::new();
        app.add_message::<AnimFinished>()
            .add_message::<TweenFinished>()
            .add_systems(Update, (start_dead_fade, release_dead_fade).chain());
        app
    }

    fn spawn_claimed_body(app: &mut App) -> Entity {
        app.world_mut()
            .spawn((IsDisappearing, Sprite::default()))
            .id()
    }

    #[test]
    fn the_die_end_opens_the_fade_on_a_claimed_body() {
        let mut app = app();
        let body = spawn_claimed_body(&mut app);
        app.world_mut().write_message(AnimFinished {
            entity: body,
            anim: AnimKind::Die,
        });
        app.update();
        let tween = app.world().get::<AlphaTween>(body).unwrap();
        assert_eq!(tween.from, 1.0);
        assert_eq!(tween.to, 0.0);
        assert_eq!(tween.interval, FADE_TIME);
    }

    #[test]
    fn an_unclaimed_body_never_fades() {
        let mut app = app();
        let passerby = app.world_mut().spawn(Sprite::default()).id();
        app.world_mut().write_message(AnimFinished {
            entity: passerby,
            anim: AnimKind::Die,
        });
        app.update();
        assert!(app.world().get::<AlphaTween>(passerby).is_none());
    }

    #[test]
    fn a_non_die_end_never_fades() {
        let mut app = app();
        let body = spawn_claimed_body(&mut app);
        app.world_mut().write_message(AnimFinished {
            entity: body,
            anim: AnimKind::Attack,
        });
        app.update();
        assert!(app.world().get::<AlphaTween>(body).is_none());
    }

    #[test]
    fn the_fade_end_releases_the_claim() {
        let mut app = app();
        let body = spawn_claimed_body(&mut app);
        app.world_mut()
            .write_message(TweenFinished { entity: body });
        app.update();
        assert!(
            app.world().get::<IsDisappearing>(body).is_none(),
            "the body is released to the departure sweep"
        );
    }
}
