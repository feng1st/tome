//! Attaches the camera target: the hero-specific display glue between the
//! core's `Hero` marker and the camera domain. Generic appearance
//! (sprite, playback) is the appearance domain's job.

use bevy::prelude::*;

use crate::core::hero::components::hero::Hero;
use crate::frontend::display::camera::components::camera_target::CameraTarget;

/// The camera follows the hero.
pub fn attach_target(mut commands: Commands, query: Query<Entity, Added<Hero>>) {
    for entity in &query {
        commands.entity(entity).insert(CameraTarget);
    }
}
