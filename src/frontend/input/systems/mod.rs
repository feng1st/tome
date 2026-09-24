//! Input systems: device translation plus the gesture resolvers.

pub mod devices;
pub mod gestures;

use bevy::prelude::*;

/// Register both system groups.
pub fn register(app: &mut App) {
    devices::register(app);
    gestures::register(app);
}
