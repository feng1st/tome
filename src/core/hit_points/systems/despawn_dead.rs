use bevy::prelude::*;

use crate::core::display_protocol::components::is_disappearing::IsDisappearing;
use crate::core::hit_points::components::hit_points::HitPoints;
use crate::core::player::components::player::Player;

#[allow(clippy::type_complexity)]
pub fn despawn_dead(
    mut commands: Commands,
    entities: Query<(Entity, &HitPoints), (Without<IsDisappearing>, Without<Player>)>,
) {
    for (entity, hit_points) in &entities {
        if hit_points.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}
