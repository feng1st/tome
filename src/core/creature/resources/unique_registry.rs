//! The unique registry: every named individual's id resolved to its
//! runtime handle. One theme, one file: the resource, its construction
//! from the declaring content files, and the aggregated validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::creature::components::unique_index::UniqueIndex;
use crate::core::creature::types::unique_id_entry::UniqueIdEntry;

/// The content files declaring individuals, in aggregation order. An
/// individual is content and lives where its gameplay data lives — the
/// monster table today (`unique_id` on monster entries); `npc.ron` joins
/// when the NPC domain lands. One file may serve several parsers, each
/// reading its own part: this registry reads only the `unique_id`
/// column, the owning domain reads the rest — no registry depends on
/// another's parse.
const UNIQUE_SOURCE_PATHS: &[&str] = &["data/core/monsters.ron"];

/// Registry of all named individuals, built once at startup from the
/// declaring content files. `UniqueIndex` is the aggregated index in
/// source order — an unstable runtime handle, never an identity; ids
/// serve file references, error messages, and save serialization only.
/// No unique content means an empty registry: valid, and no creature
/// then carries a unique handle.
#[derive(Resource)]
pub struct UniqueRegistry {
    by_id: HashMap<String, UniqueIndex>,
}

impl UniqueRegistry {
    /// An empty registry: aggregation starts here.
    pub(crate) fn empty() -> Self {
        UniqueRegistry {
            by_id: HashMap::new(),
        }
    }

    /// Aggregate unique ids from one content vocabulary, in the source's
    /// entry order.
    pub(crate) fn extend(&mut self, ids: impl IntoIterator<Item = String>) {
        for id in ids {
            assert!(!id.is_empty(), "unique vocabulary: empty unique id");
            let index = UniqueIndex::from_index(self.by_id.len());
            assert!(
                self.by_id.insert(id.clone(), index).is_none(),
                "unique vocabulary: duplicate unique id '{id}'"
            );
        }
    }

    /// The handle for an individual's id, if the id is declared. Ids
    /// resolve only at content boundaries (spawn sites, display
    /// bindings).
    pub fn get_index(&self, id: &str) -> Option<UniqueIndex> {
        self.by_id.get(id).copied()
    }
}

impl FromWorld for UniqueRegistry {
    /// Build by opening each declaring file and reading only the
    /// `unique_id` column. Self-contained (no other registry), so
    /// registration order never matters.
    fn from_world(_world: &mut World) -> Self {
        let mut unique_registry = UniqueRegistry::empty();
        for path in UNIQUE_SOURCE_PATHS {
            let text = fs::read_to_string(path)
                .unwrap_or_else(|e| panic!("cannot read unique source '{path}': {e}"));
            let unique_id_entries: Vec<UniqueIdEntry> = ron::Options::default()
                .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
                .from_str(&text)
                .unwrap_or_else(|e| panic!("unique source '{path}' is not valid RON: {e}"));
            unique_registry.extend(
                unique_id_entries
                    .into_iter()
                    .filter_map(|entry| entry.unique_id),
            );
        }
        unique_registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::monster::resources::monster_registry::MONSTER_TABLE_PATH;
    use crate::core::monster::types::monster_entry::MonsterEntry;

    #[test]
    fn ids_resolve_to_handles_in_source_order() {
        let mut unique_registry = UniqueRegistry::empty();
        unique_registry.extend(vec!["grip".to_string(), "fang".to_string()]);
        let grip = unique_registry.get_index("grip").unwrap();
        let fang = unique_registry.get_index("fang").unwrap();
        // Same id, same handle; different ids, different handles. Concrete
        // values are the aggregated index: source order decides them,
        // and logic must never depend on them.
        assert_eq!(grip, unique_registry.get_index("grip").unwrap());
        assert_ne!(grip, fang);
        assert!(unique_registry.get_index("wolf").is_none());
    }

    #[test]
    fn no_unique_content_means_an_empty_registry() {
        let unique_registry = UniqueRegistry::empty();
        assert!(unique_registry.get_index("grip").is_none());
    }

    #[test]
    #[should_panic(expected = "duplicate unique id 'grip'")]
    fn duplicate_id_panics() {
        let mut unique_registry = UniqueRegistry::empty();
        unique_registry.extend(vec!["grip".to_string()]);
        unique_registry.extend(vec!["grip".to_string()]);
    }

    #[test]
    #[should_panic(expected = "empty unique id")]
    fn empty_id_panics() {
        let mut unique_registry = UniqueRegistry::empty();
        unique_registry.extend(vec![String::new()]);
    }

    /// Cross-parser consistency: the unique-id view of a shared file
    /// must agree with the owning domain's full parse — a renamed
    /// `unique_id` field would otherwise silently empty this registry.
    #[test]
    fn unique_id_view_agrees_with_the_full_parse() {
        let text = std::fs::read_to_string(MONSTER_TABLE_PATH).unwrap();
        let options = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
        let full: Vec<MonsterEntry> = options.from_str(&text).unwrap();
        let unique_id_entries: Vec<UniqueIdEntry> = options.from_str(&text).unwrap();
        let full_ids: Vec<&str> = full
            .iter()
            .filter_map(|entry| entry.unique_id.as_deref())
            .collect();
        let view_ids: Vec<&str> = unique_id_entries
            .iter()
            .filter_map(|entry| entry.unique_id.as_deref())
            .collect();
        assert_eq!(full_ids, view_ids);
    }

    /// Spec-alignment test: the real content files on disk declare no
    /// individual, so aggregation yields an empty registry.
    #[test]
    fn no_individual_is_declared_yet() {
        let text = std::fs::read_to_string(MONSTER_TABLE_PATH).unwrap();
        let unique_id_entries: Vec<UniqueIdEntry> = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(&text)
            .unwrap();
        assert!(unique_id_entries
            .iter()
            .all(|entry| entry.unique_id.is_none()));
    }
}
