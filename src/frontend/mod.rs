// TODO: pending cleanup review — remove once stabilized
//! The frontend: everything player-facing — world rendering, input
//! modalities, and later the HUD. Replaceable as a whole (e.g. with a text
//! frontend) without touching the core; replaceable units inside it
//! (tilesets, animations, HUD, modalities) are directories, not plugins.

pub mod display;
pub mod input;

use bevy::prelude::*;

/// Register the frontend's sub-areas. Systems land in the game-loop stages
/// owned by the core (`GameLoop::Input` at frame start, `GameLoop::Display` at frame end);
/// the assembly layer only chains the labels.
pub fn register(app: &mut App) {
    display::register(app);
    input::register(app);
}
