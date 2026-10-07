//! The appearance value type: everything the renderer needs to draw a
//! creature presenting a figure.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim::Anim;

/// One appearance: sprite sheet, atlas layout, frame size, and the
/// anim table. Pure display data — entities carry only the `FigureIndex`
/// handle. The handles are cloned onto each instance at spawn (the
/// renderer reads them off the entity); the anim table is looked up per
/// frame through `anim()`.
pub struct Appearance {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    /// Frame size in texels; the texel-grid anchor derives from it (see
    /// `attach_appearance`).
    pub frame_size: UVec2,
    anims: HashMap<AnimKind, Anim>,
}

impl Appearance {
    pub fn new(
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        frame_size: UVec2,
        anims: HashMap<AnimKind, Anim>,
    ) -> Self {
        Appearance {
            texture,
            layout,
            frame_size,
            anims,
        }
    }

    /// The anim for `anim_kind`. Appearances define a subset of
    /// `AnimKind` (`Idle` is mandatory — checked at load); undefined
    /// anims fall back to `Idle`.
    pub fn anim(&self, anim_kind: AnimKind) -> &Anim {
        self.anims
            .get(&anim_kind)
            .or_else(|| self.anims.get(&AnimKind::Idle))
            .expect("every appearance defines an Idle anim")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idle() -> Anim {
        Anim {
            frames: vec![0, 1],
            fps: 8.0,
            looped: true,
        }
    }

    fn run() -> Anim {
        Anim {
            frames: vec![2, 3],
            fps: 20.0,
            looped: true,
        }
    }

    fn appearance(anims: &[(AnimKind, Anim)]) -> Appearance {
        Appearance::new(
            Handle::default(),
            Handle::default(),
            UVec2::new(12, 15),
            anims.iter().cloned().collect(),
        )
    }

    #[test]
    fn undefined_anims_fall_back_to_idle() {
        let appearance = appearance(&[(AnimKind::Idle, idle())]);
        assert_eq!(appearance.anim(AnimKind::Attack).frames, idle().frames);
    }

    #[test]
    fn defined_anims_return_their_own_frames() {
        let appearance = appearance(&[(AnimKind::Idle, idle()), (AnimKind::Run, run())]);
        assert_eq!(appearance.anim(AnimKind::Run).frames, run().frames);
        assert_eq!(appearance.anim(AnimKind::Idle).frames, idle().frames);
    }
}
