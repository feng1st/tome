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
    use crate::core::map::types::cell_coord::CellCoord;
    use crate::core::map::types::grid_map::GridMap;

    fn app() -> App {
        let mut app = App::new();
        app.add_message::<MoveToCell>()
            .insert_resource(CurrentMap::new(GridMap::demo_room()))
            .add_systems(Update, execute);
        app
    }

    #[test]
    fn unwalkable_goal_produces_no_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position::from(CellCoord::new(10, 10))))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellCoord::new(30, 30))); // water pool
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
}
