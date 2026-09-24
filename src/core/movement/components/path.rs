//! A path an entity is walking along, one tile step at a time.
//! Generic: works for the hero and, later, for mobs.

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::core::map::types::cell_coord::CellCoord;
use crate::core::movement::components::position::Position;
use crate::core::movement::utils::step_duration::step_duration;

/// Invariants: `cells.front()` is the current step's target cell;
/// `step_from`/`step_target` are the current step's endpoints; the step
/// completes when `step_t >= step_dur`.
#[derive(Component)]
pub struct Path {
    /// Remaining cells to walk, excluding the cell the entity stands in.
    pub cells: VecDeque<CellCoord>,
    /// Position the current step started from.
    pub step_from: Position,
    /// Position the current step ends at (the target cell's center).
    pub step_target: Position,
    /// Seconds accumulated in the current step.
    pub step_t: f32,
    /// Duration of the current step (straight vs diagonal, see
    /// `step_duration`).
    pub step_dur: f32,
}

impl Path {
    /// `cells` excludes the start cell (the entity is already there);
    /// `from` is the entity's current position.
    pub fn new(cells: VecDeque<CellCoord>, from: Position) -> Self {
        let next = *cells.front().expect("non-empty path");
        Path {
            cells,
            step_from: from,
            step_target: Position::from(next),
            step_t: 0.0,
            step_dur: step_duration(from.cell(), next),
        }
    }
}
