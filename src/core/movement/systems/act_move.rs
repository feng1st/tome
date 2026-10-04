//! Act move: execute every step action created this frame.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::movement::components::path::Path;
use crate::core::movement::components::r#move::Move;

/// Every entity with a freshly added `Move` sets its cell coordinate to
/// the target at once — data first, the display chases. A path follower
/// also pops the consumed cell; an emptied path is removed (the world
/// driver going pathless is what parks the world). The action component
/// is consumed so the next step registers as `Added` again.
pub fn act_move(
    mut commands: Commands,
    mut actors: Query<(Entity, &Move, &mut CellCoord, Option<&mut Path>), Added<Move>>,
) {
    for (entity, target, mut cell, path) in &mut actors {
        *cell = target.to;
        if let Some(mut path) = path {
            // Invariant: planning consumed the path's head, and the
            // Command → plan → act phase chain keeps it untouched in
            // between.
            debug_assert_eq!(path.cells.front(), Some(&target.to));
            path.cells.pop_front();
            if path.cells.is_empty() {
                commands.entity(entity).remove::<Path>();
            }
        }
        commands.entity(entity).remove::<Move>();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use bevy::ecs::system::RunSystemOnce;

    use super::*;

    #[test]
    fn move_sets_the_cell_and_is_consumed() {
        let mut world = World::new();
        let rat = world
            .spawn((
                CellCoord::new(1, 1),
                Move {
                    to: CellCoord::new(2, 1),
                },
            ))
            .id();
        world.run_system_once(act_move).unwrap();
        assert_eq!(world.get::<CellCoord>(rat).unwrap(), &CellCoord::new(2, 1));
        assert!(world.get::<Move>(rat).is_none());
    }

    #[test]
    fn path_follower_pops_and_empties() {
        let mut world = World::new();
        let player = world
            .spawn((
                CellCoord::new(1, 1),
                Path::new(VecDeque::from([CellCoord::new(2, 1)])),
                Move {
                    to: CellCoord::new(2, 1),
                },
            ))
            .id();
        world.run_system_once(act_move).unwrap();
        assert_eq!(
            world.get::<CellCoord>(player).unwrap(),
            &CellCoord::new(2, 1)
        );
        assert!(world.get::<Path>(player).is_none());
        assert!(world.get::<Move>(player).is_none());
    }

    #[test]
    fn path_keeps_the_remaining_cells() {
        let mut world = World::new();
        let player = world
            .spawn((
                CellCoord::new(1, 1),
                Path::new(VecDeque::from([CellCoord::new(2, 1), CellCoord::new(3, 1)])),
                Move {
                    to: CellCoord::new(2, 1),
                },
            ))
            .id();
        world.run_system_once(act_move).unwrap();
        let path = world.get::<Path>(player).unwrap();
        assert_eq!(path.cells.len(), 1);
        assert_eq!(path.cells.front(), Some(&CellCoord::new(3, 1)));
    }
}
