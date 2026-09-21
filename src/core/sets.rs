//! Schedule set labels: the cross-side ordering contract. The core owns
//! these labels as protocol — plugins place their systems into them and
//! order their own internals; the assembly layer chains the labels without
//! naming any concrete system.

use bevy::prelude::*;

/// Raw input is translated into core gestures.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

/// Core game logic: gesture dispatch, movement, world state.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoreSet;

/// Presentation derived from core state.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphicSet;
