//! Mouse input: translates the primary click into `PrimaryAction` gestures.

use bevy::prelude::*;

use crate::core::hero::events::primary_action::PrimaryAction;
use crate::graphic::camera::components::main_camera::MainCamera;
use crate::graphic::map::utils::coords::world_to_cell;

/// Left button = primary action on the pointed cell. Screen -> world ->
/// cell conversion needs the camera and pixel constants, which live on the
/// display side — the input plugin depends on the graphic plugin for
/// projection. What the action means is entirely the core's business.
pub fn mouse_primary_action(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut gestures: MessageWriter<PrimaryAction>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = camera.into_inner();
    let Ok(world) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };
    gestures.write(PrimaryAction(world_to_cell(world)));
}
