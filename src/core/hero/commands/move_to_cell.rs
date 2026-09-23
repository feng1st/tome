//! Command messages: the cross-side protocol. The frontend interprets raw
//! input into these concrete commands; the core validates and executes
//! them. Commands are the only channel through which the frontend may ask
//! the core for changes.

use bevy::prelude::*;

use crate::core::map::cell_pos::CellPos;

/// Command: walk the hero to a cell. The core validates walkability and
/// pathfinds; invalid targets are ignored.
#[derive(Message, Clone, Copy, Debug)]
pub struct MoveToCell(pub CellPos);
