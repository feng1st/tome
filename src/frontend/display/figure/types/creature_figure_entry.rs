//! Serde layout of the creature figure binding file
//! (`data/graphic/creature_figures.ron`).

use serde::Deserialize;

/// One binding entry: which figure a creature presents. The present
/// fields decide the key shape — `monster` alone keys by kind, `race` +
/// `class` keys by vocation within a race, `race` alone keys the race
/// default; every other combination is a format error, rejected by the
/// binding registry at load. The optional-key form lets future identity
/// dimensions (ego, subrace, …) extend entries in place without
/// restructuring the file. Optional fields parse as plain strings (the
/// registry parses with RON's `implicit_some` extension, so no
/// `Some(...)` wrappers in the file).
#[derive(Deserialize)]
pub struct CreatureFigureEntry {
    #[serde(default)]
    pub monster: Option<String>,
    #[serde(default)]
    pub race: Option<String>,
    #[serde(default)]
    pub class: Option<String>,
    pub figure: String,
}
