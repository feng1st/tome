// TODO: pending cleanup review — remove once stabilized
//! Gesture: primary action on a cell.

use bevy::prelude::*;

use crate::core::map::types::cell_coord::CellCoord;

/// The user committed the primary action on a map cell — the pointer hit
/// terrain with nothing hittable on top of it. Which map the cell belongs
/// to (world map vs. local map) is game state the resolver reads, not part
/// of the gesture. Mouse click, keyboard cursor + confirm key, and touch
/// tap all map to this gesture.
#[derive(Message, Clone, Copy, Debug)]
pub struct PrimaryActionOnCell(pub CellCoord);
