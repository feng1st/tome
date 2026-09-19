use bevy::prelude::*;

/// Handles of the raw images, waiting to be repacked into array textures.
/// `anims` is aligned with `constants::terrain_anims::TERRAIN_ANIMS`.
#[derive(Resource)]
pub struct PendingChunks {
    pub tiles: Handle<Image>,
    pub anims: Vec<Handle<Image>>,
}
