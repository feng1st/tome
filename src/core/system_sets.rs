//! Frame-phase set labels: the cross-side ordering contract. The core owns
//! these labels as protocol — plugins place their systems into them and
//! order their own internals; the assembly layer chains the labels without
//! naming any concrete system. Labels are frame phases, not owned by any
//! single side: the frontend places modality systems into `InputSet` (frame
//! start) and rendering systems into `RenderSet` (frame end).

use bevy::prelude::*;

/// Raw input is translated into core gestures.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

/// Core game logic: gesture dispatch, movement, world state.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSet;

/// Presentation derived from core state.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderSet;
