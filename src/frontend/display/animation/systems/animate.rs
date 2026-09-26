// TODO: pending cleanup review — remove once stabilized
//! Stateless frame animation: the current frame is a pure function of
//! global virtual time, `(elapsed * fps + start) % len` — no timers, no
//! per-frame ticking, and the sprite is only touched when the frame
//! actually changes. Faces the movement direction via flip_x, derived
//! from cell-space positions (presentation reads the core's state; it
//! never owns it).
//!
//! `Time` here is `Time<Virtual>`: animations freeze on pause and follow
//! the game's time scale, which is exactly what presentation wants.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;
use crate::frontend::display::animation::components::anim_clips::AnimClips;
use crate::frontend::display::animation::components::anim_start::AnimStartFrame;
use crate::frontend::display::animation::components::anim_state::AnimState;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimQuery {
    pub sprite: &'static mut Sprite,
    pub state: &'static mut AnimState,
    pub clips: &'static AnimClips,
    pub start: &'static AnimStartFrame,
    pub position: &'static Position,
    pub path: Option<&'static Path>,
}

pub fn animate(time: Res<Time>, mut query: Query<AnimQuery>) {
    let elapsed = time.elapsed_secs();
    for mut item in &mut query {
        let desired = if item.path.is_some() {
            AnimState::Run
        } else {
            AnimState::Idle
        };
        if *item.state != desired {
            *item.state = desired;
        }
        // Stateless loop: frame index derives from global time, so state
        // switches land on the globally-synced frame instead of resetting.
        let clip = item.clips.clip(*item.state);
        let frame = (elapsed * clip.fps) as usize + item.start.0;
        let index = clip.frames[frame % clip.frames.len()];
        if let Some(atlas) = &mut item.sprite.texture_atlas {
            if atlas.index != index {
                atlas.index = index;
            }
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
