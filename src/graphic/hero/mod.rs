//! The hero domain's display side: sprite sheet, frame tables, and
//! attaching appearance to heroes spawned by the core.

pub mod constants;
pub mod systems;

use bevy::prelude::*;

/// Register hero display systems. Runs before `sync_position` so a freshly
/// spawned hero has its transform written in the same frame.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::attach_appearance::attach_appearance
            .before(crate::graphic::sync::systems::sync_position::sync_position),
    );
}
