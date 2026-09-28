//! Serde layout of the appearance table file
//! (`data/graphic/appearances.ron`).

use serde::Deserialize;

use crate::frontend::display::appearance::types::anim_clip_entry::AnimClipEntry;

/// One appearance entry: the figure it binds, the texture it draws from,
/// the frame grid that slices the texture, and the anim table. The frame
/// size is per entry — creature sheets are not world-tile sized, and the
/// atlas layout is built before the pixels load, so the grid cannot be
/// derived from the image.
#[derive(Deserialize)]
pub struct AppearanceEntry {
    pub figure: String,
    pub texture: String,
    pub frame_width: u32,
    pub frame_height: u32,
    pub columns: u32,
    pub rows: u32,
    pub clips: Vec<AnimClipEntry>,
}
