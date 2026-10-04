//! The monster kind value type: a monster kind's game data.

use crate::core::speed::components::speed::Speed;

/// A monster kind's game data as loaded from the monster table. Not an
/// entity: monsters in the world are entities carrying `MonsterIndex`;
/// this is the kind's row in the vocabulary — the slot its combat data
/// (hit dice, armor, blows) grows into. Ids live in the registry's
/// lookup map; logic queries data, never identity.
pub struct MonsterKind {
    pub speed: Speed,
}
