//! Serde layout of the race vocabulary file (`data/core/races.ron`).

use serde::Deserialize;

/// One race entry in the vocabulary file: the race id, the statistic
/// modifiers merged into rolled base statistics at birth (one per
/// statistic, six total), and the race's share of the hit die. Parsed
/// as a list so a wrong length is a validated content error naming the
/// race, not a shape error in the file format.
#[derive(Deserialize)]
pub struct RaceEntry {
    pub race: String,
    pub stat_modifiers: Vec<i32>,
    pub hit_die: u16,
}
