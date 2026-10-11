use bevy::prelude::*;
use serde::Deserialize;

use crate::frontend::display::figure::types::anim_entry::AnimEntry;

#[derive(Deserialize)]
pub struct FigureEntry {
    pub figure: String,
    pub texture: String,
    pub frame_size: UVec2,
    pub columns: usize,
    pub rows: usize,
    pub anims: Vec<AnimEntry>,
}
