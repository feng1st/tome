use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

#[derive(Component, Clone, Copy, Debug)]
pub enum Order {
    Move { target: CellCoord },
    Attack { target: Entity },
}
