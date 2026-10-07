//! Mouse device translation: every mouse control (buttons today, wheel and
//! double-click later) is translated here, so click/double-click
//! disambiguation and other device-level timing stay in one place. The
//! pointer reports what it hit; it never decides what it means.

use bevy::prelude::*;

use crate::core::player::commands::target_cell::TargetCell;
use crate::frontend::display::camera::components::main_camera::MainCamera;
use crate::frontend::display::camera::utils::coords::window_to_world;
use crate::frontend::display::map::utils::coords::world_to_cell;

/// Control bindings are literals today (left button = primary action); when
/// the bindings table lands they become lookups, and this system's path and
/// signature stay unchanged. The world renders straight to the window, so
/// the cursor converts window -> world -> cell: `window_to_world` (the
/// camera domain owns the rule). The ground is the only hittable target
/// today, so every press becomes `TargetCell` and the core derives what
/// it means — a walk or an attack.
pub fn translate(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<&Transform, With<MainCamera>>,
    mut targets: MessageWriter<TargetCell>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let world = window_to_world(cursor, &window, camera.translation.truncate());
    targets.write(TargetCell(world_to_cell(world)));
}
