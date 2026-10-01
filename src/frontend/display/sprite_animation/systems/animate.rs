//! Pure playback (`DisplayPhase::Animate`): the current frame is a pure
//! function of global virtual time, `(elapsed * fps + offset) % len` — no
//! timers, no state derivation, and the sprite is only touched when the
//! frame actually changes. What to play and how to face was decided in
//! `sync_animation` (`DisplayPhase::Sync`).
//!
//! `Time` here is `Time<Virtual>`: animations freeze on pause and follow
//! the game's time scale, which is exactly what presentation wants.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;

/// One animated entity's playback inputs: the figure handle (for the
/// anim table lookup), the playback state, and the sprite to write into.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimQuery {
    pub figure_index: &'static FigureIndex,
    pub state: &'static AnimState,
    pub sprite: &'static mut Sprite,
}

/// Advance every animated entity's sprite frame as a pure function of
/// virtual time; write the atlas index only when the frame changes.
pub fn animate(time: Res<Time>, figure_registry: Res<FigureRegistry>, mut query: Query<AnimQuery>) {
    let elapsed = time.elapsed_secs();
    for mut item in &mut query {
        let anim = figure_registry
            .appearance(*item.figure_index)
            .anim(item.state.anim);
        let len = anim.frames.len();
        let frame = ((elapsed * anim.fps) as usize + item.state.frame_offset) % len;
        let index = anim.frames[frame];
        if let Some(atlas) = &mut item.sprite.texture_atlas {
            if atlas.index != index {
                atlas.index = index;
            }
        }
    }
}
