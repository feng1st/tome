//! The monster kind value type: a monster kind's game data.

use crate::core::creature::components::class_index::ClassIndex;
use crate::core::creature::components::race_index::RaceIndex;
use crate::core::speed::components::speed::Speed;

/// A monster kind's game data as loaded from the monster table: the race
/// it is, its speed, and — for kinds that have them — a vocation and an
/// individual identity. Not an entity: monsters in the world are entities
/// carrying `MonsterIndex`; this is the kind's row in the vocabulary. Ids
/// live in the registry's lookup map; logic queries data, never identity.
/// Identity fields first, value fields last.
pub struct MonsterKind {
    pub race: RaceIndex,
    pub class: Option<ClassIndex>,
    pub unique_id: Option<String>,
    pub speed: Speed,
}
