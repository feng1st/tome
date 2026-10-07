//! Serde layout of an anim table entry (inside the figure table file).

use serde::Deserialize;

use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

/// One anim table entry: an anim name (from the code-owned vocabulary,
/// lowercase in the file) mapped to a frame sequence (indices into the
/// entry's row-major sheet grid), a playback rate in frames per
/// second, and the looping flag — looping tables wrap their playback,
/// non-looping tables clamp at the last frame.
#[derive(Deserialize)]
pub struct AnimEntry {
    pub anim: AnimKind,
    pub frames: Vec<usize>,
    pub fps: f32,
    pub looped: bool,
}
