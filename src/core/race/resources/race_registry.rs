//! The race registry: every playable race resolved to its runtime
//! handle and its birth data. One theme, one file: the resource, its
//! construction from the vocabulary file, and the file's parsing and
//! validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::race::components::race_index::RaceIndex;
use crate::core::race::types::race_entry::RaceEntry;
use crate::core::race::types::race_kind::RaceKind;

/// The race vocabulary loaded at startup.
pub const RACE_TABLE_PATH: &str = "data/core/races.ron";

/// How many statistic modifiers one race entry carries: one per
/// statistic.
const STAT_MODIFIER_COUNT: usize = 6;

/// Registry of all races, built once at startup from the vocabulary
/// file. `RaceIndex` is the index into the internal table, assigned in
/// file order — an unstable runtime handle, never an identity: the same
/// race's index changes when the file's entry order changes. Spawn
/// sites resolve ids to handles here; ids serve file references, error
/// messages, and save serialization only. Entries sit in file order in
/// a dense table: row storage keeps one lookup per race and preserves
/// entry order for free.
#[derive(Resource)]
pub struct RaceRegistry {
    race_kinds: Vec<RaceKind>,
    by_id: HashMap<String, RaceIndex>,
}

impl RaceRegistry {
    /// The handle for a race id, if the id is declared. Ids resolve
    /// only at content boundaries (spawn sites, display bindings).
    pub fn get_index(&self, id: &str) -> Option<RaceIndex> {
        self.by_id.get(id).copied()
    }

    /// The race kind a handle points to: the race's birth data. Callers
    /// hold valid handles by construction (checked at load), so a
    /// missing entry at runtime is a load-time bug, not a runtime case.
    pub fn race_kind(&self, race_index: RaceIndex) -> &RaceKind {
        &self.race_kinds[race_index.index()]
    }

    /// All declared race ids with their handles, in unspecified order.
    /// Crate-internal: load validation that must cover every race
    /// iterates here — coverage checks care about presence, not order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (RaceIndex, &str)> {
        self.by_id.iter().map(|(id, index)| (*index, id.as_str()))
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
    let mut race_kinds = Vec::with_capacity(entries.len());
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
        let modifier_count = entry.stat_modifiers.len();
        let stat_modifiers: [i32; STAT_MODIFIER_COUNT] = entry
            .stat_modifiers
            .try_into()
            .unwrap_or_else(|_| {
                panic!(
                    "race table '{path}': race '{}' has {modifier_count} stat modifiers (expected {STAT_MODIFIER_COUNT})",
                    entry.race
                )
            });
        assert!(
            entry.hit_die > 0,
            "race table '{path}': race '{}' has a non-positive hit die",
            entry.race
        );
        race_kinds.push(RaceKind {
            stat_modifiers,
            hit_die: entry.hit_die,
        });
    }
    RaceRegistry { race_kinds, by_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"[
        ( race: "human", stat_modifiers: [1, 0, 0, 0, 0, -1], hit_die: 10 ),
        ( race: "elf", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 8 ),
    ]"#;

    #[test]
    fn ids_resolve_to_handles_and_kinds() {
        let race_registry = parse_race_registry("test", DOC);
        let human = race_registry.get_index("human").unwrap();
        let elf = race_registry.get_index("elf").unwrap();
        // Same id, same handle; different ids, different handles. Concrete
        // values are the internal table index: file order decides them,
        // and logic must never depend on them.
        assert_eq!(human, race_registry.get_index("human").unwrap());
        assert_ne!(human, elf);
        assert!(race_registry.get_index("orc").is_none());
        // The kind a handle points to is the entry's birth data.
        let human_kind = race_registry.race_kind(human);
        assert_eq!(human_kind.stat_modifiers, [1, 0, 0, 0, 0, -1]);
        assert_eq!(human_kind.hit_die, 10);
        assert_eq!(race_registry.race_kind(elf).hit_die, 8);
    }

    #[test]
    fn iter_covers_every_declared_id() {
        let race_registry = parse_race_registry("test", DOC);
        let mut ids: Vec<&str> = race_registry.iter().map(|(_, id)| id).collect();
        ids.sort_unstable();
        assert_eq!(ids, ["elf", "human"]);
    }

    #[test]
    #[should_panic(expected = "duplicate race id 'human'")]
    fn duplicate_id_panics() {
        parse_race_registry(
            "test",
            r#"[
                ( race: "human", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 10 ),
                ( race: "human", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 10 ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "empty race id")]
    fn empty_id_panics() {
        parse_race_registry(
            "test",
            r#"[ ( race: "", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 10 ) ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "has 2 stat modifiers (expected 6)")]
    fn wrong_modifier_count_panics() {
        parse_race_registry(
            "test",
            r#"[ ( race: "human", stat_modifiers: [0, 0], hit_die: 10 ) ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "has a non-positive hit die")]
    fn zero_hit_die_panics() {
        parse_race_registry(
            "test",
            r#"[ ( race: "human", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 0 ) ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_race_registry("test", "this is not ron");
    }

    /// Spec-alignment test: the real vocabulary file on disk declares
    /// human with its established birth data — and nothing else. Races
    /// are playable species; monster families are not races.
    #[test]
    fn vocabulary_declares_human() {
        let text = std::fs::read_to_string(RACE_TABLE_PATH).unwrap();
        let race_registry = parse_race_registry(RACE_TABLE_PATH, &text);
        let human = race_registry.get_index("human").expect("human is declared");
        let human_kind = race_registry.race_kind(human);
        assert_eq!(human_kind.stat_modifiers, [0, 0, 0, 0, 0, 0]);
        assert_eq!(human_kind.hit_die, 10);
        let ids: Vec<&str> = race_registry.iter().map(|(_, id)| id).collect();
        assert_eq!(ids, ["human"]);
    }
}
