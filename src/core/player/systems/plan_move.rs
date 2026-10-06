//! Plan move: the player's step action — one queued cell per due turn.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::health::components::dead::Dead;
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

/// Create the next step action when two conditions pass: the player's
/// next turn is due, and a path is queued. Planning spends the turn:
/// `next_turn.at = now + duration`. No path means no action: the player
/// stands ready and the world stops on the driver's unspent turn. A
/// dead driver plans nothing: the query filters the death marker out,
/// so the due turn is never spent, the clock pins there, and the world
/// freezes.
///
/// Planning never waits for pictures. A click can land while another
/// creature's picture is mid-move — the step plans at the frozen tick
/// all the same and the pictures overlap. Only the tick ledger is
/// serial; overlapping pictures are a display concern.
pub fn plan_move(
    mut commands: Commands,
    world_clock: Res<WorldClock>,
    mut player: Query<PlayerQuery, (With<WorldDriver>, Without<Dead>)>,
) {
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
    fn planning_never_waits_for_pictures() {
        use crate::core::display::components::is_moving::IsMoving;

        let mut world = world_with_clock();
        let player = world.spawn(player_bundle()).id();
        // Due on the unspent turn at 100, path queued, while a creature
        // whose turn tied this tick is mid-move. The plan lands at the
        // frozen tick all the same: pictures may overlap; only the tick
        // ledger is serial.
        world.resource_mut::<WorldClock>().now = 100;
        world.entity_mut(player).insert(NextTurn { at: 100 });
        world.spawn((NextTurn { at: 140 }, IsMoving));
        world.run_system_once(plan_move).unwrap();
        assert_eq!(
            world.get::<Move>(player).map(|m| m.to),
            Some(CellCoord::new(2, 1)),
            "due with a queued path: planning does not wait for pictures"
        );
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 200);
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

    /// A dead driver plans nothing: the queued path is never spent, so
    /// the clock pins at the driver's due turn and a monster whose turn
    /// lies beyond never comes due — the world freezes.
    #[test]
    fn a_dead_driver_freezes_the_world_on_its_unspent_turn() {
        use crate::core::world_clock::systems::advance::advance;

        let mut app = App::new();
        app.init_resource::<WorldClock>()
            .add_systems(Update, (advance, plan_move).chain());
        let player = app
            .world_mut()
            .spawn((
                WorldDriver,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn { at: 10 },
                Path::new(VecDeque::from([CellCoord::new(2, 1)])),
                Dead,
            ))
            .id();
        let monster = app
            .world_mut()
            .spawn((CellCoord::new(3, 3), NextTurn { at: 50 }))
            .id();
        for _ in 0..3 {
            app.update();
        }
        assert_eq!(
            app.world().resource::<WorldClock>().now,
            10,
            "clock pinned at the driver's unspent turn"
        );
        assert!(app.world().get::<Move>(player).is_none());
        assert_eq!(
            app.world().get::<NextTurn>(player).unwrap().at,
            10,
            "the turn is never spent"
        );
        assert_eq!(
            app.world().get::<NextTurn>(monster).unwrap().at,
            50,
            "the monster never comes due"
        );
    }
}
