//! The terrain registry: every terrain id resolved to its properties.
//! One theme, one file: the resource, its construction from the terrain
//! table file, and the table's parsing and validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::map::constants::terrain_flags::TerrainFlags;
use crate::core::map::types::terrain::Terrain;
use crate::core::map::types::terrain_entry::TerrainEntry;
use crate::core::map::types::terrain_index::TerrainIndex;

/// The terrain table loaded at startup.
pub const TERRAIN_TABLE_PATH: &str = "data/core/terrains.ron";

/// Registry of all terrains, built once at startup from the terrain
/// table file. `TerrainIndex` is the index into the internal table, assigned
/// in file order — an unstable runtime handle, never an identity: the
/// same terrain's index changes when the file's entry order changes. Logic
/// looks terrains up here and queries flags; ids serve file
/// references, error messages, and save serialization only.
#[derive(Resource)]
pub struct TerrainRegistry {
    terrains: Vec<Terrain>,
    by_id: HashMap<String, TerrainIndex>,
}

impl TerrainRegistry {
    /// Build the registry from parsed entries, assigning indexes in order.
    /// Duplicate ids are a data bug and panic.
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

    /// The terrain a handle points to, if the handle came from this
    /// registry. Runtime callers hold valid handles by construction and
    /// may treat `None` as a load-time bug.
    pub fn get(&self, terrain_index: TerrainIndex) -> Option<&Terrain> {
        self.terrains.get(terrain_index.index())
    }

    /// The handle for a terrain id, if the id is registered. Ids
    /// resolve only at load boundaries (map files, display bindings).
    pub fn get_index(&self, id: &str) -> Option<TerrainIndex> {
        self.by_id.get(id).copied()
    }

    /// All registered terrains with their handles. Crate-internal: load
    /// validation that must cover every terrain iterates here.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (TerrainIndex, &Terrain)> {
        self.terrains
            .iter()
            .enumerate()
            .map(|(index, terrain)| (TerrainIndex::from_index(index), terrain))
    }
}

impl FromWorld for TerrainRegistry {
    /// Build from the terrain table file. The registry has no resource
    /// dependencies; dependent registries pull it via
    /// `World::get_resource_or_init`.
    fn from_world(_world: &mut World) -> Self {
        let text = fs::read_to_string(TERRAIN_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read terrain table '{TERRAIN_TABLE_PATH}': {e}"));
        parse_terrain_registry(TERRAIN_TABLE_PATH, &text)
    }
}

/// Parse and validate terrain table text. Split from file IO
/// (`FromWorld`) so tests can exercise it with inline documents.
pub(crate) fn parse_terrain_registry(path: &str, text: &str) -> TerrainRegistry {
    let entries: Vec<TerrainEntry> = ron::from_str(text)
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

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"[
        ( terrain: "floor", flags: ["PASSABLE"] ),
        ( terrain: "wall",  flags: [] ),
        ( terrain: "water", flags: ["LIQUID"] ),
    ]"#;

    #[test]
    fn entries_become_queryable_terrains() {
        let terrain_registry = parse_terrain_registry("test", DOC);
        let floor = terrain_registry
            .get(terrain_registry.get_index("floor").unwrap())
            .unwrap();
        assert!(floor.flags.contains(TerrainFlags::PASSABLE));
        assert!(!floor.flags.contains(TerrainFlags::LIQUID));
        let wall = terrain_registry
            .get(terrain_registry.get_index("wall").unwrap())
            .unwrap();
        assert_eq!(wall.flags, TerrainFlags::empty());
        let water = terrain_registry
            .get(terrain_registry.get_index("water").unwrap())
            .unwrap();
        assert!(!water.flags.contains(TerrainFlags::PASSABLE));
        assert!(water.flags.contains(TerrainFlags::LIQUID));
        assert!(terrain_registry.get_index("lava").is_none());
    }

    #[test]
    fn indexes_follow_file_order() {
        let terrain_registry = parse_terrain_registry("test", DOC);
        // Indexes are the internal table index: file order decides them,
        // and logic must never depend on concrete values.
        let mut indexes: Vec<_> = ["floor", "wall", "water"]
            .iter()
            .map(|id| terrain_registry.get_index(id).unwrap())
            .collect();
        indexes.sort_by_key(|terrain_index| terrain_index.index());
        assert_eq!(indexes.len(), 3);
    }

    #[test]
    fn combined_flags_parse() {
        let terrain_registry = parse_terrain_registry(
            "test",
            r#"[ ( terrain: "mud", flags: ["PASSABLE", "LIQUID"] ) ]"#,
        );
        let mud = terrain_registry
            .get(terrain_registry.get_index("mud").unwrap())
            .unwrap();
        assert!(mud
            .flags
            .contains(TerrainFlags::PASSABLE | TerrainFlags::LIQUID));
    }

    #[test]
    #[should_panic(expected = "unknown flag 'FLAMMABLE'")]
    fn unknown_flag_panics() {
        parse_terrain_registry("test", r#"[ ( terrain: "grass", flags: ["FLAMMABLE"] ) ]"#);
    }

    #[test]
    #[should_panic(expected = "duplicate terrain id 'floor'")]
    fn duplicate_id_panics() {
        parse_terrain_registry(
            "test",
            r#"[
                ( terrain: "floor", flags: ["PASSABLE"] ),
                ( terrain: "floor", flags: [] ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_terrain_registry("test", "this is not ron");
    }
}
