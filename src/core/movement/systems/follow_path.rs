//! Advances every entity with a `Path` along its steps, in cell space.
//! Knows nothing about heroes, mobs, or pixels: the authoritative
//! `Position` is all that moves here.

use bevy::prelude::*;

use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;
use crate::core::movement::utils::step_duration::step_duration;

/// Movement model: tile-by-tile tween in continuous cell coordinates.
/// `step_t` accumulates frame time; when it reaches `step_dur` the entity
/// snaps to the target cell center and the next step begins. Overshoot time
/// carries over, so a long frame never eats a step. When the path runs out
/// the entity stands still at an integer cell until a new `Path` arrives.
pub fn follow_path(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Position, &mut Path)>,
) {
    for (entity, mut pos, mut path) in &mut query {
        path.step_t += time.delta_secs();
        loop {
            let Some(&target) = path.cells.front() else {
                commands.entity(entity).remove::<Path>();
                break;
            };
            if path.step_t < path.step_dur {
                // Mid-step: interpolate between cell centers.
                let t = path.step_t / path.step_dur;
                pos.0 = path.step_from.lerp(path.step_target, t);
                break;
            }
            // Step finished: snap to the cell center and start the next step.
            path.step_t -= path.step_dur;
            path.step_from = path.step_target;
            path.cells.pop_front();
            pos.0 = path.step_target;
            if let Some(&next) = path.cells.front() {
                path.step_dur = step_duration(target, next);
                path.step_target = next.as_vec2();
            }
        }
    }
}
