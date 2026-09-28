//! The monster registry: every monster id resolved to its runtime
//! handle. One theme, one file: the resource, its construction from the
//! vocabulary file, and the file's parsing and validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::monster::types::monster_entry::MonsterEntry;

/// The monster vocabulary loaded at startup.
pub const MONSTER_TABLE_PATH: &str = "data/core/monsters.ron";

/// Registry of all monsters, built once at startup from the vocabulary
/// file. `MonsterIndex` is the index into the internal table, assigned in
/// file order — an unstable runtime handle, never an identity: the same
/// monster's index changes when the file's entry order changes. Spawn
/// sites resolve ids to handles here; ids serve file references, error
/// messages, and save serialization only.
#[derive(Resource)]
pub struct MonsterRegistry {
    ids: Vec<String>,
    by_id: HashMap<String, MonsterIndex>,
}

impl MonsterRegistry {
    /// The handle for a monster id, if the id is declared. Ids resolve
    /// only at content boundaries (spawn sites, display bindings).
    pub fn get_index(&self, id: &str) -> Option<MonsterIndex> {
        self.by_id.get(id).copied()
    }

    /// All declared monster ids with their handles. Crate-internal: load
    /// validation that must cover every monster iterates here.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (MonsterIndex, &str)> {
        self.ids
            .iter()
            .enumerate()
            .map(|(index, id)| (MonsterIndex::from_index(index), id.as_str()))
    }
}

impl FromWorld for MonsterRegistry {
    /// Build from the vocabulary file. The registry has no resource
    /// dependencies; dependent registries pull it via
    /// `World::get_resource_or_init`.
    fn from_world(_world: &mut World) -> Self {
        let text = fs::read_to_string(MONSTER_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read monster table '{MONSTER_TABLE_PATH}': {e}"));
        parse_monster_registry(MONSTER_TABLE_PATH, &text)
    }
}

/// Parse and validate monster vocabulary text. Split from file IO
/// (`FromWorld`) so tests can exercise it with inline documents.
pub(crate) fn parse_monster_registry(path: &str, text: &str) -> MonsterRegistry {
    let entries: Vec<MonsterEntry> = ron::from_str(text)
        .unwrap_or_else(|e| panic!("monster table '{path}' is not valid RON: {e}"));
    let mut ids = Vec::with_capacity(entries.len());
    let mut by_id = HashMap::with_capacity(entries.len());
    for (index, entry) in entries.into_iter().enumerate() {
        assert!(
            !entry.monster.is_empty(),
            "monster table '{path}': empty monster id"
        );
        assert!(
            by_id
                .insert(entry.monster.clone(), MonsterIndex::from_index(index))
                .is_none(),
            "monster table '{path}': duplicate monster id '{}'",
            entry.monster
        );
        ids.push(entry.monster);
    }
    MonsterRegistry { ids, by_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"[ ( monster: "giant_white_rat" ), ( monster: "giant_grey_rat" ) ]"#;

    #[test]
    fn ids_resolve_to_handles() {
        let monster_registry = parse_monster_registry("test", DOC);
        let white = monster_registry.get_index("giant_white_rat").unwrap();
        let grey = monster_registry.get_index("giant_grey_rat").unwrap();
        // Same id, same handle; different ids, different handles. Concrete
        // values are the internal table index: file order decides them,
        // and logic must never depend on them.
        assert_eq!(
            white,
            monster_registry.get_index("giant_white_rat").unwrap()
        );
        assert_ne!(white, grey);
        assert!(monster_registry.get_index("wolf").is_none());
    }

    #[test]
    fn iter_covers_every_id_in_file_order() {
        let monster_registry = parse_monster_registry("test", DOC);
        let ids: Vec<&str> = monster_registry.iter().map(|(_, id)| id).collect();
        assert_eq!(ids, ["giant_white_rat", "giant_grey_rat"]);
    }

    #[test]
    #[should_panic(expected = "duplicate monster id 'giant_white_rat'")]
    fn duplicate_id_panics() {
        parse_monster_registry(
            "test",
            r#"[ ( monster: "giant_white_rat" ), ( monster: "giant_white_rat" ) ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "empty monster id")]
    fn empty_id_panics() {
        parse_monster_registry("test", r#"[ ( monster: "" ) ]"#);
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_monster_registry("test", "this is not ron");
    }

    /// Spec-alignment test: the real vocabulary file on disk declares the
    /// giant white rat.
    #[test]
    fn vocabulary_declares_the_giant_white_rat() {
        let text = std::fs::read_to_string(MONSTER_TABLE_PATH).unwrap();
        let monster_registry = parse_monster_registry(MONSTER_TABLE_PATH, &text);
        monster_registry
            .get_index("giant_white_rat")
            .expect("giant_white_rat is a declared monster");
    }
}
