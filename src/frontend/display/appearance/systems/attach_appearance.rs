//! Attaches a creature's appearance. The core spawns creatures as pure
//! game data (kind markers + a `FigureIndex` handle + `Position`); this
//! system resolves the handle through the appearance registry and
//! attaches the renderable parts.

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::core::figure::components::figure_index::FigureIndex;
use crate::core::movement::components::position::Position;
use crate::frontend::display::appearance::resources::appearance_registry::AppearanceRegistry;
use crate::frontend::display::constants::layout::LAYER_ACTOR;
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
}
