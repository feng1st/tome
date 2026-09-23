//! Executes `MoveToCell` commands: validation (walkability, reachability)
//! and pathfinding live here in the core — the frontend asks, the core
//! decides.

use bevy::prelude::*;

use crate::core::hero::commands::move_to_cell::MoveToCell;
use crate::core::hero::components::hero::Hero;
use crate::core::map::resources::grid_map::GridMap;
use crate::core::map::utils::pathfinding::find_path;
use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;

/// Execute every `MoveToCell` command: an unwalkable or unreachable target
/// is ignored entirely; a new goal mid-walk re-paths from the cell the
/// hero currently stands in.
pub fn execute(
    mut commands: Commands,
    mut move_commands: MessageReader<MoveToCell>,
    grid_map: Res<GridMap>,
    hero: Query<(Entity, &Position), With<Hero>>,
) {
    let Ok((hero_entity, pos)) = hero.single() else {
        return;
    };
    for command in move_commands.read() {
        let goal = command.0;
        let start = pos.cell();
        if goal == start || !grid_map.walkable(goal) {
            continue;
        }
        let Some(cells) = find_path(&grid_map, start, goal) else {
            continue;
        };
        commands.entity(hero_entity).insert(Path::new(cells, pos.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::cell_pos::CellPos;

    fn app() -> App {
        let mut app = App::new();
        app.add_message::<MoveToCell>()
            .insert_resource(GridMap::demo_room())
            .add_systems(Update, execute);
        app
    }

    #[test]
    fn unwalkable_goal_produces_no_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position(CellPos::new(10, 10).as_vec2())))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellPos::new(30, 30))); // water pool
        app.update();
        assert!(app.world().get::<Path>(hero).is_none());
    }

    #[test]
    fn walkable_goal_attaches_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position(CellPos::new(10, 10).as_vec2())))
            .id();
        app.world_mut()
            .write_message(MoveToCell(CellPos::new(12, 10)));
        app.update();
        let path = app.world().get::<Path>(hero).expect("path attached");
        assert_eq!(*path.cells.back().unwrap(), CellPos::new(12, 10));
    }
}
