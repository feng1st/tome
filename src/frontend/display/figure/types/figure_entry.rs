//! Serde layout of the figure table file (`data/graphic/figures.ron`).

use bevy::prelude::*;
use serde::Deserialize;

use crate::frontend::display::figure::types::anim_entry::AnimEntry;

/// One figure entry: the figure id, the texture it draws from, the frame
/// grid that slices the texture, and the anim table. The frame size is
/// per entry — creature sheets are not world-tile sized, and the atlas
/// layout is built before the pixels load, so the grid cannot be derived
/// from the image.
#[derive(Deserialize)]
pub struct FigureEntry {
    pub figure: String,
    pub texture: String,
    pub frame_size: UVec2,
    pub columns: u32,
    pub rows: u32,
    pub anims: Vec<AnimEntry>,
}
