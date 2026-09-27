//! Serde layout of a fixed map file (`data/maps/*.ron`).

use std::collections::HashMap;

use serde::Deserialize;

/// A fixed map document: `legend` maps each character to a terrain id,
/// `rows` draws the map top-down, one character per cell. The map's id
/// is its file name. Map dimensions come from the rows themselves —
/// width is the row length, height the row count.
#[derive(Deserialize)]
pub struct MapFile {
    pub legend: HashMap<char, String>,
    pub rows: Vec<String>,
}
