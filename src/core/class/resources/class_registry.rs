//! The class registry: every class resolved to its runtime handle and
//! its birth data. One theme, one file: the resource, its construction
//! from the vocabulary file, and the file's parsing and validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::class::components::class_index::ClassIndex;
use crate::core::class::types::class_entry::ClassEntry;
use crate::core::class::types::class_kind::ClassKind;

/// The class vocabulary loaded at startup.
pub const CLASS_TABLE_PATH: &str = "data/core/classes.ron";

/// How many statistic modifiers one class entry carries: one per
/// statistic.
const STAT_MODIFIER_COUNT: usize = 6;

/// Registry of all classes, built once at startup from the vocabulary
/// file. `ClassIndex` is the index into the internal table, assigned in
/// file order — an unstable runtime handle, never an identity: the same
/// class's index changes when the file's entry order changes. Spawn
/// sites resolve ids to handles here; ids serve file references, error
/// messages, and save serialization only. Entries sit in file order in
/// a dense table: row storage keeps one lookup per class and preserves
/// entry order for free.
#[derive(Resource)]
pub struct ClassRegistry {
    class_kinds: Vec<ClassKind>,
    by_id: HashMap<String, ClassIndex>,
}

impl ClassRegistry {
    /// The handle for a class id, if the id is declared. Ids resolve
    /// only at content boundaries (spawn sites, display bindings).
    pub fn get_index(&self, id: &str) -> Option<ClassIndex> {
        self.by_id.get(id).copied()
    }

    /// The class kind a handle points to: the class's birth data.
    /// Callers hold valid handles by construction (checked at load), so
    /// a missing entry at runtime is a load-time bug, not a runtime
    /// case.
    pub fn class_kind(&self, class_index: ClassIndex) -> &ClassKind {
        &self.class_kinds[class_index.index()]
    }
}

impl FromWorld for ClassRegistry {
    /// Build from the vocabulary file. The registry has no resource
    /// dependencies; dependent registries pull it via
    /// `World::get_resource_or_init`.
    fn from_world(_world: &mut World) -> Self {
        let text = fs::read_to_string(CLASS_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read class table '{CLASS_TABLE_PATH}': {e}"));
        parse_class_registry(CLASS_TABLE_PATH, &text)
    }
}

/// Parse and validate class vocabulary text. Split from file IO
/// (`FromWorld`) so tests can exercise it with inline documents.
pub(crate) fn parse_class_registry(path: &str, text: &str) -> ClassRegistry {
    let entries: Vec<ClassEntry> = ron::from_str(text)
        .unwrap_or_else(|e| panic!("class table '{path}' is not valid RON: {e}"));
    let mut class_kinds = Vec::with_capacity(entries.len());
    let mut by_id = HashMap::with_capacity(entries.len());
    for (index, entry) in entries.into_iter().enumerate() {
        assert!(
            !entry.class.is_empty(),
            "class table '{path}': empty class id"
        );
        assert!(
            by_id
                .insert(entry.class.clone(), ClassIndex::from_index(index))
                .is_none(),
            "class table '{path}': duplicate class id '{}'",
            entry.class
        );
        let modifier_count = entry.stat_modifiers.len();
        let stat_modifiers: [i32; STAT_MODIFIER_COUNT] = entry
            .stat_modifiers
            .try_into()
            .unwrap_or_else(|_| {
                panic!(
                    "class table '{path}': class '{}' has {modifier_count} stat modifiers (expected {STAT_MODIFIER_COUNT})",
                    entry.class
                )
            });
        assert!(
            entry.hit_die > 0,
            "class table '{path}': class '{}' has a non-positive hit die",
            entry.class
        );
        class_kinds.push(ClassKind {
            stat_modifiers,
            hit_die: entry.hit_die,
        });
    }
    ClassRegistry { class_kinds, by_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"[
        ( class: "warrior", stat_modifiers: [5, -2, -2, 2, 2, -1], hit_die: 9 ),
        ( class: "mage", stat_modifiers: [-2, 2, 2, 0, -1, 1], hit_die: 4 ),
    ]"#;

    #[test]
    fn ids_resolve_to_handles_and_kinds() {
        let class_registry = parse_class_registry("test", DOC);
        let warrior = class_registry.get_index("warrior").unwrap();
        let mage = class_registry.get_index("mage").unwrap();
        // Same id, same handle; different ids, different handles. Concrete
        // values are the internal table index: file order decides them,
        // and logic must never depend on them.
        assert_eq!(warrior, class_registry.get_index("warrior").unwrap());
        assert_ne!(warrior, mage);
        assert!(class_registry.get_index("rogue").is_none());
        // The kind a handle points to is the entry's birth data.
        let warrior_kind = class_registry.class_kind(warrior);
        assert_eq!(warrior_kind.stat_modifiers, [5, -2, -2, 2, 2, -1]);
        assert_eq!(warrior_kind.hit_die, 9);
        assert_eq!(class_registry.class_kind(mage).hit_die, 4);
    }

    #[test]
    #[should_panic(expected = "duplicate class id 'warrior'")]
    fn duplicate_id_panics() {
        parse_class_registry(
            "test",
            r#"[
                ( class: "warrior", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 9 ),
                ( class: "warrior", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 9 ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "empty class id")]
    fn empty_id_panics() {
        parse_class_registry(
            "test",
            r#"[ ( class: "", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 9 ) ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "has 5 stat modifiers (expected 6)")]
    fn wrong_modifier_count_panics() {
        parse_class_registry(
            "test",
            r#"[ ( class: "warrior", stat_modifiers: [5, -2, -2, 2, 2], hit_die: 9 ) ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "has a non-positive hit die")]
    fn zero_hit_die_panics() {
        parse_class_registry(
            "test",
            r#"[ ( class: "warrior", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 0 ) ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_class_registry("test", "this is not ron");
    }

    /// Spec-alignment test: the real vocabulary file on disk declares
    /// the warrior with its established birth data.
    #[test]
    fn vocabulary_declares_the_warrior() {
        let text = std::fs::read_to_string(CLASS_TABLE_PATH).unwrap();
        let class_registry = parse_class_registry(CLASS_TABLE_PATH, &text);
        let warrior = class_registry
            .get_index("warrior")
            .expect("warrior is declared");
        let warrior_kind = class_registry.class_kind(warrior);
        assert_eq!(warrior_kind.stat_modifiers, [5, -2, -2, 2, 2, -1]);
        assert_eq!(warrior_kind.hit_die, 9);
    }
}
