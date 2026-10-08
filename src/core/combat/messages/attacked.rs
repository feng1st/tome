//! The attacked fact.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// An attack happened: the attacker swung at the target's cell, and the
/// listed blows landed — each entry a landed blow's damage amount, in
/// blow order; an empty list means every blow missed (or the attacker
/// carried no blows to swing with). Emitted once per consumed attack
/// action, hit or miss: the swing itself is the fact. The cells travel
/// inside the fact so a reader needs no entity state — by the time a
/// reader runs, either creature may have left the world.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct Attacked {
    pub attacker: Entity,
    pub attacker_cell_coord: CellCoord,
    pub target_cell_coord: CellCoord,
    pub damage_amounts: Vec<i32>,
}
