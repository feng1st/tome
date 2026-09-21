//! The game core: display- and input-agnostic game data and logic.
//! Plain modules by design — the core defines no Plugin struct; replaceable
//! implementations (graphic, input) are plugins that depend on this core.

pub mod hero;
pub mod map;
pub mod movement;
pub mod sets;

use bevy::prelude::*;

use sets::CoreSet;

/// Register all core domains plus the core-side frame logic, ordered inside
/// `CoreSet`: gestures are resolved into paths before movement advances.
pub fn register(app: &mut App) {
    map::register(app);
    hero::register(app);
    app.add_systems(
        Update,
        (
            hero::systems::resolve_primary_action::resolve_primary_action,
            movement::systems::follow_path::follow_path,
        )
            .chain()
            .in_set(CoreSet),
    );
}
