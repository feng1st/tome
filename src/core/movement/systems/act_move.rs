//! Act move: execute every step action created this frame.

use bevy::prelude::*;

use crate::core::health::components::dead::Dead;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::movement::components::r#move::Move;

/// Every living entity with a freshly added `Move` sets its cell
/// coordinate to the target at once — data first, the display chases.
/// The action component is consumed so the next step registers as
/// `Added` again. The dead never move: a lingering death presentation
/// must not walk a corpse, whatever left a move pending on it.
/// The actor filter: fresh steps of the living only — the dead
/// never move.
type ActorFilter = (Added<Move>, Without<Dead>);

pub fn act_move(
    mut commands: Commands,
    mut actors: Query<(Entity, &Move, &mut CellCoord), ActorFilter>,
) {
    for (entity, target, mut cell) in &mut actors {
        *cell = target.to;
        commands.entity(entity).remove::<Move>();
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::core::player::components::order::Order;

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
    fn the_order_survives_the_step() {
        // The standing order is planning's business: execution touches
        // the action and the cell only.
        let mut world = World::new();
        let player = world
            .spawn((
                CellCoord::new(1, 1),
                Move {
                    to: CellCoord::new(2, 1),
                },
                Order::Move {
                    target: CellCoord::new(3, 1),
                },
            ))
            .id();
        world.run_system_once(act_move).unwrap();
        assert_eq!(
            world.get::<CellCoord>(player).unwrap(),
            &CellCoord::new(2, 1)
        );
        assert!(world.get::<Move>(player).is_none());
        assert_eq!(
            world.get::<Order>(player),
            Some(&Order::Move {
                target: CellCoord::new(3, 1)
            })
        );
    }
}
