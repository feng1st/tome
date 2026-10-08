//! Plan action: the driver's turn action from its standing order — one
//! attack at an adjacent target, or one step along a freshly computed
//! route. The order persists; the route never does.

use std::collections::HashSet;

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::action::components::attack_action::AttackAction;
use crate::core::action::components::move_action::MoveAction;
use crate::core::health::components::dead::Dead;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::utils::adjacency::adjacent;
use crate::core::map::utils::pathfinding::find_path;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::player::components::order::Order;
use crate::core::player::components::player::Player;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::action_duration::STANDARD_ACTION_DURATION;
use crate::core::speed::utils::action_duration::action_duration;
use crate::core::world_clock::components::next_turn::NextTurn;
use crate::core::world_clock::resources::world_clock::WorldClock;

/// The player's order-planning state: position, speed for pricing, the
/// persistent next turn to push forward, and the standing order.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerQuery {
    entity: Entity,
    cell: &'static CellCoord,
    speed: &'static Speed,
    next_turn: &'static mut NextTurn,
    order: Option<&'static Order>,
}

/// On a due turn, plan one action from the standing order. An attack
/// order attacks when the target is adjacent — one order, one attack —
/// and otherwise paths toward the target's current cell, following a
/// wandering target. A move order paths toward its target. Both route
/// with monster cells as obstacles, the destination cell exempt, and
/// both clear without spending the turn when they cannot proceed:
/// arrival, no route, or a final step onto an occupied target. With no
/// standing order the turn is not spent — the world stops on the
/// driver's unspent turn. A dead driver plans nothing: the query
/// filters the death marker out.
///
/// Planning never waits for pictures. A new target can land while another
/// creature's picture is mid-move — the action plans at the frozen tick
/// all the same and the pictures overlap. Only the tick ledger is
/// serial; overlapping pictures are a display concern.
/// The monster filter — living monsters as route obstacles and
/// order targets: a corpse neither blocks a route nor keeps an
/// attack order alive.
type MonsterFilter = (With<MonsterIndex>, Without<Dead>);

