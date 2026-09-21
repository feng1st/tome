//! The graphic plugin: sprite/tileset-based display of the core's game
//! state. Replaceable as a whole — e.g. with a text display — without
//! touching the core.

pub mod animation;
pub mod camera;
pub mod hero;
pub mod map;
pub mod sync;

use bevy::prelude::*;

/// Register all display-side domains. The per-frame chain (sync_position,
/// animate, follow_target) is assembled in main.rs.
pub fn register(app: &mut App) {
    map::register(app);
    camera::register(app);
    hero::register(app);
}
