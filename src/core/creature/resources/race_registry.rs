//! The race registry: every race id resolved to its runtime handle.
//! One theme, one file: the resource, its construction from the
//! vocabulary file, and the file's parsing and validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::creature::components::race_index::RaceIndex;
use crate::core::creature::types::race_entry::RaceEntry;

/// The race vocabulary loaded at startup.
pub const RACE_TABLE_PATH: &str = "data/core/races.ron";

/// Registry of all races, built once at startup from the vocabulary
/// file. `RaceIndex` is the index into the internal table, assigned in
/// file order — an unstable runtime handle, never an identity: the same
/// race's index changes when the file's entry order changes. Spawn
/// sites resolve ids to handles here; ids serve file references, error
/// messages, and save serialization only.
#[derive(Resource)]
pub struct RaceRegistry {
    ids: Vec<String>,
    by_id: HashMap<String, RaceIndex>,
}

impl RaceRegistry {
    /// The handle for a race id, if the id is declared. Ids resolve
    /// only at content boundaries (spawn sites, display bindings).
    pub fn get_index(&self, id: &str) -> Option<RaceIndex> {
        self.by_id.get(id).copied()
    }

    /// All declared race ids with their handles. Crate-internal: load
    /// validation that must cover every race iterates here.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (RaceIndex, &str)> {
        self.ids
            .iter()
            .enumerate()
            .map(|(index, id)| (RaceIndex::from_index(index), id.as_str()))
    }
}

impl FromWorld for RaceRegistry {
    /// Build from the vocabulary file. The registry has no resource
    /// dependencies; dependent registries pull it via
    /// `World::get_resource_or_init`.
    fn from_world(_world: &mut World) -> Self {
        let text = fs::read_to_string(RACE_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read race table '{RACE_TABLE_PATH}': {e}"));
        parse_race_registry(RACE_TABLE_PATH, &text)
    }
}

/// Parse and validate race vocabulary text. Split from file IO
/// (`FromWorld`) so tests can exercise it with inline documents.
pub(crate) fn parse_race_registry(path: &str, text: &str) -> RaceRegistry {
    let entries: Vec<RaceEntry> =
        ron::from_str(text).unwrap_or_else(|e| panic!("race table '{path}' is not valid RON: {e}"));
    let mut ids = Vec::with_capacity(entries.len());
    let mut by_id = HashMap::with_capacity(entries.len());
    for (index, entry) in entries.into_iter().enumerate() {
        assert!(!entry.race.is_empty(), "race table '{path}': empty race id");
        assert!(
            by_id
                .insert(entry.race.clone(), RaceIndex::from_index(index))
                .is_none(),
            "race table '{path}': duplicate race id '{}'",
            entry.race
        );
        ids.push(entry.race);
    }
    RaceRegistry { ids, by_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"[ ( race: "human" ), ( race: "giant_white_rat" ) ]"#;

    #[test]
    fn ids_resolve_to_handles() {
        let race_registry = parse_race_registry("test", DOC);
        let human = race_registry.get_index("human").unwrap();
        let rat = race_registry.get_index("giant_white_rat").unwrap();
        // Same id, same handle; different ids, different handles. Concrete
        // values are the internal table index: file order decides them,
        // and logic must never depend on them.
        assert_eq!(human, race_registry.get_index("human").unwrap());
        assert_ne!(human, rat);
        assert!(race_registry.get_index("elf").is_none());
    }

    #[test]
    fn iter_covers_every_id_in_file_order() {
        let race_registry = parse_race_registry("test", DOC);
        let ids: Vec<&str> = race_registry.iter().map(|(_, id)| id).collect();
        assert_eq!(ids, ["human", "giant_white_rat"]);
    }

    #[test]
    #[should_panic(expected = "duplicate race id 'human'")]
    fn duplicate_id_panics() {
        parse_race_registry("test", r#"[ ( race: "human" ), ( race: "human" ) ]"#);
    }

    #[test]
    #[should_panic(expected = "empty race id")]
    fn empty_id_panics() {
        parse_race_registry("test", r#"[ ( race: "" ) ]"#);
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_race_registry("test", "this is not ron");
    }

    /// Spec-alignment test: the real vocabulary file on disk declares
    /// human and the giant white rat.
    #[test]
    fn vocabulary_declares_human_and_the_giant_white_rat() {
        let text = std::fs::read_to_string(RACE_TABLE_PATH).unwrap();
        let race_registry = parse_race_registry(RACE_TABLE_PATH, &text);
        race_registry
            .get_index("human")
            .expect("human is a declared race");
        race_registry
            .get_index("giant_white_rat")
            .expect("giant_white_rat is a declared race");
    }
}
