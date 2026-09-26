//! A path an entity is walking along, one tile step at a time.
//! Generic: works for the hero and, later, for mobs.

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::core::map::types::cell_coord::CellCoord;
use crate::core::movement::components::position::Position;
use crate::core::movement::utils::step_duration::step_duration;

/// Invariants: `cells.front()` is the current step's target cell;
/// `step_from`/`step_to` are the current step's endpoints; the step
/// completes when `step_elapsed >= step_duration`.
#[derive(Component)]
pub struct Path {
    /// Remaining cells to walk, excluding the cell the entity stands in.
    pub cells: VecDeque<CellCoord>,
    /// Position the current step started from.
    pub step_from: Position,
    /// Position the current step ends at (the target cell's center).
    pub step_to: Position,
    /// Seconds accumulated in the current step.
    pub step_elapsed: f32,
    /// Duration of the current step (straight vs diagonal, see
    /// `step_duration`).
    pub step_duration: f32,
}

impl Path {
    /// `cells` excludes the start cell (the entity is already there);
    /// `from` is the entity's current position.
    pub fn new(cells: VecDeque<CellCoord>, from: Position) -> Self {
        let next = *cells.front().expect("non-empty path");
        Path {
            cells,
            step_from: from,
            step_to: Position::from(next),
            step_elapsed: 0.0,
            step_duration: step_duration(from.cell(), next),
        }
    }
}
