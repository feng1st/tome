//! Executes `TargetCell` commands: the core decides what a target means
//! — a monster's cell asks for an attack, a walkable cell for a walk —
//! and dispatches the standing order; pathfinding lives in planning.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::health::components::dead::Dead;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain::Terrain;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::player::commands::target_cell::TargetCell;
use crate::core::player::components::order::Order;
use crate::core::player::components::player::Player;

/// The command executor's player view: identity and position.
#[derive(QueryData)]
pub struct PlayerCellQuery {
    entity: Entity,
    cell: &'static CellCoord,
}

/// Execute every `TargetCell` command: the executor only dispatches —
/// a living monster on the targeted cell issues an attack order, a
/// walkable cell issues a move order, and an impassable cell or the
/// player's own cell changes nothing. A new command replaces the
/// standing order. Commands to a dead driver are ignored — the query
/// filters the death marker out.
/// The monster filter — living monsters only: a corpse (its body may
/// linger through its death fade) is not an attack target, and its
/// cell is ground to walk on.
type MonsterFilter = (With<MonsterIndex>, Without<Dead>);

pub fn execute(
    mut commands: Commands,
    mut target_cell_reader: MessageReader<TargetCell>,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    monsters: Query<(Entity, &CellCoord), MonsterFilter>,
    player: Query<PlayerCellQuery, (With<Player>, Without<Dead>)>,
) {
    let Ok(player) = player.single() else {
        return;
    };
    let local_map = current_map.map();
    for target_cell in target_cell_reader.read() {
        let target = target_cell.0;
        if target == *player.cell {
            continue;
        }
        if let Some((monster, _)) = monsters.iter().find(|(_, cell)| **cell == target) {
            commands
                .entity(player.entity)
                .insert(Order::Attack { target: monster });
            continue;
        }
        let target_walkable = local_map
            .get(target)
            .and_then(|terrain_index| terrain_registry.get(terrain_index))
            .is_some_and(Terrain::is_walkable);
        if !target_walkable {
            continue;
        }
        commands
            .entity(player.entity)
            .insert(Order::Move { target });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, VecDeque};

    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    use super::*;
    use crate::core::action::systems::act_move::act_move;
    use crate::core::core_phase::CorePhase;
    use crate::core::dice::types::dice::Dice;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;
    use crate::core::map::types::local_map::LocalMap;
    use crate::core::map::utils::pathfinding::find_path;
    use crate::core::player::systems::plan_action::plan_action;
    use crate::core::speed::components::speed::Speed;
    use crate::core::world_clock::components::next_turn::NextTurn;
    use crate::core::world_clock::components::world_driver::WorldDriver;
    use crate::core::world_clock::resources::world_clock::WorldClock;

    const TERRAINS: &str = r#"[
        ( terrain: "floor", flags: ["PASSABLE"] ),
        ( terrain: "wall",  flags: [] ),
        ( terrain: "water", flags: ["LIQUID"] ),
    ]"#;

    fn terrain_registry() -> TerrainRegistry {
        parse_terrain_registry("test", TERRAINS)
    }

    /// A walled room with a 4x5 pool slightly below its center, matching
    /// the test room's proportions at a smaller scale.
    fn room() -> LocalMap {
        let rows: Vec<String> = (0..10)
            .map(|y| {
                if y == 0 || y == 9 {
                    "################".to_string()
                } else {
                    let mut row = "#..............#".to_string();
                    if (3..8).contains(&y) {
                        row.replace_range(5..9, "~~~~");
                    }
                    row
                }
            })
            .collect();
        let rows: Vec<String> = rows.iter().map(|r| format!("\"{r}\"")).collect();
        let doc = format!(
            "( legend: {{ '#': \"wall\", '.': \"floor\", '~': \"water\" }}, rows: [ {} ], )",
            rows.join(", ")
        );
        parse_local_map("test", &doc, &terrain_registry())
    }

    fn app() -> App {
        let mut app = App::new();
        app.add_message::<TargetCell>()
            .insert_resource(terrain_registry())
            .insert_resource(CurrentMap::new(room()))
            .add_systems(Update, execute);
        app
    }

    #[test]
    fn targeting_a_floor_cell_issues_a_move_order() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(4, 2)));
        app.update();
        assert_eq!(
            app.world().get::<Order>(player),
            Some(&Order::Move {
                target: CellCoord::new(4, 2)
            })
        );
    }

    #[test]
    fn unwalkable_and_out_of_bounds_targets_change_nothing() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        for target in [
            CellCoord::new(0, 0),  // wall corner
            CellCoord::new(-1, 2), // left of the map
            CellCoord::new(16, 2), // right of the map
            CellCoord::new(6, 4),  // water pool
        ] {
            app.world_mut().write_message(TargetCell(target));
        }
        app.update();
        assert!(app.world().get::<Order>(player).is_none());
    }

    #[test]
    fn an_unreachable_target_still_issues_the_order() {
        // Reachability is planning's business: the executor dispatches
        // any walkable target, and planning clears what it cannot walk.
        let doc = r######"(
            legend: { '#': "wall", '.': "floor" },
            rows: [ "#####", "#.#.#", "#.#.#", "#.#.#", "#####" ],
        )"######;
        let mut app = App::new();
        app.add_message::<TargetCell>()
            .insert_resource(terrain_registry())
            .insert_resource(CurrentMap::new(parse_local_map(
                "test",
                doc,
                &terrain_registry(),
            )))
            .add_systems(Update, execute);
        let player = app.world_mut().spawn((Player, CellCoord::new(1, 1))).id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(3, 1)));
        app.update();
        assert_eq!(
            app.world().get::<Order>(player),
            Some(&Order::Move {
                target: CellCoord::new(3, 1)
            })
        );
    }

    #[test]
    fn targeting_a_monster_issues_an_attack_order() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        let rat = app
            .world_mut()
            .spawn((MonsterIndex::from_index(0), CellCoord::new(4, 2)))
            .id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(4, 2)));
        app.update();
        assert_eq!(
            app.world().get::<Order>(player),
            Some(&Order::Attack { target: rat })
        );
    }

    #[test]
    fn a_new_target_replaces_the_standing_order() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        let rat = app
            .world_mut()
            .spawn((MonsterIndex::from_index(0), CellCoord::new(4, 2)))
            .id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(4, 2)));
        app.update();
        assert!(app.world().get::<Order>(player).is_some());
        // A floor target replaces the pursuit with a walk.
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(6, 2)));
        app.update();
        assert_eq!(
            app.world().get::<Order>(player),
            Some(&Order::Move {
                target: CellCoord::new(6, 2)
            })
        );
        // A monster target replaces the walk with a pursuit.
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(4, 2)));
        app.update();
        assert_eq!(
            app.world().get::<Order>(player),
            Some(&Order::Attack { target: rat })
        );
    }

    #[test]
    fn a_dead_monster_is_just_a_floor_cell() {
        // Dead monsters leave the world, so a despawned occupant's cell
        // dispatches a walk — the query sees only living monsters.
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        let rat = app
            .world_mut()
            .spawn((MonsterIndex::from_index(0), CellCoord::new(4, 2)))
            .id();
        app.world_mut().despawn(rat);
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(4, 2)));
        app.update();
        assert_eq!(
            app.world().get::<Order>(player),
            Some(&Order::Move {
                target: CellCoord::new(4, 2)
            })
        );
    }

    #[test]
    fn targeting_the_player_cell_changes_nothing() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(2, 2)));
        app.update();
        assert!(app.world().get::<Order>(player).is_none());
    }

    /// A dead driver's commands are ignored: no order forms even for a
    /// walkable target.
    #[test]
    fn a_dead_driver_ignores_commands() {
        let mut app = app();
        let player = app
            .world_mut()
            .spawn((Player, Dead, CellCoord::new(2, 2)))
            .id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(4, 2)));
        app.update();
        assert!(app.world().get::<Order>(player).is_none());
    }

    /// The full combat loop over the real phase topology: a target on a
    /// distant rat pursues, attacks when adjacent, and the slain rat
    /// leaves the world — the player never standing on its cell, the
    /// world freezing once the order is spent.
    #[test]
    fn the_combat_loop_runs_from_target_to_slain_rat() {
        #[derive(Resource, Default)]
        struct Requests(Vec<Damage>);

        /// Capture the attacks between execution and application, then
        /// put them back — the apply system runs them as usual.
        fn record_requests(mut messages: ResMut<Messages<Damage>>, mut requests: ResMut<Requests>) {
            let captured: Vec<Damage> = messages.drain().collect();
            for request in &captured {
                messages.write(*request);
            }
            requests.0.extend(captured);
        }

        use crate::core::action::systems::act_attack::act_attack;
        use crate::core::combat::components::armor_class::ArmorClass;
        use crate::core::combat::components::blows::Blows;
        use crate::core::combat::components::combat_bonuses::CombatBonuses;
        use crate::core::combat::messages::attacked::Attacked;
        use crate::core::combat::types::blow::Blow;
        use crate::core::combat::utils::armor_class::armor_class;
        use crate::core::combat::utils::attack::{attack_chance, unarmed_damage};
        use crate::core::health::components::hit_points::HitPoints;
        use crate::core::health::messages::damage::Damage;
        use crate::core::health::messages::damaged::Damaged;
        use crate::core::health::systems::apply_damage::apply_damage;
        use crate::core::rng::resources::game_rng::GameRng;
        use crate::core::world_clock::systems::advance::advance;

        // One order, one attack: the seed must land the first swing
        // (percentile past the bands, power roll beating three quarters
        // of the rat's armor), or the driver stands ready after a miss.
        // The probe replays the real draw sequence against the real
        // chance.
        let probe_bonuses = CombatBonuses {
            hit: 5,
            damage: 3,
            armor: 0,
        };
        let chance = attack_chance(&probe_bonuses);
        let seed = (0..10_000u64)
            .find(|seed| {
                let mut probe = StdRng::seed_from_u64(*seed);
                let percentile = probe.random_range(0..100);
                percentile >= 10 && probe.random_range(0..chance) >= 7 * 3 / 4
            })
            .expect("a hitting seed exists");
        let mut app = App::new();
        app.insert_resource(WorldClock { now: 0 })
            .insert_resource(GameRng::seeded(seed))
            .insert_resource(terrain_registry())
            .insert_resource(CurrentMap::new(room()))
            .add_message::<TargetCell>()
            .add_message::<Damage>()
            .add_message::<Damaged>()
            .add_message::<Attacked>()
            .configure_sets(
                Update,
                (
                    CorePhase::Advance,
                    CorePhase::Command,
                    CorePhase::PlayerPlan,
                    CorePhase::PlayerAct,
                    CorePhase::PlayerResolve,
                )
                    .chain(),
            )
            .add_systems(Update, advance.in_set(CorePhase::Advance))
            .add_systems(Update, execute.in_set(CorePhase::Command))
            .add_systems(Update, plan_action.in_set(CorePhase::PlayerPlan))
            .add_systems(Update, act_move.in_set(CorePhase::PlayerAct))
            .add_systems(
                Update,
                (act_attack, record_requests)
                    .chain()
                    .in_set(CorePhase::PlayerAct),
            )
            .add_systems(Update, apply_damage.in_set(CorePhase::PlayerResolve))
            .add_systems(
                Update,
                crate::core::health::systems::despawn_dead::despawn_dead.in_set(CorePhase::Derive),
            )
            .insert_resource(Requests::default());
        // What the derive would attach, hand-built from the probe's
        // bonuses: one unarmed blow and the armor class.
        let probe_blows = Blows(vec![Blow {
            chance,
            damage: Dice {
                dice: unarmed_damage(&probe_bonuses),
                side: 1,
            },
        }]);
        let probe_armor_class = ArmorClass(armor_class(&probe_bonuses));
        let player = app
            .world_mut()
            .spawn((
                Player,
                WorldDriver,
                CellCoord::new(2, 2),
                Speed(110),
                NextTurn::default(),
                probe_bonuses,
                probe_blows,
                probe_armor_class,
            ))
            .id();
        let rat = app
            .world_mut()
            .spawn((
                MonsterIndex::from_index(0),
                ArmorClass(7),
                Speed(110),
                CellCoord::new(4, 2),
                NextTurn { at: i64::MAX },
                HitPoints { current: 3, max: 3 },
            ))
            .id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(4, 2)));

        for _ in 0..100 {
            app.update();
            if app.world().get_entity(rat).is_err() {
                break;
            }
            assert_ne!(
                app.world().get::<CellCoord>(player).unwrap(),
                &CellCoord::new(4, 2),
                "the pursuit never steps onto the target's cell"
            );
        }
        assert!(
            app.world().get_entity(rat).is_err(),
            "the rat is slain within the frame budget"
        );
        let requests = app.world().resource::<Requests>();
        assert_eq!(
            requests.0,
            vec![Damage {
                target: rat,
                source: Some(player),
                amount: 4
            }],
            "one attack of 1 + damage bonus 3 lands, attributed to the driver"
        );

        // The attack cleared the order: the driver stands ready — the
        // clock sweeps onto the driver's unspent turn and freezes there.
        assert!(app.world().get::<Order>(player).is_none());
        app.update();
        let now = app.world().resource::<WorldClock>().now;
        assert_eq!(now, 200, "the clock sits on the driver's unspent turn");
        app.update();
        assert_eq!(
            app.world().resource::<WorldClock>().now,
            now,
            "the world holds for input"
        );
    }

    /// A retarget landing on the very frame the next step is
    /// planned: the command lands in the `Command` phase, so planning
    /// reads the landed order. Wired with the real phase topology — the
    /// sync points between phases are the point of this test.
    #[test]
    fn retarget_on_the_planning_frame_follows_the_new_order() {
        use crate::core::core_phase::CorePhase;
        use crate::core::world_clock::systems::advance::advance;

        let mut app = App::new();
        app.insert_resource(WorldClock { now: 0 })
            .insert_resource(terrain_registry())
            .insert_resource(CurrentMap::new(room()))
            .add_message::<TargetCell>()
            .configure_sets(
                Update,
                (
                    CorePhase::Advance,
                    CorePhase::Command,
                    CorePhase::PlayerPlan,
                    CorePhase::PlayerAct,
                )
                    .chain(),
            )
            .add_systems(Update, advance.in_set(CorePhase::Advance))
            .add_systems(Update, execute.in_set(CorePhase::Command))
            .add_systems(Update, plan_action.in_set(CorePhase::PlayerPlan))
            .add_systems(Update, act_move.in_set(CorePhase::PlayerAct));
        // Walking east toward (6, 2); the retarget, due south, lands on
        // the very frame the next step is planned.
        let player = app
            .world_mut()
            .spawn((
                Player,
                WorldDriver,
                CellCoord::new(2, 2),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(6, 2)));
        app.update();
        assert_eq!(
            app.world().get::<CellCoord>(player).unwrap(),
            &CellCoord::new(3, 2),
            "the first hop follows the first order"
        );

        app.world_mut()
            .write_message(TargetCell(CellCoord::new(2, 4)));
        let expected: VecDeque<CellCoord> = find_path(
            &room(),
            &terrain_registry(),
            CellCoord::new(3, 2),
            CellCoord::new(2, 4),
            &HashSet::new(),
        )
        .expect("a route to the new target exists");
        app.update();
        assert_eq!(
            *app.world().get::<CellCoord>(player).unwrap(),
            *expected.front().unwrap(),
            "the hop after the retarget follows the new order"
        );
        app.update();
        assert_eq!(
            *app.world().get::<CellCoord>(player).unwrap(),
            CellCoord::new(2, 4),
            "the new target is reached"
        );
        app.update();
        assert!(
            app.world().get::<Order>(player).is_none(),
            "arrived: order cleared"
        );
    }

    /// A target beyond a monster routes around it: the blocker is
    /// never attacked, never hurt, and the player reaches the target.
    #[test]
    fn a_fading_corpse_s_cell_is_ground_to_walk_on() {
        // The corpse lingers through its death fade; its cell must
        // read as ground, not as a monster's cell — the click walks,
        // it does not attack.
        let mut app = app();
        let player_cell = CellCoord::new(1, 1);
        app.world_mut().spawn((Player, player_cell));
        let corpse_cell = CellCoord::new(2, 1);
        app.world_mut().spawn((
            MonsterIndex::from_index(0),
            corpse_cell,
            crate::core::health::components::dead::Dead,
        ));
        app.world_mut().write_message(TargetCell(corpse_cell));
        app.update();
        let mut orders = app.world_mut().query_filtered::<&Order, With<Player>>();
        let order = orders.single(app.world()).unwrap();
        assert_eq!(
            order,
            &Order::Move {
                target: corpse_cell
            },
            "the corpse's cell issues a move order, not an attack"
        );
    }

    #[test]
    fn a_target_beyond_a_monster_routes_around_it() {
        use crate::core::combat::components::combat_bonuses::CombatBonuses;
        use crate::core::health::components::hit_points::HitPoints;
        use crate::core::health::messages::damage::Damage;
        use crate::core::health::messages::damaged::Damaged;
        use crate::core::health::systems::apply_damage::apply_damage;
        use crate::core::player::components::order::Order;
        use crate::core::rng::resources::game_rng::GameRng;
        use crate::core::world_clock::systems::advance::advance;

        let mut app = App::new();
        app.insert_resource(WorldClock { now: 0 })
            .insert_resource(GameRng::seeded(7))
            .insert_resource(terrain_registry())
            .insert_resource(CurrentMap::new(room()))
            .add_message::<TargetCell>()
            .add_message::<Damage>()
            .add_message::<Damaged>()
            .configure_sets(
                Update,
                (
                    CorePhase::Advance,
                    CorePhase::Command,
                    CorePhase::PlayerPlan,
                    CorePhase::PlayerAct,
                    CorePhase::PlayerResolve,
                )
                    .chain(),
            )
            .add_systems(Update, advance.in_set(CorePhase::Advance))
            .add_systems(Update, execute.in_set(CorePhase::Command))
            .add_systems(Update, plan_action.in_set(CorePhase::PlayerPlan))
            .add_systems(Update, act_move.in_set(CorePhase::PlayerAct))
            .add_systems(Update, apply_damage.in_set(CorePhase::PlayerResolve))
            .add_systems(
                Update,
                crate::core::health::systems::despawn_dead::despawn_dead.in_set(CorePhase::Derive),
            );
        let player = app
            .world_mut()
            .spawn((
                Player,
                WorldDriver,
                CellCoord::new(2, 2),
                Speed(110),
                NextTurn::default(),
                CombatBonuses {
                    hit: 5,
                    damage: 3,
                    armor: 0,
                },
            ))
            .id();
        // A never-due monster: a static blocker on the straight line.
        let blocker = app
            .world_mut()
            .spawn((
                MonsterIndex::from_index(0),
                Speed(110),
                CellCoord::new(3, 2),
                NextTurn { at: i64::MAX },
                HitPoints { current: 3, max: 3 },
            ))
            .id();
        app.world_mut()
            .write_message(TargetCell(CellCoord::new(5, 2)));

        let mut attacks_seen = 0;
        for _ in 0..100 {
            app.update();
            attacks_seen += app
                .world_mut()
                .resource_mut::<Messages<Damage>>()
                .drain()
                .count();
            if app.world().get::<CellCoord>(player).unwrap() == &CellCoord::new(5, 2) {
                break;
            }
        }
        assert_eq!(
            app.world().get::<CellCoord>(player).unwrap(),
            &CellCoord::new(5, 2),
            "the player reaches the target around the blocker"
        );
        app.update();
        assert_eq!(attacks_seen, 0, "a floor target never starts a fight");
        assert!(
            app.world().get_entity(blocker).is_ok(),
            "the blocker survives"
        );
        assert_eq!(
            app.world().get::<HitPoints>(blocker).unwrap().current,
            3,
            "the blocker is untouched"
        );
        assert!(
            app.world().get::<Order>(player).is_none(),
            "arrival cleared the order"
        );
    }
}
