//! Frame-phase set labels: the cross-side ordering contract. The core owns
//! these labels as protocol — plugins place their systems into them and
//! order their own internals; the assembly layer chains the labels without
//! naming any concrete system. Labels are frame phases, not owned by any
//! single side: the frontend places translation systems into `Input` (frame
//! start) and rendering systems into `Render` (frame end).

use bevy::prelude::*;

/// Cross-side frame phases, chained by the assembly layer.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FramePhase {
    /// Raw input is translated into core commands (gesture resolution).
    Input,
    /// Core game logic: command execution, movement, world state.
    Game,
    /// Presentation derived from core state.
    Render,
}
