//! Mouse device translation: every mouse control (buttons today, wheel and
//! double-click later) is translated here, so click/double-click
//! disambiguation and other device-level timing stay in one place.
//! Modalities report what the pointer hit; they never decide what it means.

use bevy::prelude::*;

use crate::frontend::display::camera::components::canvas_camera::CanvasCamera;
use crate::frontend::display::canvas::utils::coords::window_to_canvas;
use crate::frontend::display::map::utils::coords::world_to_cell;
use crate::frontend::input::gestures::primary_action_on_cell::PrimaryActionOnCell;

/// Control bindings are literals today (left button = primary action); when
/// the bindings table lands they become lookups, and this system's path and
/// signature stay unchanged. The world renders into the canvas, so the
/// cursor converts window -> canvas -> world -> cell: `window_to_canvas`
/// (the canvas domain owns the rule) puts the cursor into the canvas
/// camera's viewport coordinates. The ground is the only hittable target
/// today, so every press becomes `PrimaryActionOnCell`; sprite-mask hit
/// testing (a picking backend) arrives with the first clickable monster.
pub fn translate(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<CanvasCamera>>,
    mut gestures: MessageWriter<PrimaryActionOnCell>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = camera.into_inner();
    let canvas = window_to_canvas(cursor);
    let Ok(world) = camera.viewport_to_world_2d(camera_transform, canvas) else {
        return;
    };
    gestures.write(PrimaryActionOnCell(world_to_cell(world)));
}
