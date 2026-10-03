//! Presentation position: an entity's on-screen position in continuous
//! cell coordinates. Display-side only — game logic reads the logical
//! cell, never this.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// The on-screen position in continuous cell coordinates (1 unit = 1
/// cell). Written only by the motion domain (the world driver's step tween,
/// or the chase for everyone else); read by the snap system. Integer values
/// are cell centers.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PresentationPosition {
    pub x: f32,
    pub y: f32,
}

impl From<CellCoord> for PresentationPosition {
    fn from(cell: CellCoord) -> Self {
        PresentationPosition {
            x: cell.x as f32,
            y: cell.y as f32,
        }
    }
}
