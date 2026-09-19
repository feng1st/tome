use bevy::prelude::*;

use crate::camera::components::main_camera::MainCamera;
use crate::hero::components::hero::Hero;
use crate::map::resources::grid_map::GridMap;
use crate::map::utils::pathfinding::find_path;
use crate::movement::components::path::Path;

/// Left click: pick the clicked walkable cell as the movement goal.
///
/// Unwalkable clicks (wall, water, outside) are ignored entirely; a click
/// mid-walk replaces the current path from the cell the hero is in.
pub fn handle_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    grid_map: Res<GridMap>,
    hero: Query<(Entity, &Transform), With<Hero>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok((hero_entity, hero_transform)) = hero.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = camera.into_inner();
    let Ok(world) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };
    // Screen -> world -> cell. The GridMap is the single source of truth for
    // walkability; the click never touches the rendered chunks.
    let goal = GridMap::world_to_cell(world);
    if !grid_map.walkable(goal) {
        return;
    }
    let start = GridMap::world_to_cell(hero_transform.translation.xy());
    if goal == start {
        return;
    }
    let Some(cells) = find_path(&grid_map, start, goal) else {
        return;
    };
    commands
        .entity(hero_entity)
        .insert(Path::new(cells, hero_transform.translation.xy()));
}
