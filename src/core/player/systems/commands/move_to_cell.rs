//! Executes `MoveToCell` commands: validation (walkability, reachability)
//! and pathfinding live here in the core — the frontend asks, the core
//! decides.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain::Terrain;
use crate::core::map::utils::pathfinding::find_path;
use crate::core::movement::components::path::Path;
use crate::core::player::commands::move_to_cell::MoveToCell;
use crate::core::player::components::player::Player;

/// Execute every `MoveToCell` command: an unwalkable or unreachable target
/// is ignored entirely; a new goal mid-walk re-paths from the cell the
/// player currently stands in.
pub fn execute(
    mut commands: Commands,
    mut move_commands: MessageReader<MoveToCell>,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    player: Query<(Entity, &CellCoord), With<Player>>,
) {
    let Ok((player_entity, &start)) = player.single() else {
        return;
    };
    let local_map = current_map.map();
    for command in move_commands.read() {
        let goal = command.0;
        if goal == start {
            continue;
        }
        let goal_walkable = local_map
            .get(goal)
            .and_then(|terrain_index| terrain_registry.get(terrain_index))
            .is_some_and(Terrain::walkable);
        if !goal_walkable {
            continue;
        }
        let Some(cells) = find_path(local_map, &terrain_registry, start, goal) else {
            continue;
        };
        commands.entity(player_entity).insert(Path::new(cells));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;
    use crate::core::map::types::local_map::LocalMap;

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

    fn app_with(map: LocalMap) -> App {
        let mut app = App::new();
        app.add_message::<MoveToCell>()
            .insert_resource(terrain_registry())
            .insert_resource(CurrentMap::new(map))
            .add_systems(Update, execute);
        app
    }

    fn app() -> App {
        app_with(room())
    }

    #[test]
    fn unwalkable_goal_produces_no_path() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(6, 4))); // water pool
        app.update();
        assert!(app.world().get::<Path>(player).is_none());
    }

    #[test]
    fn wall_and_out_of_bounds_goals_produce_no_path() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        for goal in [
            CellCoord::new(0, 0),  // wall corner
            CellCoord::new(-1, 2), // left of the map
            CellCoord::new(16, 2), // right of the map
        ] {
            app.world_mut().write_message(MoveToCell(goal));
        }
        app.update();
        assert!(app.world().get::<Path>(player).is_none());
    }

    #[test]
    fn unreachable_goal_produces_no_path() {
        // Two chambers split by a full wall column: the goal is walkable
        // but unreachable — a distinct case from an unwalkable target.
        let doc = r######"(
            legend: { '#': "wall", '.': "floor" },
            rows: [ "#####", "#.#.#", "#.#.#", "#.#.#", "#####" ],
        )"######;
        let mut app = app_with(parse_local_map("test", doc, &terrain_registry()));
        let player = app.world_mut().spawn((Player, CellCoord::new(1, 1))).id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(3, 1)));
        app.update();
        assert!(app.world().get::<Path>(player).is_none());
    }

    #[test]
    fn walkable_goal_attaches_path() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(4, 2)));
        app.update();
        let path = app.world().get::<Path>(player).expect("path attached");
        assert_eq!(*path.cells.back().unwrap(), CellCoord::new(4, 2));
    }

    #[test]
    fn new_goal_mid_walk_repaths() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, CellCoord::new(2, 2))).id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(4, 2)));
        app.update();
        // Mid-walk: the replacement path starts from the cell the player
        // currently stands in (the logical cell flips at each step's start).
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(6, 2)));
        app.update();
        let path = app.world().get::<Path>(player).expect("path replaced");
        assert_eq!(*path.cells.front().unwrap(), CellCoord::new(3, 2));
        assert_eq!(*path.cells.back().unwrap(), CellCoord::new(6, 2));
    }

    /// A retarget click landing on the very frame the next step is
    /// planned: the command settles in the `Command` phase, so planning
    /// reads the landed path. Wired with the real phase topology — the
    /// sync points between phases are the point of this test.
    #[test]
    fn retarget_on_the_planning_frame_follows_the_new_path() {
        use std::collections::VecDeque;

        use crate::core::core_phase::CorePhase;
        use crate::core::movement::systems::act_move::act_move;
        use crate::core::player::systems::plan_move::plan_move;
        use crate::core::speed::components::speed::Speed;
        use crate::core::world_clock::components::next_turn::NextTurn;
        use crate::core::world_clock::components::world_driver::WorldDriver;
        use crate::core::world_clock::resources::world_clock::WorldClock;
        use crate::core::world_clock::systems::advance::advance;

        let mut app = App::new();
        app.add_message::<MoveToCell>()
            .init_resource::<WorldClock>()
            .insert_resource(terrain_registry())
            .insert_resource(CurrentMap::new(room()))
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
            .add_systems(Update, plan_move.in_set(CorePhase::PlayerPlan))
            .add_systems(Update, act_move.in_set(CorePhase::PlayerAct));
        // Walking east; the click (due south) lands on the planning frame.
        let player = app
            .world_mut()
            .spawn((
                Player,
                WorldDriver,
                CellCoord::new(2, 2),
                Speed(110),
                NextTurn::default(),
                Path::new(VecDeque::from([CellCoord::new(3, 2), CellCoord::new(4, 2)])),
            ))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(2, 4)));
        app.update();
        assert_eq!(
            app.world().get::<CellCoord>(player).unwrap(),
            &CellCoord::new(2, 3),
            "the first hop after a retarget follows the new path"
        );
        app.update();
        assert_eq!(
            app.world().get::<CellCoord>(player).unwrap(),
            &CellCoord::new(2, 4)
        );
        assert!(app.world().get::<Path>(player).is_none(), "path consumed");
    }
}
