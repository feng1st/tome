//! Tween advancement: elapsed grows, values follow, ends remove
//! themselves and announce.

use bevy::prelude::*;

use crate::frontend::display::motion::components::curr_position::CurrPosition;
use crate::frontend::display::tween::components::alpha_tween::AlphaTween;
use crate::frontend::display::tween::components::pos_tween::PosTween;
use crate::frontend::display::tween::messages::tween_finished::TweenFinished;

/// Advance every tween one frame: `elapsed += delta`, the tweened
/// value follows the linear progress, and a tween that reaches its
/// interval writes its final value, removes itself, and emits a
/// `TweenFinished` fact — completion as a broadcast, not a callback.
pub fn update_tweens(
    time: Res<Time>,
    mut commands: Commands,
    mut finished: MessageWriter<TweenFinished>,
    mut alpha_tweens: Query<(Entity, &mut AlphaTween, &mut Sprite)>,
    mut pos_tweens: Query<(Entity, &mut PosTween, &mut CurrPosition)>,
) {
    let delta = time.delta_secs();
    for (entity, mut alpha_tween, mut sprite) in &mut alpha_tweens {
        alpha_tween.elapsed += delta;
        let progress = (alpha_tween.elapsed / alpha_tween.interval).clamp(0.0, 1.0);
        let alpha = alpha_tween.from + (alpha_tween.to - alpha_tween.from) * progress;
        sprite.color = sprite.color.with_alpha(alpha);
        if alpha_tween.elapsed >= alpha_tween.interval {
            commands.entity(entity).remove::<AlphaTween>();
            finished.write(TweenFinished { entity });
        }
    }
    for (entity, mut pos_tween, mut curr_position) in &mut pos_tweens {
        pos_tween.elapsed += delta;
        let progress = (pos_tween.elapsed / pos_tween.interval).clamp(0.0, 1.0);
        let landed = pos_tween.from + (pos_tween.to - pos_tween.from) * progress;
        curr_position.x = landed.x;
        curr_position.y = landed.y;
        if pos_tween.elapsed >= pos_tween.interval {
            commands.entity(entity).remove::<PosTween>();
            finished.write(TweenFinished { entity });
        }
    }
}