pub fn plan_action(
    mut commands: Commands,
    world_clock: Res<WorldClock>,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    monsters: Query<(Entity, &CellCoord), MonsterFilter>,
    mut player: Query<PlayerQuery, (With<Player>, Without<Dead>)>,
) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };
    if world_clock.now < player.next_turn.at {
        return;
    }
    match player.order.copied() {
        Some(Order::Attack { target }) => {
            // The target's current cell drives the whole order: attack when
            // adjacent, otherwise pursue.
            let Some(target_cell) = monsters.get(target).ok().map(|(_, cell)| *cell) else {
                // The order's target cannot die while it stands — the
                // attack is the only damage source, and a landed blow on
                // the holder clears the order — but a gone target simply
                // drops the order.
                commands.entity(player.entity).remove::<Order>();
                return;
            };
            if adjacent(*player.cell, target_cell) {
                player.next_turn.at =
                    world_clock.now + action_duration(STANDARD_ACTION_DURATION, player.speed.0);
                commands
                    .entity(player.entity)
                    .insert(AttackAction { target })
                    .remove::<Order>();
                return;
            }
            let obstacles: HashSet<CellCoord> = monsters
                .iter()
                .map(|(_, cell)| *cell)
                .filter(|cell| *cell != target_cell)
                .collect();
            let Some(route) = find_path(
                current_map.map(),
                &terrain_registry,
                *player.cell,
                target_cell,
                &obstacles,
            ) else {
                commands.entity(player.entity).remove::<Order>();
                return;
            };
            let Some(&step) = route.front() else {
                commands.entity(player.entity).remove::<Order>();
                return;
            };
            player.next_turn.at =
                world_clock.now + action_duration(STANDARD_ACTION_DURATION, player.speed.0);
            commands
                .entity(player.entity)
                .insert(MoveAction { target: step });
        }
        Some(Order::Move { target }) => {
            if *player.cell == target {
                commands.entity(player.entity).remove::<Order>();
                return;
            }
            let obstacles: HashSet<CellCoord> = monsters.iter().map(|(_, cell)| *cell).collect();
            let Some(route) = find_path(
                current_map.map(),
                &terrain_registry,
                *player.cell,
                target,
                &obstacles,
            ) else {
                commands.entity(player.entity).remove::<Order>();
                return;
            };
            let Some(&step) = route.front() else {
                commands.entity(player.entity).remove::<Order>();
                return;
            };
            // The route ends on the target even when a monster stands
            // there; the final step onto it is refused and the order clears.
            if monsters.iter().any(|(_, cell)| *cell == step) {
                commands.entity(player.entity).remove::<Order>();
                return;
            }
            player.next_turn.at =
                world_clock.now + action_duration(STANDARD_ACTION_DURATION, player.speed.0);
            commands
                .entity(player.entity)
                .insert(MoveAction { target: step });
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;
    use crate::core::map::types::local_map::LocalMap;
    use crate::core::world_clock::components::world_driver::WorldDriver;

    const TERRAINS: &str = r#"[
        ( terrain: "floor", flags: ["PASSABLE"] ),
        ( terrain: "wall",  flags: [] ),
    ]"#;

    fn terrain_registry() -> TerrainRegistry {
        parse_terrain_registry("test", TERRAINS)
    }

    fn map_from(rows: &[&str]) -> LocalMap {
        let rows: Vec<String> = rows.iter().map(|r| format!("\"{r}\"")).collect();
        let doc = format!(
            "( legend: {{ '#': \"wall\", '.': \"floor\" }}, rows: [ {} ], )",
            rows.join(", ")
        );
        parse_local_map("test", &doc, &terrain_registry())
    }

    fn open_room() -> LocalMap {
        map_from(&[
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
        ])
    }

    fn world_with(room: LocalMap) -> World {
        let mut world = World::new();
        world.insert_resource(WorldClock { now: 0 });
        world.insert_resource(CurrentMap::new(room));
        world.insert_resource(terrain_registry());
        world
    }

    fn rat(world: &mut World, cell: CellCoord) -> Entity {
        world.spawn((MonsterIndex::from_index(0), cell)).id()
    }

    #[test]
    fn due_player_with_a_move_order_plans_a_step() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
                Order::Move {
                    target: CellCoord::new(3, 1),
                },
            ))
            .id();
        world.run_system_once(plan_action).unwrap();
        assert_eq!(
            world.get::<MoveAction>(player).map(|m| m.target),
            Some(CellCoord::new(2, 1)),
            "the step heads for the target"
        );
        assert_eq!(
            world.get::<NextTurn>(player).unwrap().at,
            100,
            "the turn is spent: next due at now + duration"
        );
        assert!(
            world.get::<Order>(player).is_some(),
            "the order survives a step"
        );
    }

    #[test]
    fn slot_not_due_plans_nothing() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn { at: 50 },
                Order::Move {
                    target: CellCoord::new(3, 1),
                },
            ))
            .id();
        world.run_system_once(plan_action).unwrap();
        assert!(world.get::<MoveAction>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 50);
    }

    #[test]
    fn planning_never_waits_for_pictures() {
        use crate::core::display::components::is_moving::IsMoving;

        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn { at: 100 },
                Order::Move {
                    target: CellCoord::new(2, 1),
                },
            ))
            .id();
        // Due on the unspent turn at 100, order standing, while a
        // creature whose turn tied this tick is mid-move. The plan
        // lands at the frozen tick all the same: pictures may overlap;
        // only the tick ledger is serial.
        world.resource_mut::<WorldClock>().now = 100;
        world.spawn((NextTurn { at: 140 }, IsMoving));
        world.run_system_once(plan_action).unwrap();
        assert_eq!(
            world.get::<MoveAction>(player).map(|m| m.target),
            Some(CellCoord::new(2, 1)),
            "due with a standing order: planning does not wait for pictures"
        );
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 200);
    }

    #[test]
    fn no_order_plans_nothing() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        world.run_system_once(plan_action).unwrap();
        assert!(world.get::<MoveAction>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 0);
    }

    #[test]
    fn arrival_clears_the_order_without_spending() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(3, 1),
                Speed(110),
                NextTurn::default(),
                Order::Move {
                    target: CellCoord::new(3, 1),
                },
            ))
            .id();
        world.run_system_once(plan_action).unwrap();
        assert!(
            world.get::<Order>(player).is_none(),
            "arrived: order cleared"
        );
        assert!(world.get::<MoveAction>(player).is_none());
        assert_eq!(
            world.get::<NextTurn>(player).unwrap().at,
            0,
            "no action, no spend"
        );
    }

    #[test]
    fn an_unreachable_target_clears_the_order_without_spending() {
        let mut world = world_with(map_from(&["#####", "#.#.#", "#.#.#", "#.#.#", "#####"]));
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
                Order::Move {
                    target: CellCoord::new(3, 1),
                },
            ))
            .id();
        world.run_system_once(plan_action).unwrap();
        assert!(
            world.get::<Order>(player).is_none(),
            "no route: order cleared"
        );
        assert!(world.get::<MoveAction>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 0, "no spend");
    }

    #[test]
    fn the_step_routes_around_a_monster() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
                Order::Move {
                    target: CellCoord::new(3, 1),
                },
            ))
            .id();
        let blocker = rat(&mut world, CellCoord::new(2, 1)); // the straight line
        world.run_system_once(plan_action).unwrap();
        let step = world.get::<MoveAction>(player).unwrap().target;
        assert_ne!(step, CellCoord::new(2, 1), "the monster's cell is avoided");
        assert!(adjacent(CellCoord::new(1, 1), step), "one step at a time");
        let _ = blocker;
    }

    #[test]
    fn the_target_blocked_by_a_monster_clears_beside_it() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
                Order::Move {
                    target: CellCoord::new(2, 1),
                },
            ))
            .id();
        let _ = rat(&mut world, CellCoord::new(2, 1));
        world.run_system_once(plan_action).unwrap();
        assert!(
            world.get::<Order>(player).is_none(),
            "an occupied target clears the order instead of stepping on"
        );
        assert!(world.get::<MoveAction>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 0, "no spend");
    }

    #[test]
    fn an_adjacent_target_attacks_and_clears_the_order() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        let rat = rat(&mut world, CellCoord::new(2, 1));
        world
            .entity_mut(player)
            .insert(Order::Attack { target: rat });
        world.run_system_once(plan_action).unwrap();
        assert_eq!(
            world.get::<AttackAction>(player).map(|a| a.target),
            Some(rat),
            "adjacent: the attack is planned"
        );
        assert!(
            world.get::<Order>(player).is_none(),
            "one order, one attack"
        );
        assert_eq!(
            world.get::<NextTurn>(player).unwrap().at,
            100,
            "the turn is spent"
        );
    }

    #[test]
    fn pursuit_steps_toward_the_target_and_follows_it() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        let rat = rat(&mut world, CellCoord::new(4, 1));
        world
            .entity_mut(player)
            .insert(Order::Attack { target: rat });
        world.run_system_once(plan_action).unwrap();
        let first = world.get::<MoveAction>(player).unwrap().target;
        assert!(adjacent(CellCoord::new(1, 1), first));

        // The rat wanders away mid-pursuit: the next due turn heads for
        // the new cell, closing distance to it rather than to the old
        // one.
        world.entity_mut(rat).insert(CellCoord::new(4, 3));
        world.resource_mut::<WorldClock>().now = 100;
        world.entity_mut(player).insert(NextTurn { at: 100 });
        world.run_system_once(plan_action).unwrap();
        let second = world.get::<MoveAction>(player).unwrap().target;
        let distance = |a: CellCoord, b: CellCoord| (a.x - b.x).abs().max((a.y - b.y).abs());
        assert!(
            distance(second, CellCoord::new(4, 3))
                < distance(CellCoord::new(1, 1), CellCoord::new(4, 3)),
            "the pursuit follows the target's current cell"
        );
        assert!(
            world.get::<Order>(player).is_some(),
            "pursuit keeps the order"
        );
    }

    #[test]
    fn pursuit_routes_around_a_third_monster() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        let target = rat(&mut world, CellCoord::new(4, 1));
        let blocker = rat(&mut world, CellCoord::new(2, 1));
        world.entity_mut(player).insert(Order::Attack { target });
        world.run_system_once(plan_action).unwrap();
        let step = world.get::<MoveAction>(player).unwrap().target;
        assert_ne!(
            step,
            CellCoord::new(2, 1),
            "only the target's cell is exempt"
        );
        let _ = blocker;
    }

    #[test]
    fn an_unreachable_attack_target_clears_the_order_without_spending() {
        // Two chambers split by a full wall column: the rat stands in
        // the far one.
        let mut world = world_with(map_from(&["#####", "#.#.#", "#.#.#", "#.#.#", "#####"]));
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        let rat = world
            .spawn((MonsterIndex::from_index(0), CellCoord::new(3, 1)))
            .id();
        world
            .entity_mut(player)
            .insert(Order::Attack { target: rat });
        world.run_system_once(plan_action).unwrap();
        assert!(
            world.get::<Order>(player).is_none(),
            "no route: order cleared"
        );
        assert!(world.get::<MoveAction>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 0, "no spend");
    }

    /// The order's target has left the world (death despawns it): the
    /// order clears and the turn is not spent.
    #[test]
    fn a_gone_target_clears_the_order_without_spending() {
        let mut world = world_with(open_room());
        let player = world
            .spawn((
                Player,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn::default(),
            ))
            .id();
        let rat = rat(&mut world, CellCoord::new(4, 1));
        world
            .entity_mut(player)
            .insert(Order::Attack { target: rat });
        world.despawn(rat);
        world.run_system_once(plan_action).unwrap();
        assert!(
            world.get::<Order>(player).is_none(),
            "target gone: order cleared"
        );
        assert!(world.get::<MoveAction>(player).is_none());
        assert!(world.get::<AttackAction>(player).is_none());
        assert_eq!(world.get::<NextTurn>(player).unwrap().at, 0, "no spend");
    }

    /// A dead driver plans nothing: the standing order is never spent,
    /// so the clock pins at the driver's due turn and a monster whose
    /// turn lies beyond never comes due — the world freezes.
    #[test]
    fn a_dead_driver_freezes_the_world_on_its_unspent_turn() {
        use crate::core::world_clock::systems::advance::advance;

        let mut app = App::new();
        app.insert_resource(WorldClock { now: 0 })
            .insert_resource(terrain_registry())
            .insert_resource(CurrentMap::new(open_room()))
            .add_systems(Update, (advance, plan_action).chain());
        let player = app
            .world_mut()
            .spawn((
                Player,
                WorldDriver,
                CellCoord::new(1, 1),
                Speed(110),
                NextTurn { at: 10 },
                Order::Move {
                    target: CellCoord::new(2, 1),
                },
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
        assert!(app.world().get::<MoveAction>(player).is_none());
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
