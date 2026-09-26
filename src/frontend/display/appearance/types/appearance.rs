//! The appearance value type: everything the renderer needs to draw a
//! creature that wears this look.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::frontend::display::animation::constants::anim_kind::AnimKind;
use crate::frontend::display::animation::types::anim_clip::AnimClip;

/// One appearance: sprite sheet, atlas layout, and the anim table. Pure
/// display data — the core knows only the `AppearanceKind` key. The
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
    /// (`Idle` is mandatory); undefined anims fall back to `Idle`. When
    /// appearances move to data files, `frames` becomes `Rc<[usize]>`.
    pub fn clip(&self, anim: AnimKind) -> &AnimClip {
        self.clips
            .get(&anim)
            .or_else(|| self.clips.get(&AnimKind::Idle))
            .expect("every appearance defines an Idle clip")
    }
}
