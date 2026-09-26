//! Attaches a creature's look. The core spawns creatures as pure game data
//! (kind markers + `Position`); this system resolves the `AppearanceKind`
//! key through the registry and attaches the renderable parts.

use bevy::prelude::*;

use crate::core::appearance::components::appearance_kind::AppearanceKind;
use crate::core::movement::components::position::Position;
use crate::frontend::display::appearance::resources::appearances::Appearances;
use crate::frontend::display::map::constants::layout::LAYER_ACTOR;
use crate::frontend::display::map::utils::coords::cell_to_world;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

/// Attach sprite, transform and playback state to every entity wearing a
/// new appearance, cloning the registry's handles onto the instance — the
/// renderer reads them off the entity, resolve-once style. The anim table
/// stays in the registry and is looked up per frame. The initial transform
/// carries the actor z layer; x/y are overwritten by `sync_position` every
/// frame from then on.
pub fn attach_appearance(
    mut commands: Commands,
    appearances: Res<Appearances>,
    query: Query<(Entity, &AppearanceKind, &Position), Added<AppearanceKind>>,
) {
    for (entity, kind, pos) in &query {
        let appearance = appearances.appearance(*kind);
        let world = cell_to_world(*pos);
        commands.entity(entity).insert((
            Sprite {
                image: appearance.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: appearance.layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(world.x, world.y, LAYER_ACTOR),
            // A lone creature needs no phase offset; crowds take a random
            // one to desync.
            AnimState {
                anim: AnimKind::Idle,
                frame_offset: 0,
            },
        ));
    }
}
