// TODO: pending cleanup review — remove once stabilized
//! Position in continuous cell coordinates — the movement vocabulary type.

use bevy::prelude::*;

use crate::core::map::types::cell_coord::CellCoord;

/// A position in continuous cell coordinates (1 unit = 1 cell; `x` =
/// column, `y` = row, row increases downward, matching the map array).
/// Integer coordinates are cell centers; fractional values occur only
/// mid-step — the single source of movement smoothness.
///
/// As a component, this is an entity's authoritative position: a standing
/// entity always sits at integer coordinates. The frontend quantizes it
/// (pixels for graphics, cells for text); the quantized result is never
/// written back.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }

    /// Cell the position stands in (integer coordinates are cell centers,
    /// so rounding identifies the standing cell).
    pub fn cell(&self) -> CellCoord {
        CellCoord::new(self.x.round() as i32, self.y.round() as i32)
    }

    /// Linear interpolation between two positions.
    pub fn lerp(self, to: Self, t: f32) -> Self {
        Position {
            x: self.x + (to.x - self.x) * t,
            y: self.y + (to.y - self.y) * t,
        }
    }
}

/// A cell converts to its center position (integer coordinates are cell
/// centers by convention).
impl From<CellCoord> for Position {
    fn from(cell: CellCoord) -> Self {
        Position::new(cell.x as f32, cell.y as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_rounds_to_center_convention() {
        // Integer coordinates are cell centers: cell i spans [i-0.5, i+0.5).
        assert_eq!(Position::new(32.0, 10.0).cell(), CellCoord::new(32, 10));
        assert_eq!(Position::new(32.4, 10.0).cell(), CellCoord::new(32, 10));
        assert_eq!(Position::new(32.6, 10.0).cell(), CellCoord::new(33, 10));
        // Exactly at the half-step boundary, round goes to the next cell.
        assert_eq!(Position::new(32.5, 10.0).cell(), CellCoord::new(33, 10));
    }
}
