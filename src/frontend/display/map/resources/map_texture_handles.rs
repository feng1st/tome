//! Handles of the raw map textures, waiting to be repacked into array
//! textures.

use bevy::prelude::*;

/// `anims` is aligned with `constants::terrain_anims::TERRAIN_ANIMS`.
#[derive(Resource)]
pub struct MapTextureHandles {
    pub tiles: Handle<Image>,
    pub anims: Vec<Handle<Image>>,
}
