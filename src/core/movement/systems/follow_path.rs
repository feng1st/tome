//! Advances every entity with a `Path` along its steps, in cell space.
//! Knows nothing about heroes, mobs, or pixels: the authoritative
//! `Position` is all that moves here.

use bevy::prelude::*;

use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;
use crate::core::movement::utils::step_duration::step_duration;

/// Movement model: tile-by-tile tween in continuous cell coordinates.
/// `step_elapsed` accumulates frame time; when it reaches `step_duration`
/// the entity snaps to the target cell center and the next step begins.
/// Overshoot time carries over, so a long frame never eats a step. When the
/// path runs out the entity stands still at an integer cell until a new
/// `Path` arrives.
pub fn follow_path(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Position, &mut Path)>,
) {
    for (entity, mut pos, mut path) in &mut query {
        path.step_elapsed += time.delta_secs();
        loop {
            let Some(&target) = path.cells.front() else {
                commands.entity(entity).remove::<Path>();
                break;
            };
            if path.step_elapsed < path.step_duration {
                // Mid-step: interpolate between cell centers.
                let t = path.step_elapsed / path.step_duration;
                *pos = path.step_from.lerp(path.step_to, t);
                break;
            }
            // Step finished: snap to the cell center and start the next step.
            path.step_elapsed -= path.step_duration;
            path.step_from = path.step_to;
            path.cells.pop_front();
            *pos = path.step_to;
            if let Some(&next) = path.cells.front() {
                path.step_duration = step_duration(target, next);
                path.step_to = Position::from(next);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::time::Duration;

    use super::*;
    use crate::core::map::types::cell_coord::CellCoord;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .add_systems(Update, follow_path);
        app
    }

    /// Walker at (0,0) with a two-cell straight path: (1,0), (2,0).
    fn walker(app: &mut App) -> Entity {
        let from = Position::from(CellCoord::new(0, 0));
        let cells = VecDeque::from([CellCoord::new(1, 0), CellCoord::new(2, 0)]);
        app.world_mut().spawn((from, Path::new(cells, from))).id()
    }

    fn advance(app: &mut App, secs: f32) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(secs));
        app.update();
    }

    fn assert_pos_near(app: &App, entity: Entity, x: f32, y: f32) {
        let pos = app.world().get::<Position>(entity).unwrap();
        assert!(
            (pos.x - x).abs() < 1e-6 && (pos.y - y).abs() < 1e-6,
            "{pos:?} vs ({x}, {y})"
        );
    }

    #[test]
    fn mid_step_interpolates_between_cell_centers() {
        let mut app = app();
        let walker = walker(&mut app);
        advance(&mut app, 0.075); // half of the 0.15s straight step
        assert_pos_near(&app, walker, 0.5, 0.0);
    }

    #[test]
    fn overshoot_carries_into_the_next_step() {
        let mut app = app();
        let walker = walker(&mut app);
        advance(&mut app, 0.225); // one full step plus half of the next
        assert_pos_near(&app, walker, 1.5, 0.0);
    }

    #[test]
    fn completed_path_is_removed_and_stands_at_the_goal() {
        let mut app = app();
        let walker = walker(&mut app);
        advance(&mut app, 0.3); // both steps
        assert_pos_near(&app, walker, 2.0, 0.0);
        assert!(app.world().get::<Path>(walker).is_none());
    }
}
