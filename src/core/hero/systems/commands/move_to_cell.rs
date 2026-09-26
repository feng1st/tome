//! Executes `MoveToCell` commands: validation (walkability, reachability)
//! and pathfinding live here in the core — the frontend asks, the core
//! decides.

use bevy::prelude::*;

use crate::core::hero::commands::move_to_cell::MoveToCell;
use crate::core::hero::components::hero::Hero;
use crate::core::map::resources::current_map::CurrentMap;
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
    hero: Query<(Entity, &Position), With<Hero>>,
) {
    let Ok((hero_entity, pos)) = hero.single() else {
        return;
    };
    let map = current_map.map();
    for command in move_commands.read() {
        let goal = command.0;
        let start = pos.cell();
        if goal == start || !map.walkable(goal) {
            continue;
        }
        let Some(cells) = find_path(map, start, goal) else {
            continue;
        };
        commands.entity(hero_entity).insert(Path::new(cells, *pos));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::constants::layout::MAP_W;
    use crate::core::map::constants::tile_kind::TileKind;
    use crate::core::map::types::cell_coord::CellCoord;
    use crate::core::map::types::grid_map::GridMap;

    fn app_with(map: GridMap) -> App {
        let mut app = App::new();
        app.add_message::<MoveToCell>()
            .insert_resource(CurrentMap::new(map))
            .add_systems(Update, execute);
        app
    }

    fn app() -> App {
        app_with(GridMap::demo_room())
    }

    #[test]
    fn unwalkable_goal_produces_no_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(10, 10))))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(24, 19))); // water pool
        app.update();
        assert!(app.world().get::<Path>(hero).is_none());
    }

    #[test]
    fn wall_and_out_of_bounds_goals_produce_no_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(10, 10))))
            .id();
        for goal in [
            CellCoord::new(0, 0),             // wall corner
            CellCoord::new(-1, 10),           // left of the map
            CellCoord::new(MAP_W as i32, 10), // right of the map
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
        let mut tiles = vec![TileKind::Floor; 5 * 5];
        for y in 0..5 {
            for x in 0..5 {
                if x == 0 || y == 0 || x == 4 || y == 4 || x == 2 {
                    tiles[x + y * 5] = TileKind::Wall;
                }
            }
        }
        let mut app = app_with(GridMap {
            width: 5,
            height: 5,
            tiles,
        });
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
            .spawn((Hero, Position::from(CellCoord::new(10, 10))))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(12, 10)));
        app.update();
        let path = app.world().get::<Path>(hero).expect("path attached");
        assert_eq!(*path.cells.back().unwrap(), CellCoord::new(12, 10));
    }

    #[test]
    fn new_goal_mid_walk_repaths() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(10, 10))))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(12, 10)));
        app.update();
        // Mid-step: the replacement path must start from where the hero
        // currently is, not from the old path's cell.
        let mid_step = Position::new(10.5, 10.0);
        *app.world_mut().get_mut::<Position>(hero).unwrap() = mid_step;
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(14, 10)));
        app.update();
        let path = app.world().get::<Path>(hero).expect("path replaced");
        assert_eq!(path.step_from, mid_step);
        assert_eq!(*path.cells.front().unwrap(), CellCoord::new(12, 10));
        assert_eq!(*path.cells.back().unwrap(), CellCoord::new(14, 10));
    }
}
