//! Serde layout of the monster vocabulary file
//! (`data/core/monsters.ron`).

use serde::Deserialize;

/// One monster entry in the vocabulary file: the monster id and its
/// speed (an index into `SPEED_RATE_TABLE`). The kind is the monster's
/// identity — there is no separate race, class, or individual id.
#[derive(Deserialize)]
pub struct MonsterEntry {
    pub monster: String,
    pub speed: usize,
}
