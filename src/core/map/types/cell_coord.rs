// TODO: pending cleanup review — remove once stabilized
//! Cell coordinates: the map grid's vocabulary type.

use bevy::prelude::*;

/// Integer coordinates of a cell: column `x`, row `y`, row 0 at the top.
///
/// Deliberately a vocabulary type, not a bare `IVec2`: signatures that take
/// cells say so, and cell coordinates cannot be confused with pixel offsets
/// or other integer vectors. Named `CellCoord` rather than `Cell` — `Cell`
/// may later denote a cell entity with contents — and distinct from
/// `Position`, the continuous cell-space position.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CellCoord {
    pub x: i32,
    pub y: i32,
}

impl CellCoord {
    pub const fn new(x: i32, y: i32) -> Self {
        CellCoord { x, y }
    }
}

/// Coordinate plus a plain displacement vector stays a coordinate — used
/// for neighbor steps (`cell + DIRECTION`).
impl std::ops::Add<IVec2> for CellCoord {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self {
        CellCoord::new(self.x + rhs.x, self.y + rhs.y)
    }
}
