//! The bitmap-text domain: the font table, the font registry, and
//! text spawning as glyph sprites. Rendering a text is a mechanism —
//! when and what to render belongs to the caller.

pub mod constants;
pub mod entities;
pub mod resources;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use self::resources::font_registry::FontRegistry;

/// Register the bitmap-text domain: the registry builds at app build
/// time (`FromWorld`), decoding and splitting every font's texture.
pub fn register(app: &mut App) {
    app.init_resource::<FontRegistry>();
}
