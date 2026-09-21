//! The game core: display- and input-agnostic game data and logic.
//! Plain modules by design — the core defines no Plugin struct; replaceable
//! implementations (graphic, input) are plugins that depend on this core.

pub mod hero;
pub mod map;
pub mod movement;

use bevy::prelude::*;

/// Register all core domains. Cross-domain system ordering is assembled in
/// main.rs, not here.
pub fn register(app: &mut App) {
    map::register(app);
    hero::register(app);
}
