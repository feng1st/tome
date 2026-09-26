// TODO: pending cleanup review — remove once stabilized
//! Loop-phase offset for stateless frame animation.

use bevy::prelude::*;

/// Starting frame of the entity's looping clips. Looping animation is a
/// pure function of global time — `(elapsed * fps + start) % len` — so this
/// is the only per-entity animation state: `0` syncs the entity with every
/// other zero-start entity (wind-like unison), a random value desyncs
/// crowds. Written once at spawn; never ticked.
#[derive(Component, Clone, Copy, Debug)]
pub struct AnimStartFrame(pub usize);
