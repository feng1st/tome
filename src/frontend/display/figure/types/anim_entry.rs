use serde::Deserialize;

use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;

#[derive(Deserialize)]
pub struct AnimEntry {
    pub anim: AnimKind,
    pub frames: Vec<usize>,
    pub fps: f32,
    pub looped: bool,
}
