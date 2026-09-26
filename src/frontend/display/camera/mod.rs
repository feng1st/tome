// TODO: pending cleanup review — remove once stabilized
//! The gameplay camera: spawn and target following. Camera is presentation;
//! a text display would have none.

pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the camera domain: the camera is display infrastructure,
/// spawned in `Startup`; target following runs in the Camera phase.
pub fn register(app: &mut App) {
    app.add_systems(Startup, entities::camera::spawn_camera)
        .add_systems(
            Update,
            systems::follow_target::follow_target.in_set(DisplayPhase::Camera),
        );
}
