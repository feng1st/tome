//! The hero domain's game-data side: marker, spawning, and the gesture
//! message type. Appearance lives in `graphic::hero`.

pub mod components;
pub mod entities;
pub mod events;
pub mod systems;

use bevy::prelude::*;

use events::primary_action::PrimaryAction;

/// Register hero-domain game data: the gesture message type and the spawn
/// system. `resolve_primary_action` joins the core-side chain registered
/// in `core::register`.
pub fn register(app: &mut App) {
    app.add_message::<PrimaryAction>()
        .add_systems(Startup, entities::hero::spawn_hero);
}
