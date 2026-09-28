//! Serde layout of an anim table entry (inside the appearance table
//! file).

use serde::Deserialize;

use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

/// One anim table entry: an anim name (from the code-owned vocabulary,
/// lowercase in the file) mapped to a frame sequence (indices into the
/// entry's row-major sheet grid) and a playback rate in frames per
/// second.
#[derive(Deserialize)]
pub struct AnimClipEntry {
    pub anim: AnimKind,
    pub frames: Vec<usize>,
    pub fps: f32,
}
