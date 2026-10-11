//! CurrPosition: a picture's current position in continuous cell
//! coordinates. Display-side only — game logic reads the logical cell,
//! never this.

use bevy::prelude::*;

use crate::core::common::by_value::ByValue;
use crate::core::map::components::cell_coord::CellCoord;

/// A picture's current position in continuous cell coordinates (1 unit
/// = 1 cell). Written only by the motion domain (the move toward the
/// logical cell); read by the snap system. Integer values are cell
/// centers.
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct CellPosition {
    pub x: f32,
    pub y: f32,
}

impl ByValue for CellPosition {}

impl From<CellCoord> for CellPosition {
    fn from(cell_coord: CellCoord) -> Self {
        CellPosition {
            x: cell_coord.x as f32,
            y: cell_coord.y as f32,
        }
    }
}
