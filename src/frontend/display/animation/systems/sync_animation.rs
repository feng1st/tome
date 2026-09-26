//! Derives playback intent and facing from core state (`DisplayPhase::Sync`
//! work: presentation reads the core's state; it never owns it). Writes
//! `Playback` only on anim switches — steady-state playback is a pure
//! function of global time, computed in `animate`.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;
use crate::frontend::display::animation::components::anim_clips::AnimClips;
use crate::frontend::display::animation::components::anim_state::AnimState;
use crate::frontend::display::animation::constants::anim_kind::AnimKind;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimSyncQuery {
    pub path: Option<&'static Path>,
    pub position: &'static Position,
    pub clips: &'static AnimClips,
    pub state: &'static mut AnimState,
    pub sprite: &'static mut Sprite,
}

pub fn sync_animation(time: Res<Time>, mut query: Query<AnimSyncQuery>) {
    let elapsed = time.elapsed_secs();
    for mut item in &mut query {
        let desired = if item.path.is_some() {
            AnimKind::Run
        } else {
            AnimKind::Idle
        };
        if item.state.anim != desired {
            let clip = item.clips.clip(desired);
            item.state.switch(desired, clip, elapsed);
        }
        // Face the horizontal direction of the current step. Cell space has
        // the same x orientation as world space, so the sign carries over.
        if let Some(path) = item.path {
            let dx = path.step_to.x - item.position.x;
            if dx < 0.0 {
                item.sprite.flip_x = true;
            } else if dx > 0.0 {
                item.sprite.flip_x = false;
            }
        }
    }
}
