use bevy::prelude::*;

use crate::map::resources::grid_map::GridMap;
use crate::movement::components::path::Path;
use crate::movement::utils::step_duration::step_duration;

/// Advance every entity with a `Path` along its steps, preserving its z.
/// Knows nothing about heroes or mobs.
pub fn follow_path(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Path)>,
) {
    for (entity, mut transform, mut path) in &mut query {
        path.step_t += time.delta_secs();
        let z = transform.translation.z;
        loop {
            let Some(&target) = path.cells.front() else {
                commands.entity(entity).remove::<Path>();
                break;
            };
            let target_center = GridMap::cell_center(target);
            if path.step_t < path.step_dur {
                let t = path.step_t / path.step_dur;
                transform.translation = path.step_from.lerp(target_center, t).extend(z);
                break;
            }
            // Step finished: snap to the tile center and start the next step.
            path.step_t -= path.step_dur;
            path.step_from = target_center;
            path.cells.pop_front();
            transform.translation = target_center.extend(z);
            if let Some(&next) = path.cells.front() {
                path.step_dur = step_duration(target, next);
                path.step_target = GridMap::cell_center(next);
            }
        }
    }
}
