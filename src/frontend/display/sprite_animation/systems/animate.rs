use bevy::prelude::*;

use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::messages::anim_finished::AnimFinished;

pub fn animate(
    time: Res<Time>,
    figure_registry: Res<FigureRegistry>,
    mut targets: Query<(Entity, &FigureIndex, &AnimKind, &mut AnimState, &mut Sprite)>,
    mut anim_finished_writer: MessageWriter<AnimFinished>,
) {
    let elapsed = time.elapsed_secs();
    for (entity, &figure_index, &anim_kind, mut anim_state, mut sprite) in &mut targets {
        let anim = figure_registry.figure(figure_index).anim(anim_kind);
        let _ = anim_state.switch_if_neq(anim_kind, anim, elapsed);
        let frame_count = anim.frames.len();
        let frame = if anim.looped {
            ((elapsed * anim.fps) as usize + anim_state.frame_offset) % frame_count
        } else {
            let frame = ((elapsed - anim_state.started_at) * anim.fps) as usize;
            if frame >= frame_count {
                if !anim_state.finished {
                    anim_state.finished = true;
                    anim_finished_writer.write(AnimFinished {
                        entity,
                        anim_kind: anim_state.anim_kind,
                    });
                }
                frame_count - 1
            } else {
                frame
            }
        };
        if let Some(atlas) = &mut sprite.texture_atlas {
            let index = anim.frames[frame];
            if atlas.index != index {
                atlas.index = index;
            }
        }
    }
}
