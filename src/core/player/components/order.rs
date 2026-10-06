//! The player's standing order: the one intent the turn loop serves.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// The player's standing order — at most one, in exactly one form, and
/// inserting a variant replaces whatever stood before. An order persists
/// across turns until it clears: a move order on arrival, no route, or
/// a final step onto an occupied cell; an attack order on its strike
/// (one order, one strike), no route, or the target's death; any order
/// when damage lands.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum Order {
    /// Reach one cell: planning re-paths toward it each due turn.
    Move { target: CellCoord },
    /// Strike one monster: planning pursues the target's current cell
    /// each due turn.
    Attack { target: Entity },
}
