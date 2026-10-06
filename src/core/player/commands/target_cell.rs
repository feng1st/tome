//! Command messages: the cross-side protocol. The frontend interprets raw
//! input into these concrete commands; the core validates and executes
//! them. Commands are the only channel through which the frontend may ask
//! the core for changes.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// Command: the player targeted a cell. The core decides what the target
/// means — a strike when a living monster stands there, a walk otherwise
/// — and ignores invalid cells.
#[derive(Message, Clone, Copy, Debug)]
pub struct TargetCell(pub CellCoord);
