pub mod components;
pub mod resources;
pub mod types;

use bevy::prelude::*;

use self::resources::class_registry::ClassRegistry;

pub fn register(app: &mut App) {
    app.init_resource::<ClassRegistry>();
}
