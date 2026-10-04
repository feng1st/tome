//! CurrPosition: a picture's current position in continuous cell
//! coordinates. Display-side only — game logic reads the logical cell,
//! never this.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// A picture's current position in continuous cell coordinates (1 unit
/// = 1 cell). Written only by the motion domain (the move toward the
/// logical cell); read by the snap system. Integer values are cell
/// centers.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct CurrPosition {
    pub x: f32,
    pub y: f32,
}

impl From<CellCoord> for CurrPosition {
    fn from(cell: CellCoord) -> Self {
        CurrPosition {
            x: cell.x as f32,
            y: cell.y as f32,
        }
    }
}
