// TODO: pending cleanup review — remove once stabilized
//! Attaches the hero's display components. The core spawns heroes as pure
//! game data (`Hero` + `Position`); this system is the display side's
//! registered appearance, reacting to `Added<Hero>` so later entity kinds
//! follow the same pattern.

use bevy::prelude::*;

use crate::core::hero::components::hero::Hero;
use crate::core::movement::components::position::Position;
use crate::frontend::display::animation::components::anim_state::AnimState;
use crate::frontend::display::animation::constants::anim_kind::AnimKind;
use crate::frontend::display::camera::components::camera_target::CameraTarget;
use crate::frontend::display::hero::constants::anim_frames::warrior_clips;
use crate::frontend::display::hero::resources::hero_sprites::HeroSprites;
use crate::frontend::display::map::constants::layout::LAYER_ACTOR;
use crate::frontend::display::map::utils::coords::cell_to_world;

/// Attach sprite, animation and camera components to every newly spawned
/// hero, cloning the preloaded asset handles. The initial transform carries
/// the actor z layer; x/y are overwritten by `sync_position` every frame
/// from then on.
pub fn attach_appearance(
    mut commands: Commands,
    sprites: Res<HeroSprites>,
    query: Query<(Entity, &Position), Added<Hero>>,
) {
    for (entity, pos) in &query {
        let world = cell_to_world(*pos);
        commands.entity(entity).insert((
            Sprite {
                image: sprites.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: sprites.layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(world.x, world.y, LAYER_ACTOR),
            CameraTarget,
            warrior_clips(),
            // A lone hero needs no phase offset; crowds take a random one.
            AnimState {
                anim: AnimKind::Idle,
                frame_offset: 0,
            },
        ));
    }
}
