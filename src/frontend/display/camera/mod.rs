//! The gameplay camera: spawn and target following. Camera is presentation;
//! a text display would have none.

pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

/// Register camera spawning. `follow_target` joins the cross-domain chain
/// assembled in main.rs.
pub fn register(app: &mut App) {
    app.add_systems(Startup, entities::camera::spawn_camera);
}
