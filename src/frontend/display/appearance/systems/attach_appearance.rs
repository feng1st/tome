//! Attaches a creature's appearance. The core spawns creatures as pure
//! game data (kind markers + a `FigureIndex` handle + `Position`); this
//! system resolves the handle through the appearance registry and
//! attaches the renderable parts.

use bevy::prelude::*;

use crate::core::figure::components::figure_index::FigureIndex;
use crate::core::movement::components::position::Position;
use crate::frontend::display::appearance::resources::appearance_registry::AppearanceRegistry;
use crate::frontend::display::constants::layout::LAYER_ACTOR;
use crate::frontend::display::map::utils::coords::cell_to_world;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

/// Attach sprite, transform and playback state to every entity
/// presenting a new figure, cloning the registry's handles onto the
/// instance — the renderer reads them off the entity, resolve-once
/// style. The anim
/// table stays in the registry and is looked up per frame. The initial
/// transform carries the actor z layer; x/y are overwritten by
/// `sync_position` every frame from then on.
pub fn attach_appearance(
    mut commands: Commands,
    appearance_registry: Res<AppearanceRegistry>,
    query: Query<(Entity, &FigureIndex, &Position), Added<FigureIndex>>,
) {
    for (entity, figure_index, pos) in &query {
        let appearance = appearance_registry.appearance(*figure_index);
        let world = cell_to_world(*pos);
        // The phase offset derives from the standing cell: a deterministic
        // desync for crowds spawned on the same tick (zero would sync
        // them).
        let idle_len = appearance.clip(AnimKind::Idle).frames.len();
        let cell = pos.cell();
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
            AnimState {
                anim: AnimKind::Idle,
                frame_offset: (cell.x + cell.y) as usize % idle_len,
            },
        ));
    }
}
