//! A path an entity is walking along, one tile step at a time.
//! Generic: works for the hero and, later, for mobs.

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::core::movement::utils::step_duration::step_duration;

/// Invariants: `cells.front()` is the current step's target cell;
/// `step_from`/`step_target` are cell-space centers (integer coordinates)
/// of the previous and target cell; the step completes when
/// `step_t >= step_dur`.
#[derive(Component)]
pub struct Path {
    pub cells: VecDeque<IVec2>,
    /// Cell-space position the current step started from.
    pub step_from: Vec2,
    /// Cell-space center of the current step's target cell.
    pub step_target: Vec2,
    pub step_t: f32,
    pub step_dur: f32,
}

impl Path {
    /// `cells` excludes the start cell (the entity is already there);
    /// `from` is the entity's current cell-space position.
    pub fn new(cells: VecDeque<IVec2>, from: Vec2) -> Self {
        let next = *cells.front().expect("non-empty path");
        Path {
            cells,
            step_from: from,
            step_target: next.as_vec2(),
            step_t: 0.0,
            step_dur: step_duration(from.round().as_ivec2(), next),
        }
    }
}
