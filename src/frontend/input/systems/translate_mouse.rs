use bevy::prelude::*;

use crate::core::player::commands::target_cell::TargetCell;
use crate::frontend::display::camera::components::main_camera::MainCamera;
use crate::frontend::display::camera::utils::coords::window_to_world_position;
use crate::frontend::display::map::utils::coords::world_position_to_cell_coord;

pub fn translate_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<&Transform, With<MainCamera>>,
    mut target_cell_writer: MessageWriter<TargetCell>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(window_position) = window.cursor_position() else {
        return;
    };
    let world_position =
        window_to_world_position(window_position, &window, camera.translation.truncate());
    target_cell_writer.write(TargetCell(world_position_to_cell_coord(world_position)));
}
