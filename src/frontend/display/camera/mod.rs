//! The camera rig: the canvas camera renders the world into the canvas,
//! the screen camera presents it, and the follow system keeps the rig
//! on the pixel grid while scrolling smoothly. Cameras are
//! presentation; a text display would have none.

pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the camera domain: the cameras are display infrastructure,
/// spawned in `Startup`; target following runs in the Camera phase.
pub fn register(app: &mut App) {
    app.add_systems(
        Startup,
        (
            entities::canvas_camera::spawn_canvas_camera,
            entities::screen_camera::spawn_screen_camera,
        ),
    )
    .add_systems(
        Update,
        systems::attach_target::attach_target.in_set(DisplayPhase::Attach),
    )
    .add_systems(
        Update,
        systems::follow_target::follow_target.in_set(DisplayPhase::Camera),
    );
}
