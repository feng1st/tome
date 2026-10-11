use std::collections::HashMap;

use bevy::prelude::*;

use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim::Anim;

pub struct Figure {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub frame_size: UVec2,
    anims: HashMap<AnimKind, Anim>,
}

impl Figure {
    pub fn new(
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        frame_size: UVec2,
        anims: HashMap<AnimKind, Anim>,
    ) -> Self {
        Figure {
            texture,
            layout,
            frame_size,
            anims,
        }
    }

    pub fn anim(&self, anim_kind: AnimKind) -> &Anim {
        self.anims
            .get(&anim_kind)
            .or_else(|| self.anims.get(&AnimKind::Idle))
            .expect("every figure defines an Idle anim")
    }
}
