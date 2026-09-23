//! Mouse left button: the primary pointer action. Modalities report what
//! the pointer hit; they never decide what it means.

use bevy::prelude::*;

use crate::frontend::graphic::camera::components::main_camera::MainCamera;
use crate::frontend::graphic::map::utils::coords::world_to_cell;
use crate::frontend::input::gestures::primary_action_on_cell::PrimaryActionOnCell;

/// Left button = primary action on the pointed target. Screen -> world ->
/// cell conversion needs the camera and pixel constants, which live in the
/// frontend's rendering side — modality and presentation ship together.
/// The ground is the only hittable target today, so every click becomes
/// `PrimaryActionOnCell`; sprite-mask hit testing (a picking backend)
/// arrives with the first clickable monster.
pub fn left_click(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut gestures: MessageWriter<PrimaryActionOnCell>,
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
    gestures.write(PrimaryActionOnCell(world_to_cell(world)));
}
