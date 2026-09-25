//! The gameplay camera: spawn and target following. Camera is presentation;
//! a text display would have none.

pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

/// Register the camera domain.
pub fn register(app: &mut App) {
    entities::register(app);
    systems::register(app);
}
