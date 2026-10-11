use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

#[derive(Message, Debug)]
#[allow(dead_code)]
pub struct Damaged {
    pub source: Option<Entity>,
    pub source_cell_coord: Option<CellCoord>,
    pub target: Entity,
    pub target_cell_coord: CellCoord,
    pub amount: i32,
    pub max_hp: i32,
}
