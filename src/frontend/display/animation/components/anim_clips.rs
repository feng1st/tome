//! A creature template's anim table: which clip each `AnimKind` plays.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::frontend::display::animation::constants::anim_kind::AnimKind;
use crate::frontend::display::animation::types::anim_clip::AnimClip;

/// The template's anim map. Templates define a subset of `AnimKind`
/// (`Idle` is mandatory — it is the fallback); the map is cloned onto each
/// instance at spawn, resolve-once style. When templates move to data
/// files, `frames` becomes `Rc<[usize]>` and this cloning stays cheap.
#[derive(Component, Clone)]
pub struct AnimClips(pub HashMap<AnimKind, AnimClip>);

impl AnimClips {
    /// The clip for `anim`, falling back to `Idle` when the template does
    /// not define it.
    pub fn clip(&self, anim: AnimKind) -> &AnimClip {
        self.0
            .get(&anim)
            .or_else(|| self.0.get(&AnimKind::Idle))
            .expect("every template defines an Idle clip")
    }
}
