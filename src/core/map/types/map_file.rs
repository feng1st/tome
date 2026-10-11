use std::collections::HashMap;

use serde::Deserialize;

use crate::core::map::types::monster_spawn_entry::MonsterSpawnEntry;

#[derive(Debug, Deserialize)]
pub struct MapFile {
    pub legend: HashMap<char, String>,
    pub rows: Vec<String>,
    #[serde(default)]
    pub monsters: Vec<MonsterSpawnEntry>,
}
