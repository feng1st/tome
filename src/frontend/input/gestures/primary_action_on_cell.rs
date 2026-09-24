//! Gesture: primary action on a map cell.

use bevy::prelude::*;

use crate::core::map::types::cell_coord::CellCoord;

/// The user committed the primary action on a map cell — the pointer hit
/// terrain with nothing hittable on top of it. Mouse click, keyboard
/// cursor + confirm key, and touch tap all map to this gesture.
#[derive(Message, Clone, Copy, Debug)]
pub struct PrimaryActionOnCell(pub CellCoord);
