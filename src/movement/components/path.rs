use std::collections::VecDeque;

use bevy::prelude::*;

use crate::map::resources::grid_map::GridMap;
use crate::movement::utils::step_duration::step_duration;

/// A path an entity is walking along, one tile step at a time.
/// Generic: works for the hero and, later, for mobs.
///
/// Invariants: `cells.front()` is the current step's target cell;
/// `step_from`/`step_target` are world-space centers of the previous and
/// target cell; the step completes when `step_t >= step_dur`.
#[derive(Component)]
pub struct Path {
    pub cells: VecDeque<IVec2>,
    pub step_from: Vec2,
    /// World-space center of the current step's target cell.
    pub step_target: Vec2,
    pub step_t: f32,
    pub step_dur: f32,
}

impl Path {
    /// `cells` excludes the start cell (the entity is already there).
    pub fn new(cells: VecDeque<IVec2>, from: Vec2) -> Self {
        let next = *cells.front().expect("non-empty path");
        Path {
            cells,
            step_from: from,
            step_target: GridMap::cell_center(next),
            step_t: 0.0,
            step_dur: step_duration(GridMap::world_to_cell(from), next),
        }
    }
}
