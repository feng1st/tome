//! Advance: sweep the world clock to the nearest creature's next turn.
//! Pure integer arithmetic — real time never crosses this boundary.

use bevy::prelude::*;

use crate::core::display::components::is_moving::IsMoving;
use crate::core::health::components::dead::Dead;
use crate::core::world_clock::components::next_turn::NextTurn;
use crate::core::world_clock::components::world_driver::WorldDriver;
use crate::core::world_clock::resources::world_clock::WorldClock;

/// The frame opener, two rules:
///
/// 1. Movement freeze — while any creature's picture is still moving,
///    the clock holds still (the world waits for its pictures; queued
///    turns wait theirs out as time debts).
/// 2. Otherwise the clock steps to the nearest next turn. A ready
///    driver's own future turn is the nearest one, so the world stops on
///    it until input produces the next action. Opening the frame with
///    the sweep is what lets a turn due this frame plan this same
///    frame — no one-frame handshake gap between landing and the next
///    action.
///
/// Dead turns leave the schedule — with one exception: the driver's.
/// A dead driver's never-spent turn is what holds the world at the
/// game over; a dead monster's turn must not pin anything, or the
/// world would freeze on a corpse. The exception lives here, in the
/// domain that owns the driver marker.
/// The next-turn filter — turns the schedule still counts: every
/// living creature's, plus the driver's even in death — the unspent
/// driver turn is the game-over hold, while any other dead turn
/// leaves the schedule so a corpse pins nothing.
type NextTurnFilter = Or<(Without<Dead>, With<WorldDriver>)>;

