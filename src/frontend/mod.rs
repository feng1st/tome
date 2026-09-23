//! The frontend: everything player-facing — world rendering, input
//! modalities, and later the HUD. Replaceable as a whole (e.g. with a text
//! frontend) without touching the core; replaceable units inside it
//! (tilesets, animations, HUD, modalities) are directories, not plugins.

pub mod graphic;
pub mod input;

use bevy::prelude::*;

/// Register the frontend's sub-areas. Systems land in the frame-phase sets
/// owned by the core (`InputSet` at frame start, `RenderSet` at frame end);
/// the assembly layer only chains the labels.
pub fn register(app: &mut App) {
    graphic::register(app);
    input::register(app);
}
