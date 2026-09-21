//! The hero domain's game-data side: marker, spawning, and resolving move
//! intents into paths. Appearance lives in `graphic::hero`.

pub mod components;
pub mod entities;
pub mod events;
pub mod systems;

use bevy::prelude::*;

use events::move_to::MoveTo;

/// Register hero-domain game data: the intent event type and the spawn
/// system. `resolve_goal` joins the cross-domain chain assembled in main.rs.
pub fn register(app: &mut App) {
    app.add_message::<MoveTo>()
        .add_systems(Startup, entities::hero::spawn_hero);
}
