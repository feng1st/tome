//! Preloaded hero sprite assets.

use bevy::prelude::*;

/// Handles for the hero's sprite sheet and atlas layout, loaded once at
/// startup and cloned per hero entity. `Assets::add` does not deduplicate,
/// so building the layout per entity would pile up identical assets.
#[derive(Resource, Clone)]
pub struct HeroSprites {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
