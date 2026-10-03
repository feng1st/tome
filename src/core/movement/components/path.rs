//! A path an entity is walking along, one tile step at a time. Generic:
//! works for the hero and, later, for mobs.

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// A queued path computed once at command time and consumed one cell per
/// action. Recomputed only when the goal changes or a future invalidation
/// event (a blocker appearing, an interrupt) demands it.
#[derive(Component)]
pub struct Path {
    /// Remaining cells to walk, excluding the cell the entity stands in.
    pub cells: VecDeque<CellCoord>,
}

impl Path {
    /// `cells` excludes the start cell (the entity is already there).
    pub fn new(cells: VecDeque<CellCoord>) -> Self {
        assert!(!cells.is_empty(), "non-empty path");
        Path { cells }
    }
}
