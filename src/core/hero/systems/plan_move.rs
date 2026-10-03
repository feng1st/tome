//! Plan move: the world driver's step action — one queued cell per due
//! turn.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::creature::components::speed::Speed;
use crate::core::movement::components::path::Path;
use crate::core::movement::components::r#move::Move;
use crate::core::time::components::next_turn::NextTurn;
use crate::core::time::components::presenting::Presenting;
use crate::core::time::components::world_driver::WorldDriver;
use crate::core::time::resources::world_clock::WorldClock;
use crate::core::time::utils::action_cost::standard_action_cost;

/// The world driver's step-planning state: queued path for the target,
/// speed for pricing, and the persistent turn slot to push forward.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct DriverQuery {
    entity: Entity,
    path: &'static Path,
    speed: &'static Speed,
    turn: &'static mut NextTurn,
}

/// Create the next step action when three gates all pass: nothing is in
/// presentation anywhere (the world waits for its pictures — including a
/// faster creature's interleaved hops), the driver's turn is due, and a
/// path is queued. Planning spends the turn: `turn.at = now + cost`. No
/// path means no action: the driver stands ready and the world parks on
/// its unspent turn.
pub fn plan_move(
    mut commands: Commands,
    world_clock: Res<WorldClock>,
    presenting: Query<(), With<Presenting>>,
    mut world_driver: Query<DriverQuery, With<WorldDriver>>,
) {
    if !presenting.is_empty() {
        return;
    }
    let Ok(mut driver) = world_driver.single_mut() else {
        return;
    };
    if world_clock.now < driver.turn.at {
        return;
    }
    let Some(&to) = driver.path.cells.front() else {
        return;
    };
    driver.turn.at = world_clock.now + standard_action_cost(driver.speed.0);
    commands.entity(driver.entity).insert(Move { to });
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

    fn driver_bundle() -> impl Bundle {
        (
            WorldDriver,
            CellCoord::new(1, 1),
            Speed(110),
            NextTurn::default(),
            Path::new(VecDeque::from([CellCoord::new(2, 1)])),
        )
    }

    #[test]
    fn due_driver_with_path_plans_a_step() {
        let mut world = world_with_clock();
        let hero = world.spawn(driver_bundle()).id();
        world.run_system_once(plan_move).unwrap();
        assert_eq!(
            world.get::<Move>(hero).map(|m| m.to),
            Some(CellCoord::new(2, 1))
        );
        assert_eq!(
            world.get::<NextTurn>(hero).unwrap().at,
            100,
            "the turn is spent: next due at now + cost"
        );
    }

    #[test]
    fn slot_not_due_plans_nothing() {
        let mut world = world_with_clock();
        let hero = world.spawn(driver_bundle()).id();
        world.entity_mut(hero).insert(NextTurn { at: 50 });
        world.run_system_once(plan_move).unwrap();
        assert!(world.get::<Move>(hero).is_none());
        assert_eq!(world.get::<NextTurn>(hero).unwrap().at, 50);
    }

    #[test]
    fn the_world_waits_for_any_presentation() {
        let mut world = world_with_clock();
        let hero = world.spawn(driver_bundle()).id();
        // A faster creature's hop is in flight somewhere: nobody plans.
        world.spawn((NextTurn { at: 7 }, Presenting));
        world.run_system_once(plan_move).unwrap();
        assert!(world.get::<Move>(hero).is_none());
        assert_eq!(world.get::<NextTurn>(hero).unwrap().at, 0);
    }

    #[test]
    fn no_path_plans_nothing() {
        let mut world = world_with_clock();
        let hero = world
            .spawn((
                WorldDriver,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        world.run_system_once(plan_move).unwrap();
        assert!(world.get::<Move>(hero).is_none());
        assert_eq!(world.get::<NextTurn>(hero).unwrap().at, 0);
    }
}
