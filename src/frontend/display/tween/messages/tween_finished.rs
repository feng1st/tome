//! The tween-finished fact.

use bevy::prelude::*;

/// A tween ran to its end on this entity and removed itself — the
/// completion callback as a message. Emitted by `update_tweens`; any
/// number of readers may react (release the body it was fading, chain
/// the next step, and so on).
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TweenFinished {
    pub entity: Entity,
}
