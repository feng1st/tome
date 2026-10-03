//! The creature identity domain: what a creature is, as entity-carried
//! registry handles. Race is required (every creature is some race);
//! class and unique id are optional — class for creatures with a
//! vocation, unique id for named individuals. What an identity looks
//! like is the display side's business; the core only names identities.

pub mod components;
pub mod constants;
pub mod resources;
pub mod types;

use bevy::prelude::*;

use self::resources::class_registry::ClassRegistry;
use self::resources::race_registry::RaceRegistry;
use self::resources::unique_registry::UniqueRegistry;

/// Register the creature domain: the identity vocabularies build at app
/// build time (`FromWorld`); dependents pull them into existence, so
/// registration order never matters and data errors panic before the
/// window opens.
pub fn register(app: &mut App) {
    app.init_resource::<RaceRegistry>()
        .init_resource::<ClassRegistry>()
        .init_resource::<UniqueRegistry>();
}
