//! Cell position: the map grid's vocabulary type.

use bevy::prelude::*;

/// A position on the map grid: column x, row y, row 0 at the top. Integer
/// coordinates; the continuous form lives in `Position` (movement domain).
///
/// Deliberately a vocabulary type, not a bare `IVec2`: signatures that take
/// cells say so, and cell coordinates cannot be confused with pixel offsets
/// or other integer vectors. Named `CellPos` rather than `Cell` — `Cell`
/// may later denote a cell entity with contents.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CellPos(pub IVec2);

impl CellPos {
    pub const fn new(x: i32, y: i32) -> Self {
        CellPos(IVec2::new(x, y))
    }

    /// Continuous cell-space coordinates of this cell's center (integer
    /// coordinates are cell centers by convention).
    pub fn as_vec2(self) -> Vec2 {
        self.0.as_vec2()
    }
}

impl From<IVec2> for CellPos {
    fn from(v: IVec2) -> Self {
        CellPos(v)
    }
}
