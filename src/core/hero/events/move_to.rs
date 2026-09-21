//! Intent events emitted by input plugins and consumed by the core.

use bevy::prelude::*;

/// Intent to move the hero to a target cell. Input plugins translate raw
/// input (mouse, touch, …) into this display-independent form; the core
/// validates walkability and pathfinds. Carrying a cell rather than screen
/// or world coordinates keeps display concepts out of the protocol.
#[derive(Message, Clone, Copy, Debug)]
pub struct MoveTo(pub IVec2);
