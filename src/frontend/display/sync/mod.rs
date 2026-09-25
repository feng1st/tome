//! Position sync: derives render transforms from the core's authoritative
//! cell-space positions.

pub mod systems;

use bevy::prelude::*;

/// Register the sync domain.
pub fn register(app: &mut App) {
    systems::register(app);
}
