//! Serde layout of a font table entry (inside the font table file).

use serde::Deserialize;

/// One font table entry: the font's name, its texture, the row height
/// that bounds the glyph scan, the layout baseline, the tracking added
/// to every glyph advance, and the character table the glyph stretches
/// pair with.
#[derive(Deserialize)]
pub struct FontEntry {
    pub font: String,
    pub texture: String,
    pub row_height: u32,
    pub baseline: i32,
    pub tracking: f32,
    pub chars: String,
}
