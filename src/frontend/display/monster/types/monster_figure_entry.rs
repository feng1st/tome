//! Serde layout of the monster figure binding file
//! (`data/graphic/monster_figures.ron`).

use serde::Deserialize;

/// One binding entry: which figure a monster kind presents. Both ids
/// resolve against their vocabularies at load time.
#[derive(Deserialize)]
pub struct MonsterFigureEntry {
    pub monster: String,
    pub figure: String,
}
