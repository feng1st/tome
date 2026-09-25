//! Mouse device translation: every mouse control (buttons today, wheel and
//! double-click later) is translated here, so click/double-click
//! disambiguation and other device-level timing stay in one place.
//! Modalities report what the pointer hit; they never decide what it means.

use bevy::prelude::*;

use crate::frontend::display::camera::components::main_camera::MainCamera;
use crate::frontend::display::map::utils::coords::world_to_cell;
use crate::frontend::input::gestures::primary_action_on_local_map_cell::PrimaryActionOnLocalMapCell;

/// Control bindings are literals today (left button = primary action); when
/// the bindings table lands they become lookups, and this system's path and
/// signature stay unchanged. Screen -> world -> cell conversion needs the
/// camera and pixel constants, which live in the frontend's rendering
/// side — modality and presentation ship together. The ground is the only
/// hittable target today, so every press becomes `PrimaryActionOnLocalMapCell`;
/// sprite-mask hit testing (a picking backend) arrives with the first
/// clickable monster.
pub fn translate(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut gestures: MessageWriter<PrimaryActionOnLocalMapCell>,
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
    gestures.write(PrimaryActionOnLocalMapCell(world_to_cell(world)));
}
