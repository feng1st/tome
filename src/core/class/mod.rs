//! The class vocabulary: the vocations a creature can learn. Each entry
//! carries the birth data a classed creature merges into its rolled
//! statistics — statistic modifiers and a share of the hit die.

pub mod components;
pub mod resources;
pub mod types;

use bevy::prelude::*;

use self::resources::class_registry::ClassRegistry;

/// Register the class domain: the vocabulary builds at app build time
/// (`FromWorld`); dependents pull it into existence, so registration
/// order never matters and data errors panic before the window opens.
pub fn register(app: &mut App) {
    app.init_resource::<ClassRegistry>();
}
