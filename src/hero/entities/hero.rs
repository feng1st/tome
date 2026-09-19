use bevy::prelude::*;

use crate::animation::components::anim_clips::{AnimClip, AnimClips};
use crate::animation::components::anim_state::AnimState;
use crate::animation::components::anim_timer::AnimTimer;
use crate::camera::components::camera_target::CameraTarget;
use crate::hero::components::hero::Hero;
use crate::map::constants::layout::LAYER_ACTOR;
use crate::map::resources::grid_map::GridMap;

const HERO_START: IVec2 = IVec2::new(32, 10);

// Warrior sheet frame tables (tier 0, unarmored). Sequences match the
// original HeroSprite: idle breathes between frames 0/1, run cycles 2..7.
const IDLE: AnimClip = AnimClip {
    frames: &[0, 0, 0, 1, 0, 0, 1, 1],
    fps: 8.0,
};
const RUN: AnimClip = AnimClip {
    frames: &[2, 3, 4, 5, 6, 7],
    fps: 20.0,
};

/// Startup system: spawn the hero with the tier-0 (unarmored) sprite frames.
pub fn spawn_hero(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("warrior.png");
    // Warrior sheet: 12x15 frames, tier 0 (top row) is the unarmored look.
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(12, 15),
        21,
        8,
        None,
        None,
    ));
    let center = GridMap::cell_center(HERO_START);
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas { layout, index: 0 }),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, LAYER_ACTOR),
        Hero,
        CameraTarget,
        AnimState::Idle,
        AnimClips {
            idle: IDLE,
            run: RUN,
        },
        AnimTimer::new(IDLE.fps),
    ));
}
