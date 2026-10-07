//! Plan action: the monster's turn planning — the attack at an
//! adjacent living player first, the idle wander otherwise.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use rand::Rng;

use crate::core::combat::components::attack::Attack;
use crate::core::health::components::dead::Dead;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain::Terrain;
use crate::core::map::utils::adjacency::adjacent;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::movement::components::r#move::Move;
use crate::core::player::components::player::Player;
use crate::core::rng::resources::game_rng::GameRng;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::action_duration::STANDARD_ACTION_DURATION;
use crate::core::speed::utils::action_duration::action_duration;
use crate::core::world_clock::components::next_turn::NextTurn;
use crate::core::world_clock::resources::world_clock::WorldClock;

/// A monster's turn-planning state: cell and speed for pricing, and the
/// persistent next turn.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct MonsterQuery {
    entity: Entity,
    cell: &'static CellCoord,
    speed: &'static Speed,
    next_turn: &'static mut NextTurn,
}

/// The monster planner's query filter: monster entities only, never
/// the dead, never the driver.
type MonsterFilter = (With<MonsterIndex>, Without<Dead>);

/// The living player's data: the entity an attack would target and the
/// cell adjacency is judged against.
#[derive(QueryData)]
pub struct PlayerQuery {
    entity: Entity,
    cell: &'static CellCoord,
}

/// The 8 step directions a wander can pick.
const DIRECTIONS: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

/// How many independent random directions one wander attempt tries
/// before giving up: the first walkable pick wins, and a monster that
/// fails all of them stands.
const RANDOM_ATTEMPTS: usize = 4;

