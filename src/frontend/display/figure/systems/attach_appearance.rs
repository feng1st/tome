//! Attaches a creature's appearance. Creatures enter the world as pure
//! game data plus a `FigureIndex` handle (resolved from their identity
//! by the binding side); this system resolves the handle through the
//! figure registry and attaches the renderable parts.

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::core::movement::components::position::Position;
use crate::frontend::display::constants::layout::LAYER_ACTOR;
use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::map::utils::coords::cell_to_world;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

/// Texel-grid discipline for creature frames: a frame with an odd
/// dimension, centered on an integer position, lands its texel edges
/// exactly on the canvas's pixel centers — every row/column on that
/// axis becomes a boundary lottery (rows duplicate or drop). Odd axes
/// get a half-texel nudge; y nudges down so the feet land flush on the
/// cell's bottom edge.
fn texel_aligned_anchor(frame_size: UVec2) -> Anchor {
    let size = frame_size.as_vec2();
    let center_shift = Vec2::new(
        if frame_size.x % 2 == 1 { 0.5 } else { 0.0 },
        if frame_size.y % 2 == 1 { -0.5 } else { 0.0 },
    );
    // Sprite centers sit at `transform − anchor × size`.
    Anchor(-center_shift / size)
}

/// Attach sprite, transform and playback state to every entity
/// presenting a new figure, cloning the registry's handles onto the
/// instance — the renderer reads them off the entity, resolve-once
/// style. The anim table stays in the registry and is looked up per
/// frame. The initial transform carries the actor z layer; x/y are
/// overwritten by `sync_position` every frame from then on.
pub fn attach_appearance(
    mut commands: Commands,
    figure_registry: Res<FigureRegistry>,
    query: Query<(Entity, &FigureIndex, &Position), Added<FigureIndex>>,
) {
    for (entity, figure_index, pos) in &query {
        let appearance = figure_registry.appearance(*figure_index);
        let world = cell_to_world(*pos);
        // The phase offset derives from the standing cell: a deterministic
        // desync for crowds spawned on the same tick (zero would sync
        // them).
        let idle_len = appearance.anim(AnimKind::Idle).frames.len();
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
            texel_aligned_anchor(appearance.frame_size),
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

    /// The sprite center shift implied by an anchor.
    fn center_shift(frame_size: UVec2, anchor: Anchor) -> Vec2 {
        -anchor.0 * frame_size.as_vec2()
    }

    #[test]
    fn even_frames_stay_centered() {
        let shift = center_shift(UVec2::new(16, 16), texel_aligned_anchor(UVec2::new(16, 16)));
        assert_eq!(shift, Vec2::ZERO);
    }

    #[test]
    fn odd_height_nudges_down_half_a_texel() {
        // The warrior frame is 12x15.
        let shift = center_shift(UVec2::new(12, 15), texel_aligned_anchor(UVec2::new(12, 15)));
        assert!((shift - Vec2::new(0.0, -0.5)).length() < 1e-6);
    }

    #[test]
    fn odd_width_nudges_half_a_texel() {
        let shift = center_shift(UVec2::new(13, 16), texel_aligned_anchor(UVec2::new(13, 16)));
        assert!((shift - Vec2::new(0.5, 0.0)).length() < 1e-6);
    }

    /// Spec-alignment: a new figure handle mounts the sprite, the actor
    /// layer, and the idle playback state.
    #[test]
    fn new_figure_handle_mounts_sprite_and_playback_state() {
        use bevy::ecs::system::RunSystemOnce;

        use crate::core::map::types::cell_coord::CellCoord;
        use crate::core::movement::components::position::Position;
        use crate::frontend::display::constants::layout::LAYER_ACTOR;
        use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
        use crate::frontend::display::sprite_animation::components::anim_state::AnimState;

        let mut world = World::new();
        world.insert_resource(FigureRegistry::for_test(&["warrior"]));
        let entity = world
            .spawn((
                FigureIndex::from_index(0),
                Position::from(CellCoord::new(2, 3)),
            ))
            .id();
        world.run_system_once(attach_appearance).unwrap();

        let sprite = world.get::<Sprite>(entity).expect("sprite mounted");
        assert!(sprite.texture_atlas.is_some());
        let transform = world.get::<Transform>(entity).expect("transform mounted");
        assert_eq!(transform.translation.z, LAYER_ACTOR);
        let state = world.get::<AnimState>(entity).expect("playback mounted");
        assert_eq!(state.anim, AnimKind::Idle);
    }
}
