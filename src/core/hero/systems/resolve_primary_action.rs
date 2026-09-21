//! Resolves primary actions: the contextual dispatch (move today; pick up,
//! attack, … grow here) lives in the core, where game state is available.
//! Input plugins emit gestures without knowing what they will mean.

use bevy::prelude::*;

use crate::core::hero::components::hero::Hero;
use crate::core::hero::events::primary_action::PrimaryAction;
use crate::core::map::resources::grid_map::GridMap;
use crate::core::map::utils::pathfinding::find_path;
use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;

/// Resolve every `PrimaryAction` gesture. Currently the only meaning is
/// movement: an unwalkable or unreachable target is ignored entirely, and
/// a new goal mid-walk re-paths from the cell the hero stands in.
pub fn resolve_primary_action(
    mut commands: Commands,
    mut gestures: MessageReader<PrimaryAction>,
    grid_map: Res<GridMap>,
    hero: Query<(Entity, &Position), With<Hero>>,
) {
    let Ok((hero_entity, pos)) = hero.single() else {
        return;
    };
    for gesture in gestures.read() {
        let goal = gesture.0;
        let start = pos.cell();
        if goal == start || !grid_map.walkable(goal) {
            continue;
        }
        let Some(cells) = find_path(&grid_map, start, goal) else {
            continue;
        };
        commands
            .entity(hero_entity)
            .insert(Path::new(cells, pos.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        let mut app = App::new();
        app.add_message::<PrimaryAction>()
            .insert_resource(GridMap::demo_room())
            .add_systems(Update, resolve_primary_action);
        app
    }

    #[test]
    fn unwalkable_goal_produces_no_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position(IVec2::new(10, 10).as_vec2())))
            .id();
        app.world_mut()
            .write_message(PrimaryAction(IVec2::new(30, 30))); // water pool
        app.update();
        assert!(app.world().get::<Path>(hero).is_none());
    }

    #[test]
    fn walkable_goal_attaches_path() {
        let mut app = app();
        let hero = app
            .world_mut()
            .spawn((Hero, Position(IVec2::new(10, 10).as_vec2())))
            .id();
        app.world_mut()
            .write_message(PrimaryAction(IVec2::new(12, 10)));
        app.update();
        let path = app.world().get::<Path>(hero).expect("path attached");
        assert_eq!(*path.cells.back().unwrap(), IVec2::new(12, 10));
    }
}
