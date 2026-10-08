//! The damaged fact.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// A wound landed: the target, the cells, the amount, and the maximum
/// the amount is weighed against. Emitted once per applied damage
/// request. Everything a reader needs travels inside the fact: by the
/// time a reader runs, the target may have left the world (the killing
/// blow included), so the cells and the maximum are captured here, not
/// looked up. `source_cell` reads as `None` when the wound names no
/// source or the source left the world before application.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Damaged {
    pub target: Entity,
    pub cell: CellCoord,
    pub source_cell: Option<CellCoord>,
    pub amount: i32,
    pub max: i32,
}
