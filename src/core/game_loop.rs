// TODO: pending cleanup review — remove once stabilized
//! Game-loop set labels: the cross-side ordering contract. The core owns
//! these labels as protocol — plugins place their systems into them and
//! order their own internals; the assembly layer chains the labels without
//! naming any concrete system. The three stages mirror the three sides:
//! input, core, display. Naming follows the classic game-loop pattern
//! (input → update → present), adapted to the side names.
//!
//! Note: actual GPU rendering runs in Bevy's separate render sub-app, not
//! in the main schedule — `Display` here is presentation *preparation* in
//! the main world (likewise, `Present` would name the engine's swapchain
//! submission, so it is avoided too).

use bevy::prelude::*;

/// One iteration of the game loop, chained by the assembly layer.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameLoop {
    /// Core commands are produced from gestures.
    Input,
    /// Core game logic: command execution, movement, world state.
    Core,
    /// Presentation derived from core state.
    Display,
}
