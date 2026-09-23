//! The game core: display- and input-agnostic game data and logic.
//! Plain modules by design — the core defines no Plugin struct; the
//! replaceable frontend is a plugin that depends on this core.

pub mod hero;
pub mod map;
pub mod movement;
pub mod system_sets;

use bevy::prelude::*;

use system_sets::GameSet;

/// Register all core domains plus the core-side frame logic, ordered inside
/// `GameSet`: commands are executed before movement advances.
pub fn register(app: &mut App) {
    map::register(app);
    hero::register(app);
    app.add_systems(
        Update,
        (
            hero::systems::commands::move_to_cell::execute,
            movement::systems::follow_path::follow_path,
        )
            .chain()
            .in_set(GameSet),
    );
}
