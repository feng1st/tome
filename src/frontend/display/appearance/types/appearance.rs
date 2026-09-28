//! The appearance value type: everything the renderer needs to draw a
//! creature presenting this figure.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim_clip::AnimClip;

/// One appearance: sprite sheet, atlas layout, and the anim table. Pure
/// display data — the core knows only the `FigureIndex` handle. The
/// handles are cloned onto each instance at spawn (the renderer reads
/// them off the entity); the anim table is looked up per frame through
/// `clip()`.
pub struct Appearance {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    clips: HashMap<AnimKind, AnimClip>,
}

impl Appearance {
    pub fn new(
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        clips: HashMap<AnimKind, AnimClip>,
    ) -> Self {
        Appearance {
            image,
            layout,
            clips,
        }
    }

    /// The clip for `anim`. Appearances define a subset of `AnimKind`
    /// (`Idle` is mandatory — checked at load); undefined anims fall
    /// back to `Idle`.
    pub fn clip(&self, anim: AnimKind) -> &AnimClip {
        self.clips
            .get(&anim)
            .or_else(|| self.clips.get(&AnimKind::Idle))
            .expect("every appearance defines an Idle clip")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idle() -> AnimClip {
        AnimClip {
            frames: vec![0, 1],
            fps: 8.0,
        }
    }

    fn run() -> AnimClip {
        AnimClip {
            frames: vec![2, 3],
            fps: 20.0,
        }
    }

    fn appearance(clips: &[(AnimKind, AnimClip)]) -> Appearance {
        Appearance::new(
            Handle::default(),
            Handle::default(),
            clips.iter().cloned().collect(),
        )
    }

    #[test]
    fn undefined_anims_fall_back_to_idle() {
        let appearance = appearance(&[(AnimKind::Idle, idle())]);
        assert_eq!(appearance.clip(AnimKind::Attack).frames, idle().frames);
    }

    #[test]
    fn defined_anims_return_their_own_clip() {
        let appearance = appearance(&[(AnimKind::Idle, idle()), (AnimKind::Run, run())]);
        assert_eq!(appearance.clip(AnimKind::Run).frames, run().frames);
        assert_eq!(appearance.clip(AnimKind::Idle).frames, idle().frames);
    }
}
