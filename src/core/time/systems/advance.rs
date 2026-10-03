//! Advance: sweep the world clock to the nearest creature's next turn.
//! Pure integer arithmetic — real time never crosses this boundary.

use bevy::prelude::*;

use crate::core::time::components::next_turn::NextTurn;
use crate::core::time::components::presenting::Presenting;
use crate::core::time::resources::world_clock::WorldClock;

/// The frame opener, two rules:
///
/// 1. Presentation freeze — while any creature's picture is in flight,
///    the clock holds still (the world waits for its pictures; queued
///    turns wait theirs out as time debts).
/// 2. Otherwise the clock steps to the nearest next turn. A ready
///    driver's own future turn is the nearest one, so the world parks on
///    it until input produces the next action. Opening the frame with
///    the sweep is what lets a turn due this frame plan this same
///    frame — no one-frame handshake gap between landing and the next
///    action.
pub fn advance(
    mut world_clock: ResMut<WorldClock>,
    turns: Query<&NextTurn>,
    presenting: Query<(), With<Presenting>>,
) {
    if !presenting.is_empty() {
        return;
    }
    if let Some(nearest) = turns.iter().map(|turn| turn.at).min() {
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
    fn presentation_freezes_the_clock() {
        let mut app = app();
        app.world_mut().spawn((NextTurn { at: 33 }, Presenting));
        app.world_mut().spawn(NextTurn { at: 50 });
        app.update();
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
    fn an_unspent_future_turn_parks_the_clock() {
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
        assert_eq!(now(&app), 0);
    }

    /// End to end: the driver (cost 100) walks a 3-cell path while a rat
    /// (cost 40) takes its interleaved turns. Core-only: no presentation
    /// ever freezes the clock, so turns resolve at frame rate.
    #[test]
    fn the_world_parks_with_a_ready_driver() {
        use std::collections::VecDeque;

        use crate::core::creature::components::speed::Speed;
        use crate::core::hero::systems::plan_move::plan_move;
        use crate::core::map::components::cell_coord::CellCoord;
        use crate::core::map::resources::current_map::{parse_local_map, CurrentMap};
        use crate::core::map::resources::terrain_registry::parse_terrain_registry;
        use crate::core::monster::components::monster_index::MonsterIndex;
        use crate::core::monster::systems::plan_wander::plan_wander;
        use crate::core::movement::components::path::Path;
        use crate::core::movement::systems::act_move::act_move;
        use crate::core::time::components::world_driver::WorldDriver;

        let terrains =
            parse_terrain_registry("test", r#"[ ( terrain: "floor", flags: ["PASSABLE"] ) ]"#);
        let map = parse_local_map(
            "test",
            r#"( legend: { '.': "floor" }, rows: [ "......", ], )"#,
            &terrains,
        );
        let mut app = App::new();
        app.init_resource::<WorldClock>()
            .insert_resource(terrains)
            .insert_resource(CurrentMap::new(map))
            .add_systems(
                Update,
                (advance, plan_move, act_move, plan_wander, act_move).chain(),
            );
        let hero = app
            .world_mut()
            .spawn((
                WorldDriver,
                CellCoord::new(1, 0),
                Speed(110),
                NextTurn::default(),
                Path::new(VecDeque::from([
                    CellCoord::new(2, 0),
                    CellCoord::new(3, 0),
                    CellCoord::new(4, 0),
                ])),
            ))
            .id();
        let rat = app
            .world_mut()
            .spawn((
                MonsterIndex::from_index(0),
                Speed(125),
                CellCoord::new(5, 0),
                NextTurn::default(),
            ))
            .id();

        for _ in 0..30 {
            app.update();
        }

        assert_eq!(
            app.world().get::<CellCoord>(hero).unwrap(),
            &CellCoord::new(4, 0)
        );
        assert!(app.world().get::<Path>(hero).is_none(), "path consumed");
        assert_eq!(
            app.world().resource::<WorldClock>().now,
            300,
            "parked on the driver's unspent turn"
        );
        assert_eq!(app.world().get::<NextTurn>(hero).unwrap().at, 300);
        assert_eq!(
            app.world().get::<NextTurn>(rat).unwrap().at,
            320,
            "the rat's cadence is exact: 8 turns at cost 40, the last one
             waiting beyond the park"
        );
        app.update();
        assert_eq!(app.world().resource::<WorldClock>().now, 300, "frozen");
    }
}
