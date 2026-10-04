//! Attaches the camera target: the player-specific display glue between the
//! core's `Player` marker and the camera domain. Generic appearance
//! (sprite, playback) is the figure domain's job.

use bevy::prelude::*;

use crate::core::player::components::player::Player;
use crate::frontend::display::camera::components::camera_target::CameraTarget;

/// The camera follows the player.
pub fn attach_target(mut commands: Commands, query: Query<Entity, Added<Player>>) {
    for entity in &query {
        commands.entity(entity).insert(CameraTarget);
    }
}
