//! Cell coordinates: the map grid's vocabulary type, and a creature's
//! logical position when used as a component.

use bevy::prelude::*;
use serde::Deserialize;

/// Integer coordinates of a cell: column `x`, row `y`, row 0 at the top.
///
/// Two roles, one type. As a plain value it is the grid vocabulary of map
/// data (spawn entries, …). As a component it is the creature's logical
/// position: always an integer cell, the only position game logic reads;
/// the continuous presentation position lives on the display side.
///
/// Deliberately a vocabulary type, not a bare `IVec2`: signatures that take
/// cells say so, and cell coordinates cannot be confused with pixel offsets
/// or other integer vectors. Named `CellCoord` rather than `Cell` — `Cell`
/// may later denote a cell entity with contents.
///
/// `Deserialize` serves map file layouts (spawn entries, …).
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize)]
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
