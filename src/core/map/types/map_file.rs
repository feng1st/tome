//! Serde layout of a fixed map file (`data/maps/*.ron`).

use std::collections::HashMap;

use serde::Deserialize;

use crate::core::map::types::monster_spawn_entry::MonsterSpawnEntry;

/// A fixed map document: `legend` maps each character to a terrain id,
/// `rows` draws the map top-down, one character per cell. The map's id
/// is its file name. Map dimensions come from the rows themselves —
/// width is the row length, height the row count. `monsters` declares
/// what spawns where; absent means nothing spawns.
#[derive(Deserialize)]
pub struct MapFile {
    pub legend: HashMap<char, String>,
    pub rows: Vec<String>,
    #[serde(default)]
    pub monsters: Vec<MonsterSpawnEntry>,
}
