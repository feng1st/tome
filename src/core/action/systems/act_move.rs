//! Act move: execute every step action created this frame.

use bevy::prelude::*;

use crate::core::action::components::move_action::MoveAction;
use crate::core::health::components::dead::Dead;
use crate::core::map::components::cell_coord::CellCoord;

/// The actor filter: fresh steps of the living only — the dead never
/// move: a lingering death presentation must not walk a corpse,
/// whatever left a move pending on it.
type ActorFilter = (Added<MoveAction>, Without<Dead>);

/// Every living entity with a freshly added `MoveAction` sets its cell
/// coordinate to the target at once — data first, the display chases.
/// The action component is consumed so the next step registers as
/// `Added` again.
pub fn act_move(
    mut commands: Commands,
    mut actors: Query<(Entity, &MoveAction, &mut CellCoord), ActorFilter>,
) {
    for (actor, action, mut cell_coord) in &mut actors {
        commands.entity(actor).remove::<MoveAction>();
        *cell_coord = action.target;
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
                MoveAction {
                    target: CellCoord::new(2, 1),
                },
            ))
            .id();
        world.run_system_once(act_move).unwrap();
        assert_eq!(world.get::<CellCoord>(rat).unwrap(), &CellCoord::new(2, 1));
        assert!(world.get::<MoveAction>(rat).is_none());
    }

    #[test]
    fn the_order_survives_the_step() {
        // The standing order is planning's business: execution touches
        // the action and the cell only.
        let mut world = World::new();
        let player = world
            .spawn((
                CellCoord::new(1, 1),
                MoveAction {
                    target: CellCoord::new(2, 1),
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
        assert!(world.get::<MoveAction>(player).is_none());
        assert_eq!(
            world.get::<Order>(player),
            Some(&Order::Move {
                target: CellCoord::new(3, 1)
            })
        );
    }
}
