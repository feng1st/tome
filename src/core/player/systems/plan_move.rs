//! Plan move: the player's step action — one queued cell per due turn.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::display::components::is_moving::IsMoving;
use crate::core::movement::components::path::Path;
use crate::core::movement::components::r#move::Move;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::action_duration::STANDARD_ACTION_DURATION;
use crate::core::speed::utils::action_duration::action_duration;
use crate::core::world_clock::components::next_turn::NextTurn;
use crate::core::world_clock::components::world_driver::WorldDriver;
use crate::core::world_clock::resources::world_clock::WorldClock;

/// The player's step-planning state: queued path for the target, speed
/// for pricing, and the persistent next turn to push forward.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerQuery {
    entity: Entity,
    path: &'static Path,
    speed: &'static Speed,
    next_turn: &'static mut NextTurn,
}

/// Create the next step action when three conditions all pass: nothing is
/// moving anywhere (the world waits for moving pictures — including a
/// faster creature's interleaved hops), the player's next turn is due,
/// and a path is queued. Planning spends the turn: `next_turn.at =
/// now + duration`. No path means no action: the player stands ready
/// and the world parks on the driver's unspent turn.
pub fn plan_move(
    mut commands: Commands,
    world_clock: Res<WorldClock>,
    is_moving: Query<(), With<IsMoving>>,
    mut player: Query<PlayerQuery, With<WorldDriver>>,
) {
    if !is_moving.is_empty() {
        return;
    }
    let Ok(mut player) = player.single_mut() else {
        return;
    };
    if world_clock.now < player.next_turn.at {
        return;
    }
    let Some(&to) = player.path.cells.front() else {
        return;
    };
    player.next_turn.at =
        world_clock.now + action_duration(STANDARD_ACTION_DURATION, player.speed.0);
    commands.entity(player.entity).insert(Move { to });
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;

    fn world_with_clock() -> World {
        let mut world = World::new();
        world.insert_resource(WorldClock::default());
        world
    }

    fn player_bundle() -> impl Bundle {
        (
            WorldDriver,
            CellCoord::new(1, 1),
            Speed(110),
            NextTurn::default(),
            Path::new(VecDeque::from([CellCoord::new(2, 1)])),
        )
    }

    #[test]
    fn due_player_with_path_plans_a_step() {
        let mut world = world_with_clock();
        let player = world.spawn(player_bundle()).id();
        world.run_system_once(plan_move).unwrap();
        assert_eq!(
            world.get::<Move>(player).map(|m| m.to),
            Some(CellCoord::new(2, 1))
        );
        assert_eq!(
            world.get::<NextTurn>(player).unwrap().at,
            100,
            "the turn is spent: next due at now + duration"
        );
    }

    #[test]
    fn slot_not_due_plans_nothing() {
        let mut world = world_with_clock();
        let player = world.spawn(player_bundle()).id();
        world.entity_mut(player).insert(NextTurn { at: 50 });
        world.run_system_once(plan_move).unwrap();
        assert!(world.get::<Move>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 50);
    }

    #[test]
    fn the_world_waits_for_any_movement() {
        let mut world = world_with_clock();
        let player = world.spawn(player_bundle()).id();
        // A faster creature's hop is in flight somewhere: nobody plans.
        world.spawn((NextTurn { at: 7 }, IsMoving));
        world.run_system_once(plan_move).unwrap();
        assert!(world.get::<Move>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 0);
    }

    #[test]
    fn no_path_plans_nothing() {
        let mut world = world_with_clock();
        let player = world
            .spawn((
                WorldDriver,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        world.run_system_once(plan_move).unwrap();
        assert!(world.get::<Move>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 0);
    }
}
