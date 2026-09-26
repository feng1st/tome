//! The hero domain's display side: only hero-specific glue lives here
//! (today: the camera target). Generic creature appearance — sprite,
//! anim table, loading — belongs to the appearance domain.

pub mod systems;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the hero display domain.
pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        systems::attach_camera_target::attach_camera_target.in_set(DisplayPhase::Attach),
    );
}
