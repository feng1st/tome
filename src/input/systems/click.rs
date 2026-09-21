//! Translates left mouse clicks into `MoveTo` intents.

use bevy::prelude::*;

use crate::core::hero::events::move_to::MoveTo;
use crate::graphic::camera::components::main_camera::MainCamera;
use crate::graphic::map::utils::coords::world_to_cell;

/// Left click: emit the clicked cell as a `MoveTo` intent. Screen -> world
/// -> cell conversion needs the camera and pixel constants, which live on
/// the display side — the input plugin depends on the graphic plugin for
/// projection. What the intent causes is entirely the core's business.
pub fn handle_click(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut events: MessageWriter<MoveTo>,
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
    events.write(MoveTo(world_to_cell(world)));
}
