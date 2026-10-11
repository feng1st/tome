use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

#[derive(Message, Debug)]
#[allow(dead_code)]
pub struct Attacked {
    pub attacker: Entity,
    pub attacker_cell_coord: CellCoord,
    pub target: Entity,
    pub target_cell_coord: CellCoord,
}
