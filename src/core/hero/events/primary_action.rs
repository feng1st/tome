//! Input gestures: modality-independent requests from input plugins.

use bevy::prelude::*;

/// The user committed the primary action on a cell. Mouse click, keyboard
/// cursor + confirm key, and touch tap all map to this one gesture; what
/// the action means (move, pick up, attack, …) is decided by the core from
/// game state — input plugins neither know nor decide.
#[derive(Message, Clone, Copy, Debug)]
pub struct PrimaryAction(pub IVec2);
