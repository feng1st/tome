//! The core's figure vocabulary: the figure a creature presents, as an
//! entity-carried registry handle. Everything visual (textures, layouts,
//! clips) lives in the display side; the core only names the figure.

pub mod components;
pub mod resources;

use bevy::prelude::*;

use self::resources::figure_registry::FigureRegistry;

/// Register the figure domain: the vocabulary registry builds at app
/// build time (`FromWorld`); dependents pull it into existence, so
/// registration order never matters and data errors panic before the
/// window opens.
pub fn register(app: &mut App) {
    app.init_resource::<FigureRegistry>();
}
