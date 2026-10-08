//! Hit points, the damage channel that spends them, and the death
//! handling that follows. The maximum's source differs per side —
//! players derive it from constitution at birth, monsters from their
//! kind's hit dice — but the component and its meaning are one.

pub mod components;
pub mod constants;
pub mod messages;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use self::messages::damage::Damage;
use self::messages::damaged::Damaged;
use self::systems::apply_damage::apply_damage;
use self::systems::despawn_dead::despawn_dead;
use crate::core::core_phase::CorePhase;

/// Register the health domain: the request and fact messages exist; the
/// apply system runs once per resolve phase — one instance after the
/// driver's action, one after the world's, the destructive drain making
/// the twin registrations safe — and the departure sweep runs ahead of
/// them each frame, taking the dead whose disappearance is done.
pub fn register(app: &mut App) {
    app.add_message::<Damage>()
        .add_message::<Damaged>()
        .add_systems(Update, despawn_dead.in_set(CorePhase::Derive))
        .add_systems(Update, apply_damage.in_set(CorePhase::PlayerResolve))
        .add_systems(Update, apply_damage.in_set(CorePhase::WorldResolve));
}
