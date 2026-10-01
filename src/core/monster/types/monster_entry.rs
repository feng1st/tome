//! Serde layout of the monster vocabulary file
//! (`data/core/monsters.ron`).

use serde::Deserialize;

/// One monster entry in the vocabulary file: the monster id, the race it
/// is, and — for monsters that have them — a vocation (`class`) and an
/// individual identity (`unique_id`). Entry form (a record, not a bare
/// string) so kernel fields (speed, hit dice, …) extend the file format
/// in place when game logic first queries them. Optional fields parse as
/// plain strings (the registry parses with RON's `implicit_some`
/// extension, so no `Some(...)` wrappers in the file).
#[derive(Deserialize)]
pub struct MonsterEntry {
    pub monster: String,
    pub race: String,
    #[serde(default)]
    pub class: Option<String>,
    #[serde(default)]
    pub unique_id: Option<String>,
}
