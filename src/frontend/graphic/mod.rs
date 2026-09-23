//! World rendering: sprite/tileset-based display of the core's game state,
//! plus the graphic-side frame chain. Presentation only.

pub mod animation;
pub mod camera;
pub mod hero;
pub mod map;
pub mod sync;

use bevy::prelude::*;

use crate::core::system_sets::RenderSet;

/// Register all rendering domains plus the graphic-side frame logic,
/// ordered inside `RenderSet`: appearance first so a freshly spawned hero
/// has a transform, then positions sync, then animation and camera read
/// the results.
pub fn register(app: &mut App) {
    map::register(app);
    camera::register(app);
    app.add_systems(
        Update,
        (
            hero::systems::attach_appearance::attach_appearance,
            sync::systems::sync_position::sync_position,
            animation::systems::animate::animate,
            camera::systems::follow_target::follow_target,
        )
            .chain()
            .in_set(RenderSet),
    );
}
