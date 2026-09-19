pub mod components;
pub mod entities;
pub mod systems;

use bevy::prelude::*;

pub fn register(app: &mut App) {
    app.add_systems(Startup, entities::camera::spawn_camera);
}
