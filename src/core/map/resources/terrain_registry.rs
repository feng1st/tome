use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::map::constants::terrain_flags::TerrainFlags;
use crate::core::map::types::terrain::Terrain;
use crate::core::map::types::terrain_entry::TerrainEntry;
use crate::core::map::types::terrain_index::TerrainIndex;

pub const TERRAIN_TABLE_PATH: &str = "data/core/terrains.ron";

#[derive(Resource)]
pub struct TerrainRegistry {
    terrains: Vec<Terrain>,
    by_id: HashMap<String, TerrainIndex>,
}

impl TerrainRegistry {
    pub(crate) fn new(terrains: Vec<Terrain>) -> Self {
        let mut by_id = HashMap::with_capacity(terrains.len());
        for (index, terrain) in terrains.iter().enumerate() {
            let terrain_index = TerrainIndex::from_index(index);
            assert!(
                by_id.insert(terrain.id.clone(), terrain_index).is_none(),
                "duplicate terrain id '{}'",
                terrain.id
            );
        }
        TerrainRegistry { terrains, by_id }
    }

    pub fn get(&self, terrain_index: TerrainIndex) -> Option<&Terrain> {
        self.terrains.get(terrain_index.index())
    }

    pub fn get_index(&self, id: &str) -> Option<TerrainIndex> {
        self.by_id.get(id).copied()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (TerrainIndex, &Terrain)> {
        self.terrains
            .iter()
            .enumerate()
            .map(|(index, terrain)| (TerrainIndex::from_index(index), terrain))
    }
}

impl FromWorld for TerrainRegistry {
    fn from_world(_world: &mut World) -> Self {
        let terrain_table_content = fs::read_to_string(TERRAIN_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read terrain table '{TERRAIN_TABLE_PATH}': {e}"));
        parse_terrain_table(TERRAIN_TABLE_PATH, &terrain_table_content)
    }
}

pub(crate) fn parse_terrain_table(path: &str, terrain_table_content: &str) -> TerrainRegistry {
    let entries: Vec<TerrainEntry> = ron::from_str(terrain_table_content)
        .unwrap_or_else(|e| panic!("terrain table '{path}' is not valid RON: {e}"));
    let mut terrains = Vec::with_capacity(entries.len());
    for entry in entries {
        assert!(
            !entry.terrain.is_empty(),
            "terrain table '{path}': empty terrain id"
        );
        let mut flags = TerrainFlags::empty();
        for name in &entry.flags {
            let Some(flag) = TerrainFlags::from_name(name) else {
                panic!(
                    "terrain table '{path}': terrain '{}' has unknown flag '{name}'",
                    entry.terrain
                );
            };
            flags |= flag;
        }
        terrains.push(Terrain {
            id: entry.terrain,
            flags,
        });
    }
    TerrainRegistry::new(terrains)
}