pub fn advance(
    next_turns: Query<&NextTurn, NextTurnFilter>,
    is_moving: Query<(), With<IsMoving>>,
    mut world_clock: ResMut<WorldClock>,
) {
    if !is_moving.is_empty() {
        return;
    }
    if let Some(nearest) = next_turns.iter().map(|next_turn| next_turn.at).min() {
        world_clock.now = world_clock.now.max(nearest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        let mut app = App::new();
        app.init_resource::<WorldClock>()
            .add_systems(Update, advance);
        app
    }

    fn now(app: &App) -> i64 {
        app.world().resource::<WorldClock>().now
    }

    #[test]
    fn movement_freezes_the_clock() {
        let mut app = app();
        app.world_mut().spawn((NextTurn { at: 33 }, IsMoving));
        app.world_mut().spawn(NextTurn { at: 50 });
        app.update();
        // The clock rests at its current tick while the picture moves.
        assert_eq!(now(&app), 0, "the world waits for its pictures");
    }

    #[test]
    fn clock_sweeps_to_the_nearest_turn() {
        let mut app = app();
        app.world_mut().spawn(NextTurn { at: 33 });
        app.world_mut().spawn(NextTurn { at: 50 });
        app.world_mut().spawn(NextTurn { at: 100 });
        app.update();
        assert_eq!(now(&app), 33);
    }

    #[test]
    fn an_unspent_future_turn_holds_the_clock() {
        let mut app = app();
        app.world_mut().spawn(NextTurn { at: 100 });
        app.world_mut().spawn(NextTurn { at: 320 });
        app.update();
        assert_eq!(now(&app), 100);
        app.update();
        assert_eq!(now(&app), 100, "the world never runs past a ready driver");
    }

    #[test]
    fn empty_world_holds_still() {
        let mut app = app();
        app.update();
        // An empty world: nothing is due, time rests at the origin.
        assert_eq!(now(&app), 0);
    }

    #[test]
    fn a_dead_monster_s_turn_leaves_the_schedule() {
        let mut app = app();
        // A dead monster's due turn and a living one's future turn:
        // the clock must sweep past the corpse onto the living turn.
        app.world_mut().spawn((
            NextTurn { at: -5 },
            crate::core::health::components::dead::Dead,
        ));
        app.world_mut().spawn(NextTurn { at: 40 });
        app.update();
        assert_eq!(now(&app), 40, "the corpse's turn pins nothing");
    }

    #[test]
    fn a_dead_driver_s_turn_holds_the_world() {
        let mut app = app();
        app.world_mut().spawn((
            crate::core::world_clock::components::world_driver::WorldDriver,
            crate::core::health::components::dead::Dead,
            NextTurn { at: 7 },
        ));
        app.update();
        assert_eq!(now(&app), 7, "the game over holds on the unspent turn");
    }

    #[test]
    fn the_ready_driver_s_unspent_turn_holds_the_clock() {
        let mut app = app();
        app.world_mut().spawn((
            crate::core::world_clock::components::world_driver::WorldDriver,
            NextTurn::default(),
        ));
        app.world_mut().spawn(NextTurn { at: 100 });
        app.update();
        assert_eq!(now(&app), 0);
        app.update();
        // The ready driver's unspent turn is the nearest: the world
        // waits on it until input produces the next action.
        assert_eq!(now(&app), 0, "the world never runs past a ready driver");
    }

    /// End to end: the player (duration 100) walks a 3-cell path while a
    /// rat (duration 40) takes its interleaved turns. Core-only: no
    /// movement ever freezes the clock, so turns resolve at frame rate.
    #[test]
    fn the_world_stops_with_a_ready_driver() {
        use crate::core::action::systems::act_move::act_move;
        use crate::core::map::components::cell_coord::CellCoord;
        use crate::core::map::resources::current_map::{parse_local_map, CurrentMap};
        use crate::core::map::resources::terrain_registry::parse_terrain_registry;
        use crate::core::monster::components::monster_index::MonsterIndex;
        use crate::core::monster::systems::plan_action::plan_action as monster_plan_action;
        use crate::core::player::components::order::Order;
        use crate::core::player::systems::plan_action::plan_action;
        use crate::core::rng::resources::game_rng::GameRng;
        use crate::core::speed::components::speed::Speed;
        use crate::core::world_clock::components::world_driver::WorldDriver;

        let terrains = parse_terrain_registry(
            "test",
            r#"[ ( terrain: "floor", flags: ["PASSABLE"] ), ( terrain: "wall", flags: [] ) ]"#,
        );
        // The rat wanders in a sealed row below the wall: its cadence is
        // exact, and it can never interfere with the driver's walk.
        let map = parse_local_map(
            "test",
            r#"( legend: { '.': "floor", 'x': "wall" }, rows: [ "......", "xxxxxx", "......", ], )"#,
            &terrains,
        );
        let mut app = App::new();
        app.init_resource::<WorldClock>()
            .init_resource::<GameRng>()
            .insert_resource(terrains)
            .insert_resource(CurrentMap::new(map))
            .add_systems(
                Update,
                (
                    advance,
                    plan_action,
                    act_move,
                    monster_plan_action,
                    act_move,
                )
                    .chain(),
            );
        let player = app
            .world_mut()
            .spawn((
                crate::core::player::components::player::Player,
                WorldDriver,
                CellCoord::new(1, 0),
                Speed(110),
                NextTurn::default(),
                Order::Move {
                    target: CellCoord::new(4, 0),
                },
            ))
            .id();
        let rat = app
            .world_mut()
            .spawn((
                MonsterIndex::from_index(0),
                Speed(125),
                CellCoord::new(5, 2),
                NextTurn::default(),
            ))
            .id();

        for _ in 0..30 {
            app.update();
        }

        assert_eq!(
            app.world().get::<CellCoord>(player).unwrap(),
            &CellCoord::new(4, 0)
        );
        assert!(
            app.world().get::<Order>(player).is_none(),
            "arrival cleared the order"
        );
        assert_eq!(
            app.world().resource::<WorldClock>().now,
            300,
            "held on the driver's unspent turn"
        );
        assert_eq!(app.world().get::<NextTurn>(player).unwrap().at, 300);
        assert_eq!(
            app.world().get::<NextTurn>(rat).unwrap().at,
            320,
            "the rat's cadence is exact: 8 turns at duration 40, the last
             one waiting beyond the stop"
        );
        app.update();
        assert_eq!(app.world().resource::<WorldClock>().now, 300, "frozen");
    }
}
