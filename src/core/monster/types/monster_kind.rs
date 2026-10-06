//! The monster kind value type: a monster kind's game data.

use crate::core::dice::types::dice::Dice;
use crate::core::monster::types::monster_blow::MonsterBlow;
use crate::core::speed::components::speed::Speed;

/// A monster kind's game data as loaded from the monster table. Not an
/// entity: monsters in the world are entities carrying `MonsterIndex`;
/// this is the kind's row in the vocabulary — its combat profile (hit
/// dice, armor class, level, blows). Ids live in the registry's lookup
/// map; logic queries data, never identity.
pub struct MonsterKind {
    pub speed: Speed,
    pub hit_points: Dice,
    pub armor_class: i32,
    // Both consumed by monster attack resolution when it lands.
    #[allow(dead_code)]
    pub level: i32,
    #[allow(dead_code)]
    pub blows: Vec<MonsterBlow>,
}