/// Every living monster with a due turn plans one action. A living
/// player in one of the eight neighboring cells is attacked: the attack
/// is deterministic — no stand roll, no direction draw — plans no step,
/// and prices its turn as usual. Otherwise the monster wanders: 75%
/// standing still, 25% a random direction — up to four independent
/// picks, the first passable and creature-free one wins, and a monster
/// whose picks all fail stands. Occupied cells — any living creature's
/// cell, the player's included, plus the cells this frame's planned
/// steps head for — count as blocked: a monster never merges into
/// another creature. Standing is an action all the same: the turn is
/// spent either way. The world opens at zero: every monster's first
/// turn sits at the world's zero alongside the driver's, so resident
/// monsters take their first action as the world opens; afterwards an
/// unspent turn waits its tick like everyone else's — no planner
/// checks for it. Dead monsters are skipped: a corpse neither plans
/// nor blocks a cell.
///
/// The only gate is the clock: while any picture is still moving the
/// clock is held (by `advance`), and a plan always prices its turn into
/// the future. Planning never waits for pictures — they may overlap;
/// only the tick ledger is serial.
// The system boundary keeps the parameters flat: each is its own query
// or resource, and none pair naturally into a bundle.
#[allow(clippy::too_many_arguments)]
pub fn plan_action(
    mut commands: Commands,
    world_clock: Res<WorldClock>,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    mut game_rng: ResMut<GameRng>,
    player: Query<PlayerQuery, (With<Player>, Without<Dead>)>,
    mut monsters: Query<MonsterQuery, MonsterFilter>,
) {
    // Occupied cells: every living creature's cell — the player's
    // included. A planned step's target joins the set as plans land, so
    // two monsters planning on the same frame never merge either.
    let mut occupied: Vec<CellCoord> = monsters.iter().map(|monster| *monster.cell).collect();
    let player = player
        .single()
        .ok()
        .map(|player| (player.entity, *player.cell));
    if let Some((_, player_cell)) = player {
        occupied.push(player_cell);
    }
    let rng = &mut game_rng.rng;
    for mut monster in &mut monsters {
        // The turn is not due yet.
        if world_clock.now < monster.next_turn.at {
            continue;
        }
        // The attack branch precedes the wander: an adjacent living
        // player is always attacked, and the turn is priced either way.
        if let Some((player_entity, player_cell)) = player {
            if adjacent(*monster.cell, player_cell) {
                commands.entity(monster.entity).insert(Attack {
                    target: player_entity,
                });
                monster.next_turn.at =
                    world_clock.now + action_duration(STANDARD_ACTION_DURATION, monster.speed.0);
                continue;
            }
        }
        let stands = rng.random_range(0..100) >= 25;
        let is_walkable = |target: CellCoord| {
            !occupied.contains(&target)
                && current_map
                    .map()
                    .get(target)
                    .and_then(|terrain_index| terrain_registry.get(terrain_index))
                    .is_some_and(Terrain::is_walkable)
        };
        let mut target = None;
        if !stands {
            for _ in 0..RANDOM_ATTEMPTS {
                let direction = DIRECTIONS[rng.random_range(0..DIRECTIONS.len())];
                let to = *monster.cell + direction;
                if is_walkable(to) {
                    target = Some(to);
                    break;
                }
            }
        }
        if let Some(to) = target {
            occupied.push(to);
            commands.entity(monster.entity).insert(Move { to });
        }
        monster.next_turn.at =
            world_clock.now + action_duration(STANDARD_ACTION_DURATION, monster.speed.0);
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    use super::*;
    use crate::core::display::components::is_moving::IsMoving;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;
    use crate::core::map::types::local_map::LocalMap;
    use crate::core::world_clock::components::world_driver::WorldDriver;

    const TERRAINS: &str = r#"[
        ( terrain: "floor", flags: ["PASSABLE"] ),
        ( terrain: "wall",  flags: [] ),
    ]"#;

    /// A walled room; `floor` is the list of floor cells inside it.
    fn room(width: usize, height: usize, floor: &[CellCoord]) -> LocalMap {
        let rows: Vec<String> = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        if floor.contains(&CellCoord::new(x as i32, y as i32)) {
                            '.'
                        } else {
                            '#'
                        }
                    })
                    .collect()
            })
            .collect();
        let rows: Vec<String> = rows.iter().map(|r| format!("\"{r}\"")).collect();
        let doc = format!(
            "( legend: {{ '#': \"wall\", '.': \"floor\" }}, rows: [ {} ], )",
            rows.join(", ")
        );
        parse_local_map("test", &doc, &parse_terrain_registry("test", TERRAINS))
    }

    fn open_room() -> LocalMap {
        let floor: Vec<CellCoord> = (1..5)
            .flat_map(|y| (1..5).map(move |x| CellCoord::new(x, y)))
            .collect();
        room(6, 6, &floor)
    }

    fn app_with(map: LocalMap) -> App {
        let mut app = App::new();
        app.insert_resource(WorldClock { now: 0 })
            .init_resource::<GameRng>()
            .insert_resource(parse_terrain_registry("test", TERRAINS))
            .insert_resource(CurrentMap::new(map))
            .add_systems(
                Update,
                (
                    crate::core::world_clock::systems::advance::advance,
                    plan_action,
                )
                    .chain(),
            );
        app
    }

    /// A started world with its driver at `cell` (the turn has left
    /// zero).
    fn spawn_driver_at(app: &mut App, cell: CellCoord) -> Entity {
        app.world_mut()
            .spawn((Player, WorldDriver, cell, NextTurn { at: 10 }))
            .id()
    }

    fn spawn_started_driver(app: &mut App) {
        spawn_driver_at(app, CellCoord::new(0, 0));
    }

    fn spawn_rat(app: &mut App, cell: CellCoord) -> Entity {
        app.world_mut()
            .spawn((
                MonsterIndex::from_index(0),
                Speed(110),
                cell,
                NextTurn::default(),
            ))
            .id()
    }

    #[test]
    fn a_due_monster_plans_as_the_world_opens() {
        // The world opens at zero: the driver's and the monster's
        // first turns sit there together, so the monster takes its
        // first action without waiting for input.
        let mut app = app_with(open_room());
        app.insert_resource(WorldClock::default());
        app.world_mut()
            .spawn((WorldDriver, CellCoord::new(0, 0), NextTurn::default()));
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        app.update();
        assert_eq!(
            app.world().get::<NextTurn>(rat).unwrap().at,
            100,
            "the rat's turn came due at the world's zero and was spent"
        );
    }

    #[test]
    fn due_monster_spends_its_turn() {
        let mut app = app_with(open_room());
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        app.update();
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 100);
        // The wander, not the attack: the driver is beyond adjacency.
        assert!(
            app.world().get::<Attack>(rat).is_none(),
            "no attack planned"
        );
        if let Some(m) = app.world().get::<Move>(rat) {
            let dx = (m.to.x - 2).abs();
            let dy = (m.to.y - 2).abs();
            assert!(dx <= 1 && dy <= 1 && (dx + dy) > 0);
        }
    }

    #[test]
    fn slot_not_due_plans_nothing() {
        let mut app = app_with(open_room());
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        app.world_mut()
            .entity_mut(rat)
            .insert(NextTurn { at: 1000 });
        app.update();
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 1000);
        assert!(app.world().get::<Move>(rat).is_none());
    }

    #[test]
    fn hop_in_flight_holds_the_clock_so_no_plan_lands() {
        let mut app = app_with(open_room());
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        // A hop in flight: the flag is up and the turn is priced into the
        // future. `advance` holds the clock for the flag, so the turn can
        // never come due while the hop runs — the planner needs no flag
        // of its own.
        app.world_mut().entity_mut(rat).insert(IsMoving);
        app.world_mut().entity_mut(rat).insert(NextTurn { at: 40 });
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.world().resource::<WorldClock>().now, 0, "clock held");
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 40);
        assert!(app.world().get::<Move>(rat).is_none());
    }

    #[test]
    fn an_adjacent_player_is_attacked_and_the_wander_never_runs() {
        // The attack branch precedes the 75% stand roll: over the seeds
        // the turnover never yields a wander action.
        for seed in 1..=8u64 {
            let mut app = app_with(open_room());
            app.insert_resource(GameRng::seeded(seed));
            let driver = spawn_driver_at(&mut app, CellCoord::new(2, 2));
            let rat = spawn_rat(&mut app, CellCoord::new(3, 3));

            app.update();

            assert_eq!(
                app.world().get::<Attack>(rat).map(|attack| attack.target),
                Some(driver),
                "seed {seed}"
            );
            assert!(app.world().get::<Move>(rat).is_none(), "no step is planned");
            assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 100);
        }
    }

    #[test]
    fn a_dead_player_is_not_attacked() {
        let mut app = app_with(open_room());
        let driver = spawn_driver_at(&mut app, CellCoord::new(2, 2));
        app.world_mut().entity_mut(driver).insert(Dead);
        let rat = spawn_rat(&mut app, CellCoord::new(3, 3));

        app.update();

        assert!(app.world().get::<Attack>(rat).is_none());
    }

    #[test]
    fn walled_in_spends_the_turn_without_a_step() {
        // The driver stands off adjacency, so the walled-in rat gets its
        // wander roll — and stands on it.
        let mut app = app_with(room(4, 4, &[CellCoord::new(1, 1)]));
        spawn_driver_at(&mut app, CellCoord::new(3, 3));
        let rat = spawn_rat(&mut app, CellCoord::new(1, 1));
        app.update();
        assert_eq!(app.world().get::<NextTurn>(rat).unwrap().at, 100);
        assert!(
            app.world().get::<Move>(rat).is_none(),
            "no walkable neighbor: stand, not a wasted step"
        );
    }

    /// A seeded source predicts the plan exactly: an independent
    /// generator on the same seed, replaying the same draw order (one
    /// stand roll, then direction picks), must land on the same plan.
    #[test]
    fn seeded_source_predicts_the_plan() {
        for seed in 1..=20u64 {
            let mut replay = StdRng::seed_from_u64(seed);
            let stands = replay.random_range(0..100) >= 25;
            let expected = if stands {
                None
            } else {
                // The open room's every neighbor is walkable: the
                // first direction pick lands.
                let direction = DIRECTIONS[replay.random_range(0..DIRECTIONS.len())];
                Some(CellCoord::new(2, 2) + direction)
            };

            let mut app = app_with(open_room());
            app.insert_resource(GameRng::seeded(seed));
            spawn_started_driver(&mut app);
            let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
            app.update();

            let observed = app.world().get::<Move>(rat).map(|m| m.to);
            assert_eq!(observed, expected, "seed {seed}");
        }
    }

    /// Drive `turns` due turns for one rat through a live app, handing
    /// back every planned step (the move is consumed each turn so the
    /// next can be observed).
    fn driven_steps(app: &mut App, rat: Entity, turns: u32) -> Vec<Move> {
        let mut steps = Vec::new();
        for turn in 1..=turns {
            app.world_mut().resource_mut::<WorldClock>().now = turn as i64 * 100;
            app.world_mut().entity_mut(rat).insert(NextTurn {
                at: turn as i64 * 100,
            });
            app.update();
            if let Some(step) = app.world().get::<Move>(rat).copied() {
                steps.push(step);
                app.world_mut().entity_mut(rat).remove::<Move>();
            }
        }
        steps
    }

    #[test]
    fn an_occupied_neighbor_counts_as_blocked() {
        // The cell east is held by another rat: across a long sweep the
        // rat never plans a step onto it, though every neighbor is open
        // floor.
        let mut app = app_with(open_room());
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        let _ = spawn_rat(&mut app, CellCoord::new(3, 2));
        let steps = driven_steps(&mut app, rat, 200);
        assert!(
            steps.len() > 20,
            "the sweep exercised real moves: {}",
            steps.len()
        );
        for step in &steps {
            assert_ne!(
                step.to,
                CellCoord::new(3, 2),
                "an occupied cell is never picked"
            );
        }
    }

    #[test]
    fn the_driver_cell_is_never_stepped_onto() {
        // Off adjacency to start: the rat wanders until it reaches a
        // neighboring cell, then attacks instead of stepping — either
        // way, never onto the driver's cell.
        let mut app = app_with(open_room());
        let driver = spawn_driver_at(&mut app, CellCoord::new(2, 2));
        let driver_cell = *app.world().get::<CellCoord>(driver).unwrap();
        let rat = spawn_rat(&mut app, CellCoord::new(4, 1));
        let mut actions = 0;
        for turn in 1..=200 {
            app.world_mut().resource_mut::<WorldClock>().now = turn as i64 * 100;
            app.world_mut().entity_mut(rat).insert(NextTurn {
                at: turn as i64 * 100,
            });
            app.update();
            if let Some(step) = app.world().get::<Move>(rat).copied() {
                actions += 1;
                assert_ne!(step.to, driver_cell, "the driver's cell is never picked");
                app.world_mut().entity_mut(rat).remove::<Move>();
            } else if app.world().get::<Attack>(rat).is_some() {
                actions += 1;
                app.world_mut().entity_mut(rat).remove::<Attack>();
            }
        }
        assert!(actions > 20, "the sweep exercised real actions: {actions}");
    }

    #[test]
    fn a_rat_ringed_by_creatures_stands() {
        // Every neighbor held: all four picks fail each move roll, the
        // rat stands, and the turn is spent as usual.
        let mut app = app_with(open_room());
        spawn_started_driver(&mut app);
        let rat = spawn_rat(&mut app, CellCoord::new(2, 2));
        for direction in DIRECTIONS {
            let _ = spawn_rat(&mut app, CellCoord::new(2 + direction.x, 2 + direction.y));
        }
        let steps = driven_steps(&mut app, rat, 50);
        assert!(steps.is_empty(), "a ringed rat never moves");
        assert_eq!(
            app.world().get::<NextTurn>(rat).unwrap().at,
            51 * 100,
            "standing still still spends the turn: the 50th turn prices the next"
        );
    }

    /// Two monsters cannot plan onto the same cell in one frame: a
    /// planned step's target counts as occupied for the rest of the run.
    #[test]
    fn two_monsters_never_plan_onto_the_same_cell() {
        let mut moved = 0;
        for seed in 0..300u64 {
            let mut app = app_with(room(
                6,
                3,
                &[
                    CellCoord::new(1, 1),
                    CellCoord::new(2, 1),
                    CellCoord::new(3, 1),
                    CellCoord::new(4, 1),
                ],
            ));
            app.insert_resource(GameRng::seeded(seed));
            spawn_started_driver(&mut app);
            let a = spawn_rat(&mut app, CellCoord::new(2, 1));
            let b = spawn_rat(&mut app, CellCoord::new(4, 1));
            app.update();
            let ta = app.world().get::<Move>(a).map(|m| m.to);
            let tb = app.world().get::<Move>(b).map(|m| m.to);
            if ta.is_some() || tb.is_some() {
                moved += 1;
            }
            assert!(
                ta != Some(CellCoord::new(3, 1)) || tb != Some(CellCoord::new(3, 1)),
                "both monsters targeted the shared cell at seed {seed}: {ta:?} / {tb:?}"
            );
        }
        assert!(moved > 20, "the sweep exercised real moves: {moved}");
    }
}
