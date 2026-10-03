//! Attaches a creature's appearance. Creatures enter the world as pure
//! game data plus a `FigureIndex` handle (resolved from their identity
//! by the binding side); this system resolves the handle through the
//! figure registry and attaches the renderable parts.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::frontend::display::camera::components::sprite_size::SpriteSize;
use crate::frontend::display::constants::layout::LAYER_ACTOR;
use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::figure::utils::sprite_anchor::sprite_anchor;
use crate::frontend::display::map::utils::coords::cell_to_world;
use crate::frontend::display::motion::components::presentation_position::PresentationPosition;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

/// Attach sprite, transform and playback state to every entity
/// presenting a new figure, cloning the registry's handles onto the
/// instance — the renderer reads them off the entity, resolve-once
/// style. The anim table stays in the registry and is looked up per
/// frame. The initial transform carries the actor z layer; x/y are
/// rewritten by the presentation snap every frame from then on.
pub fn attach_appearance(
    mut commands: Commands,
    figure_registry: Res<FigureRegistry>,
    query: Query<(Entity, &FigureIndex, &CellCoord), Added<FigureIndex>>,
) {
    for (entity, figure_index, cell) in &query {
        let appearance = figure_registry.appearance(*figure_index);
        let world = cell_to_world(PresentationPosition::from(*cell));
        // The phase offset derives from the standing cell: a deterministic
        // desync for crowds spawned on the same tick (zero would sync
        // them).
        let idle_len = appearance.anim(AnimKind::Idle).frames.len();
        let cell = *cell;
        commands.entity(entity).insert((
            PresentationPosition::from(cell),
            Sprite {
                image: appearance.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: appearance.layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            sprite_anchor(appearance.frame_size),
            SpriteSize::new(appearance.frame_size.as_vec2()),
            Transform::from_xyz(world.x, world.y, LAYER_ACTOR),
            AnimState {
                anim: AnimKind::Idle,
                frame_offset: (cell.x + cell.y) as usize % idle_len,
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec-alignment: a new figure handle mounts the sprite, the actor
    /// layer, and the idle playback state.
    #[test]
    fn new_figure_handle_mounts_sprite_and_playback_state() {
        use bevy::ecs::system::RunSystemOnce;

        use crate::core::map::components::cell_coord::CellCoord;

        let mut world = World::new();
        world.insert_resource(FigureRegistry::for_test(&["warrior"]));
        let entity = world
            .spawn((FigureIndex::from_index(0), CellCoord::new(2, 3)))
            .id();
        world.run_system_once(attach_appearance).unwrap();

        let sprite = world.get::<Sprite>(entity).expect("sprite mounted");
        assert!(sprite.texture_atlas.is_some());
        let transform = world.get::<Transform>(entity).expect("transform mounted");
        assert_eq!(transform.translation.z, LAYER_ACTOR);
        let state = world.get::<AnimState>(entity).expect("playback mounted");
        assert_eq!(state.anim, AnimKind::Idle);
        // The presentation position mounts from the spawn cell.
        assert_eq!(
            world.get::<PresentationPosition>(entity).copied(),
            Some(PresentationPosition::from(CellCoord::new(2, 3)))
        );
    }
}
