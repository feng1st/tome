//! Frame animation for any entity with animation components.
//! Faces the movement direction via flip_x, derived from cell-space
//! positions (presentation reads the core's state; it never owns it).

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;
use crate::frontend::display::animation::components::anim_clips::AnimClips;
use crate::frontend::display::animation::components::anim_state::AnimState;
use crate::frontend::display::animation::components::anim_timer::AnimTimer;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimQuery {
    pub sprite: &'static mut Sprite,
    pub state: &'static mut AnimState,
    pub clips: &'static AnimClips,
    pub timer: &'static mut AnimTimer,
    pub position: &'static Position,
    pub path: Option<&'static Path>,
}

pub fn animate(time: Res<Time>, mut query: Query<AnimQuery>) {
    for mut item in &mut query {
        let desired = if item.path.is_some() {
            AnimState::Run
        } else {
            AnimState::Idle
        };
        if *item.state != desired {
            *item.state = desired;
            item.timer.set_fps(item.clips.clip(desired).fps);
        }
        item.timer.timer.tick(time.delta());
        if item.timer.timer.just_finished() {
            let clip = item.clips.clip(*item.state);
            item.timer.cursor = (item.timer.cursor + 1) % clip.frames.len();
            if let Some(atlas) = &mut item.sprite.texture_atlas {
                atlas.index = clip.frames[item.timer.cursor];
            }
        }
        // Face the horizontal direction of the current step. Cell space has
        // the same x orientation as world space, so the sign carries over.
        if let Some(path) = item.path {
            let dx = path.step_target.x - item.position.0.x;
            if dx < 0.0 {
                item.sprite.flip_x = true;
            } else if dx > 0.0 {
                item.sprite.flip_x = false;
            }
        }
    }
}
