//! The camera: a single direct-to-window camera snapped to the screen
//! grid, plus the presentation snap every rendered entity passes
//! through. Cameras are presentation; a text display would have none.

pub mod components;
pub mod entities;
pub mod messages;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use self::messages::shake_camera::ShakeCamera;
use self::resources::camera_shake::CameraShake;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the camera domain: the camera is display infrastructure,
/// spawned in `Startup`; the shake request message and state exist, and
/// the snaps register into their phases (the phase chain at the side
/// root orders camera before presentation). The shake advances ahead
/// of the follow snap within the Camera phase — the snap consumes this
/// frame's offset.
pub fn register(app: &mut App) {
    app.add_message::<ShakeCamera>()
        .init_resource::<CameraShake>()
        .add_systems(Startup, entities::main_camera::spawn_main_camera)
        .add_systems(
            Update,
            systems::attach_target::attach_target.in_set(DisplayPhase::Attach),
        )
        .add_systems(
            Update,
            (
                systems::update_shake::update_shake,
                systems::snap_camera::snap_camera,
            )
                .chain()
                .in_set(DisplayPhase::Camera),
        )
        .add_systems(
            Update,
            systems::snap_sprites::snap_sprites.in_set(DisplayPhase::Snap),
        );
}
