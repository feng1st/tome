pub mod components;
pub mod messages;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use self::messages::damaged::Damaged;
use crate::core::core_phase::CorePhase;

pub fn register(app: &mut App) {
    app.add_message::<Damaged>()
        .add_systems(
            Update,
            systems::despawn_dead::despawn_dead.in_set(CorePhase::WorldAdvance),
        )
        .add_systems(
            Update,
            systems::derive_hit_points::derive_hit_points.in_set(CorePhase::Derive),
        );
}
