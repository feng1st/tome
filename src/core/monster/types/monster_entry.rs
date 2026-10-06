//! Serde layout of the monster vocabulary file
//! (`data/core/monsters.ron`).
//!
//! Co-location exception, not a grouping precedent: the default is one
//! value type per file. `MonsterBlowEntry` sits beside `MonsterEntry`
//! only because both conditions hold at once — it is a node of this
//! file's wire format, and nothing outside the format consumes it.
//! Missing either condition means the type moves to its own file, as
//! the parsed values the game reads did (`MonsterKind`, `MonsterBlow`).

use serde::Deserialize;

/// One blow in the vocabulary: the damage dice only. Method and effect
/// columns join when the effect family lands; the entry shape is the
/// slot they grow into.
#[derive(Deserialize)]
pub struct MonsterBlowEntry {
    pub damage: String,
}

/// One monster entry in the vocabulary file: the monster id, its speed
/// (an index into `SPEED_RATE_TABLE`), and its combat profile — hit
/// dice, armor class, level, and blows. The kind is the monster's
/// identity — there is no separate race, class, or individual id.
/// Required fields are plain types: absence is a format error at the
/// serde layer (file, field, position); load validation then judges
/// values, naming the entry.
#[derive(Deserialize)]
pub struct MonsterEntry {
    pub monster: String,
    pub speed: usize,
    pub hit_points: String,
    pub armor_class: i32,
    pub level: i32,
    pub blows: Vec<MonsterBlowEntry>,
}
