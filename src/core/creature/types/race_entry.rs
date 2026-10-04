//! Serde layout of the race vocabulary file (`data/core/races.ron`).

use serde::Deserialize;

/// One race entry in the vocabulary file. Entry form (a record, not a
/// bare string) so kernel fields (stat modifiers, flags, …) extend the
/// file format in place when game logic first queries them.
#[derive(Deserialize)]
pub struct RaceEntry {
    pub race: String,
}
