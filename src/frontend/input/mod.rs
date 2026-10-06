//! Input: raw device input translated into core commands. A mouse press
//! becomes the core command `TargetCell`; what the target means — a walk
//! or a strike — is the core's decision.

pub mod systems;

use bevy::prelude::*;

use crate::core::game_loop::GameLoop;

/// Register the input domain: the translation systems land in the
/// core-owned `GameLoop::Input` stage.
pub fn register(app: &mut App) {
    app.add_systems(Update, systems::mouse::translate.in_set(GameLoop::Input));
}
