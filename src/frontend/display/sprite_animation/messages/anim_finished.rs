//! The anim-finished fact.

use bevy::prelude::*;

use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

/// A one-shot animation ran to its end on this entity — the completion
/// callback as a message. Emitted by `sync_animation` once per
/// one-shot, the frame its end passes; readers (the death fade's start,
/// for one) never touch animation state directly.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimFinished {
    pub entity: Entity,
    pub anim: AnimKind,
}
