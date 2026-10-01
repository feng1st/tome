//! The camera: a single direct-to-window camera snapped to the screen
//! grid, plus the presentation snap every rendered entity passes
//! through. Cameras are presentation; a text display would have none.

pub mod components;
pub mod entities;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the camera domain: the camera is display infrastructure,
/// spawned in `Startup`; the snaps register into their phases (the
/// phase chain at the side root orders camera before presentation).
pub fn register(app: &mut App) {
    app.add_systems(Startup, entities::main_camera::spawn_main_camera)
        .add_systems(
            Update,
            systems::attach_target::attach_target.in_set(DisplayPhase::Attach),
        )
        .add_systems(
            Update,
            systems::snap_camera::snap_camera.in_set(DisplayPhase::Camera),
        )
        .add_systems(
            Update,
            systems::snap_sprites::snap_sprites.in_set(DisplayPhase::Snap),
        );
}
