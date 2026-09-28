//! Serde layout of the monster vocabulary file
//! (`data/core/monsters.ron`).

use serde::Deserialize;

/// One monster entry in the vocabulary file. Entry form (a record, not
/// a bare string) so kernel fields (speed, hit dice, …) extend the file
/// format in place when game logic first queries them.
#[derive(Deserialize)]
pub struct MonsterEntry {
    pub monster: String,
}
