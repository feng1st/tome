//! Executes `MoveToCell` commands: validation (walkability, reachability)
//! and pathfinding live here in the core — the frontend asks, the core
//! decides.

use bevy::prelude::*;

use crate::core::hero::commands::move_to_cell::MoveToCell;
use crate::core::hero::components::hero::Hero;
use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain::Terrain;
use crate::core::map::utils::pathfinding::find_path;
use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;

/// Execute every `MoveToCell` command: an unwalkable or unreachable target
/// is ignored entirely; a new goal mid-walk re-paths from the cell the
/// hero currently stands in.
pub fn execute(
    mut commands: Commands,
    mut move_commands: MessageReader<MoveToCell>,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    hero: Query<(Entity, &Position), With<Hero>>,
) {
    let Ok((hero_entity, pos)) = hero.single() else {
        return;
    };
    let local_map = current_map.map();
    for command in move_commands.read() {
        let goal = command.0;
        let start = pos.cell();
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
        commands.entity(hero_entity).insert(Path::new(cells, *pos));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;
    use crate::core::map::types::cell_coord::CellCoord;
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
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(2, 2))))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(6, 4))); // water pool
        app.update();
        assert!(app.world().get::<Path>(hero).is_none());
    }

    #[test]
    fn wall_and_out_of_bounds_goals_produce_no_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(2, 2))))
            .id();
        for goal in [
            CellCoord::new(0, 0),  // wall corner
            CellCoord::new(-1, 2), // left of the map
            CellCoord::new(16, 2), // right of the map
        ] {
            app.world_mut().write_message(MoveToCell(goal));
        }
        app.update();
        assert!(app.world().get::<Path>(hero).is_none());
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
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(1, 1))))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(3, 1)));
        app.update();
        assert!(app.world().get::<Path>(hero).is_none());
    }

    #[test]
    fn walkable_goal_attaches_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(2, 2))))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(4, 2)));
        app.update();
        let path = app.world().get::<Path>(hero).expect("path attached");
        assert_eq!(*path.cells.back().unwrap(), CellCoord::new(4, 2));
    }

    #[test]
    fn new_goal_mid_walk_repaths() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(2, 2))))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(4, 2)));
        app.update();
        // Mid-step: the replacement path must start from where the hero
        // currently is, not from the old path's cell.
        let mid_step = Position::new(2.5, 2.0);
        *app.world_mut().get_mut::<Position>(hero).unwrap() = mid_step;
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(6, 2)));
        app.update();
        let path = app.world().get::<Path>(hero).expect("path replaced");
        assert_eq!(path.step_from, mid_step);
        assert_eq!(*path.cells.front().unwrap(), CellCoord::new(4, 2));
        assert_eq!(*path.cells.back().unwrap(), CellCoord::new(6, 2));
    }
}
