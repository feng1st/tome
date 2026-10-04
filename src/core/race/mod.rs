//! The race vocabulary: the species a creature can be played as. Each
//! entry carries the birth data a played creature merges into its
//! rolled statistics — statistic modifiers and a share of the hit die.
//! What a race looks like is the display side's business.

pub mod components;
pub mod resources;
pub mod types;

use bevy::prelude::*;

use self::resources::race_registry::RaceRegistry;

/// Register the race domain: the vocabulary builds at app build time
/// (`FromWorld`); dependents pull it into existence, so registration
/// order never matters and data errors panic before the window opens.
pub fn register(app: &mut App) {
    app.init_resource::<RaceRegistry>();
}
