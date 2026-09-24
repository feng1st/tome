//! Attaches the hero's display components. The core spawns heroes as pure
//! game data (`Hero` + `Position`); this system is the display side's
//! registered appearance, reacting to `Added<Hero>` so later entity kinds
//! follow the same pattern.

use bevy::prelude::*;

use crate::core::hero::components::hero::Hero;
use crate::core::movement::components::position::Position;
use crate::frontend::display::animation::components::anim_clips::AnimClips;
use crate::frontend::display::animation::components::anim_state::AnimState;
use crate::frontend::display::animation::components::anim_timer::AnimTimer;
use crate::frontend::display::camera::components::camera_target::CameraTarget;
use crate::frontend::display::hero::constants::anim_frames::{
    IDLE, RUN, WARRIOR_COLS, WARRIOR_FRAME, WARRIOR_ROWS, WARRIOR_TEXTURE,
};
use crate::frontend::display::map::constants::layout::LAYER_ACTOR;
use crate::frontend::display::map::utils::coords::cell_to_world;

/// Attach sprite, animation and camera components to every newly spawned
/// hero, with the tier-0 (unarmored) warrior look. The initial transform
/// carries the actor z layer; x/y are overwritten by `sync_position` every
/// frame from then on.
pub fn attach_appearance(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<(Entity, &Position), Added<Hero>>,
) {
    for (entity, pos) in &query {
        let texture = asset_server.load(WARRIOR_TEXTURE);
        let layout = layouts.add(TextureAtlasLayout::from_grid(
            WARRIOR_FRAME,
            WARRIOR_COLS,
            WARRIOR_ROWS,
            None,
            None,
        ));
        let world = cell_to_world(pos.0);
        commands.entity(entity).insert((
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas { layout, index: 0 }),
                ..default()
            },
            Transform::from_xyz(world.x, world.y, LAYER_ACTOR),
            CameraTarget,
            AnimState::Idle,
            AnimClips {
                idle: IDLE,
                run: RUN,
            },
            AnimTimer::new(IDLE.fps),
        ));
    }
}
