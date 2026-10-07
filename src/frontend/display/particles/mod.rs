//! The particles domain: pixel particles — short-lived squares
//! sprayed in a cone, integrated under gravity, shrinking out over
//! their lifetimes. The burst parameters are the caller's; this domain
//! owns the mechanism only.

pub mod components;
pub mod entities;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use self::systems::update_particles::update_particles;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the particles domain: motion advances with the rest of
/// presentation, in the Animate phase.
pub fn register(app: &mut App) {
    app.add_systems(Update, update_particles.in_set(DisplayPhase::Animate));
}
