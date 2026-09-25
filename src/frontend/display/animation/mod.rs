//! Frame animation for sprite entities: clips, states, timers.

pub mod components;
pub mod systems;

use bevy::prelude::*;

/// Register the animation domain.
pub fn register(app: &mut App) {
    systems::register(app);
}
