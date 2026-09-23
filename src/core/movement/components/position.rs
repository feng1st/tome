//! Authoritative entity position in continuous cell coordinates.

use bevy::prelude::*;

use crate::core::map::cell_pos::CellPos;

/// Authoritative position of an entity in continuous cell coordinates
/// (1 unit = 1 cell; x = column, y = row, row increases downward, matching
/// the map array). Integer coordinates are cell centers: a standing entity
/// always sits at integer coordinates, and fractional values occur only
/// mid-step — the single source of movement smoothness.
///
/// Display plugins quantize this value (pixels for graphics, cells for
/// text); the quantized result is never written back here.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Position(pub Vec2);

impl Position {
    /// Cell the entity currently stands in (integer coordinates are cell
    /// centers, so rounding identifies the standing cell).
    pub fn cell(&self) -> CellPos {
        CellPos(self.0.round().as_ivec2())
    }
}
