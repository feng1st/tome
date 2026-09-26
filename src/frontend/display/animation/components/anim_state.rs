// TODO: pending cleanup review — remove once stabilized
//! Locomotion animation states shared by all animated characters.

use bevy::prelude::*;

/// The locomotion states any animated character can be in. `animate` picks
/// the state from the entity's situation (currently: walking a path or not)
/// and looks up the matching clip in `AnimClips`.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnimState {
    Idle,
    Run,
}
