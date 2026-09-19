//! Frame animation for any entity with animation components.
//! Faces the movement direction via flip_x (presentation lives here,
//! not in movement).

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::animation::components::anim_clips::AnimClips;
use crate::animation::components::anim_state::AnimState;
use crate::animation::components::anim_timer::AnimTimer;
use crate::movement::components::path::Path;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimQuery {
    pub sprite: &'static mut Sprite,
    pub state: &'static mut AnimState,
    pub clips: &'static AnimClips,
    pub timer: &'static mut AnimTimer,
    pub transform: &'static Transform,
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
        // Face the horizontal direction of the current step.
        if let Some(path) = item.path {
            let dx = path.step_target.x - item.transform.translation.x;
            if dx < 0.0 {
                item.sprite.flip_x = true;
            } else if dx > 0.0 {
                item.sprite.flip_x = false;
            }
        }
    }
}
