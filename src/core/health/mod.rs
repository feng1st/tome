//! Hit points, the damage channel that spends them, and the death
//! handling that follows. The ceiling's source differs per side —
//! players derive it from constitution at birth, monsters from their
//! kind's hit dice — but the component and its meaning are one.

pub mod components;
pub mod constants;
pub mod messages;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use self::messages::damage::Damage;
use self::systems::settle_damage::settle_damage;
use crate::core::core_phase::CorePhase;

/// Register the health domain: the damage message exists, and the
/// settle system runs once per resolve phase — one instance after the
/// driver's action, one after the world's. The destructive drain makes
/// the twin registrations safe: exactly the instance that follows the
/// writer applies each request.
pub fn register(app: &mut App) {
    app.add_message::<Damage>()
        .add_systems(Update, settle_damage.in_set(CorePhase::PlayerResolve))
        .add_systems(Update, settle_damage.in_set(CorePhase::WorldResolve));
}
