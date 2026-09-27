//! A tileset: a texture loaded as a uniform grid of frames.

use bevy::prelude::*;

/// A loaded tileset: the texture handle (loaded as a grid image array —
/// the chunk shader samples `texture_2d_array`) and its frame grid.
/// Frame pixel size is the world tile size by definition — the loader
/// imposes it via `ImageArrayLayout::GridSize` — so it is not stored
/// here.
pub struct Tileset {
    pub texture: Handle<Image>,
    pub columns: u32,
    pub rows: u32,
}
