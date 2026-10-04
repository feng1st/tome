//! Serde layout of the class vocabulary file (`data/core/classes.ron`).

use serde::Deserialize;

/// One class entry in the vocabulary file: the class id, the statistic
/// modifiers merged into rolled base statistics at birth (one per
/// statistic, six total), and the class's share of the hit die. Parsed
/// as a list so a wrong length is a validated content error naming the
/// class, not a shape error in the file format.
#[derive(Deserialize)]
pub struct ClassEntry {
    pub class: String,
    pub stat_modifiers: Vec<i32>,
    pub hit_die: u16,
}
