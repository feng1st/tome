//! Serde layout of the class vocabulary file (`data/core/classes.ron`).

use serde::Deserialize;

/// One class entry in the vocabulary file. Entry form (a record, not a
/// bare string) so kernel fields (stat modifiers, skills, …) extend the
/// file format in place when game logic first queries them.
#[derive(Deserialize)]
pub struct ClassEntry {
    pub class: String,
}
