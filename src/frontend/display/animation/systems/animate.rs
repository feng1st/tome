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

use crate::core::appearance::components::appearance_kind::AppearanceKind;
use crate::frontend::display::animation::components::anim_state::AnimState;
use crate::frontend::display::appearance::resources::appearances::Appearances;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimQuery {
    pub kind: &'static AppearanceKind,
    pub state: &'static AnimState,
    pub sprite: &'static mut Sprite,
}

pub fn animate(time: Res<Time>, appearances: Res<Appearances>, mut query: Query<AnimQuery>) {
    let elapsed = time.elapsed_secs();
    for mut item in &mut query {
        let clip = appearances.appearance(*item.kind).clip(item.state.anim);
        let len = clip.frames.len();
        let frame = ((elapsed * clip.fps) as usize + item.state.frame_offset) % len;
        let index = clip.frames[frame];
        if let Some(atlas) = &mut item.sprite.texture_atlas {
            if atlas.index != index {
                atlas.index = index;
            }
        }
    }
}
