use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

#[derive(Message, Debug)]
pub struct TargetCell(pub CellCoord);
