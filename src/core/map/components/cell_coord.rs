use bevy::prelude::*;
use serde::Deserialize;

use crate::core::common::by_value::ByValue;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize)]
pub struct CellCoord {
    pub x: i32,
    pub y: i32,
}

impl ByValue for CellCoord {}

impl CellCoord {
    pub const fn new(x: i32, y: i32) -> Self {
        CellCoord { x, y }
    }
}

impl std::ops::Add<IVec2> for CellCoord {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self {
        CellCoord::new(self.x + rhs.x, self.y + rhs.y)
    }
}
