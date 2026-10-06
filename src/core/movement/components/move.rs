//! Move: the transient step action. Carries its target so the executor
//! needs no knowledge of the actor's species or intent.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// One step to an adjacent cell, created in Plan and executed in Act of
/// the same frame (via `Added<Move>`), then removed. Shared by every
/// species: the world driver steps along its ordered route, monsters
/// step into their rolled direction.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub to: CellCoord,
}
