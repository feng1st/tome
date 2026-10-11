use bevy::prelude::*;

use crate::core::display_protocol::components::is_performing::IsPerforming;
use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::tween::components::pos_tween::PosTween;

#[allow(clippy::type_complexity)]
pub fn update_is_performing(
    time: Res<Time>,
    mut commands: Commands,
    trailing: Query<
        (
            Entity,
            Option<&PosTween>,
            Option<&AnimKind>,
            Option<&AnimState>,
        ),
        Without<IsPerforming>,
    >,
    flagged: Query<
        (
            Entity,
            Option<&PosTween>,
            Option<&AnimKind>,
            Option<&AnimState>,
        ),
        With<IsPerforming>,
    >,
) {
    let step = time.delta_secs();
    for (entity, pos_tween, anim_kind, anim_state) in &trailing {
        if holds(pos_tween, anim_kind.copied(), anim_state, step) {
            commands.entity(entity).insert(IsPerforming);
        }
    }
    for (entity, pos_tween, anim_kind, anim_state) in &flagged {
        if !holds(pos_tween, anim_kind.copied(), anim_state, step) {
            commands.entity(entity).remove::<IsPerforming>();
        }
    }
}

/// Whether the pictures still owe this creature something: a glide is
/// owed while road remains; a swing, while its last frame hasn't
/// played — the kind component speaks from the sync (same frame as the
/// landed blow), the finished fact from the animator.
fn holds(
    pos_tween: Option<&PosTween>,
    anim_kind: Option<AnimKind>,
    anim_state: Option<&AnimState>,
    step: f32,
) -> bool {
    let gliding = pos_tween.is_some_and(|tween| tween.interval - tween.elapsed > step);
    let swinging = anim_kind.is_some_and(|kind| kind == AnimKind::Attack)
        && anim_state.is_some_and(|state| !state.finished);
    gliding || swinging
}
