//! The monster registry: every monster id resolved to its runtime
//! handle and its game data. One theme, one file: the resource, its
//! construction from the vocabulary file, and the file's parsing and
//! validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::dice::utils::parse::parse_dice;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::monster::types::monster_blow::MonsterBlow;
use crate::core::monster::types::monster_entry::MonsterEntry;
use crate::core::monster::types::monster_kind::MonsterKind;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::speed::SPEED_RATE_TABLE;

/// The monster vocabulary loaded at startup.
pub const MONSTER_TABLE_PATH: &str = "data/core/monsters.ron";

/// Registry of all monsters, built once at startup from the vocabulary
/// file. `MonsterIndex` is the index into the internal table, assigned in
/// file order — an unstable runtime handle, never an identity: the same
/// monster's index changes when the file's entry order changes. Spawn
/// sites resolve ids to handles here; ids serve file references, error
/// messages, and save serialization only. Entries sit in file order in a
/// dense table: row storage keeps one lookup per monster and preserves
/// entry order for free.
#[derive(Resource)]
pub struct MonsterRegistry {
    monster_kinds: Vec<MonsterKind>,
    by_id: HashMap<String, MonsterIndex>,
}

impl MonsterRegistry {
    /// The handle for a monster id, if the id is declared. Ids resolve
    /// only at content boundaries (spawn sites, display bindings).
    pub fn get_index(&self, id: &str) -> Option<MonsterIndex> {
        self.by_id.get(id).copied()
    }

    /// The monster kind a handle points to. Callers hold valid handles
    /// by construction (checked at load), so a missing entry at runtime
    /// is a load-time bug, not a runtime case.
    pub fn monster_kind(&self, monster_index: MonsterIndex) -> &MonsterKind {
        &self.monster_kinds[monster_index.index()]
    }

    /// All declared monster ids with their handles, in unspecified
    /// order. Crate-internal: load validation that must cover every
    /// monster iterates here — coverage checks care about presence, not
    /// order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (MonsterIndex, &str)> {
        self.by_id.iter().map(|(id, index)| (*index, id.as_str()))
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
    let entries: Vec<MonsterEntry> = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(text)
        .unwrap_or_else(|e| panic!("monster table '{path}' is not valid RON: {e}"));
    let mut monster_kinds = Vec::with_capacity(entries.len());
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
        assert!(
            entry.speed < SPEED_RATE_TABLE.len(),
            "monster table '{path}': monster '{}' has speed {} outside the rate table",
            entry.monster,
            entry.speed
        );
        let hit_points = parse_dice(&entry.hit_points).unwrap_or_else(|e| {
            panic!(
                "monster table '{path}': monster '{}' has an invalid hit_points field: {e}",
                entry.monster
            )
        });
        let blows = entry
            .blows
            .into_iter()
            .map(|blow| {
                let damage = parse_dice(&blow.damage).unwrap_or_else(|e| {
                    panic!(
                        "monster table '{path}': monster '{}' has an invalid blow damage: {e}",
                        entry.monster
                    )
                });
                MonsterBlow { damage }
            })
            .collect();
        monster_kinds.push(MonsterKind {
            speed: Speed(entry.speed),
            hit_points,
            armor_class: entry.armor_class,
            level: entry.level,
            blows,
        });
    }
    MonsterRegistry {
        monster_kinds,
        by_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dice::types::dice::Dice;

    const DOC: &str = r#"[
        (
            monster: "giant_white_rat",
            speed: 110,
            hit_points: "2d2",
            armor_class: 7,
            level: 4,
            blows: [ ( damage: "1d3" ) ],
        ),
        (
            monster: "jackal",
            speed: 120,
            hit_points: "1d4",
            armor_class: 4,
            level: 2,
            blows: [ ( damage: "1d2" ) ],
        ),
    ]"#;

    #[test]
    fn ids_resolve_to_handles_and_kinds() {
        let monster_registry = parse_monster_registry("test", DOC);
        let rat = monster_registry.get_index("giant_white_rat").unwrap();
        let jackal = monster_registry.get_index("jackal").unwrap();
        // Same id, same handle; different ids, different handles. Concrete
        // values are the internal table index: file order decides them,
        // and logic must never depend on them.
        assert_eq!(rat, monster_registry.get_index("giant_white_rat").unwrap());
        assert_ne!(rat, jackal);
        assert!(monster_registry.get_index("wolf").is_none());
        // The kind a handle points to is the entry's game data: speed
        // plus the combat profile.
        assert_eq!(monster_registry.monster_kind(rat).speed, Speed(110));
        assert_eq!(
            monster_registry.monster_kind(rat).hit_points,
            Dice { n: 2, m: 2 }
        );
        assert_eq!(monster_registry.monster_kind(rat).armor_class, 7);
        assert_eq!(monster_registry.monster_kind(rat).level, 4);
        assert_eq!(
            monster_registry.monster_kind(rat).blows,
            vec![MonsterBlow {
                damage: Dice { n: 1, m: 3 }
            }]
        );
        assert_eq!(monster_registry.monster_kind(jackal).speed, Speed(120));
        assert_eq!(
            monster_registry.monster_kind(jackal).hit_points,
            Dice { n: 1, m: 4 }
        );
        assert_eq!(monster_registry.monster_kind(jackal).armor_class, 4);
        assert_eq!(monster_registry.monster_kind(jackal).level, 2);
        assert_eq!(
            monster_registry.monster_kind(jackal).blows,
            vec![MonsterBlow {
                damage: Dice { n: 1, m: 2 }
            }]
        );
    }

    #[test]
    fn iter_covers_every_declared_id() {
        let monster_registry = parse_monster_registry("test", DOC);
        let mut ids: Vec<&str> = monster_registry.iter().map(|(_, id)| id).collect();
        ids.sort_unstable();
        assert_eq!(ids, ["giant_white_rat", "jackal"]);
    }

    #[test]
    #[should_panic(expected = "duplicate monster id 'giant_white_rat'")]
    fn duplicate_id_panics() {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    speed: 110,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
                (
                    monster: "giant_white_rat",
                    speed: 110,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "empty monster id")]
    fn empty_id_panics() {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "",
                    speed: 110,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "outside the rate table")]
    fn speed_outside_rate_table_panics() {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    speed: 300,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_monster_registry("test", "this is not ron");
    }

    #[test]
    #[should_panic(expected = "missing field named `speed`")]
    fn missing_speed_is_a_format_error() {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "missing field named `hit_points`")]
    fn missing_combat_field_is_a_format_error() {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    speed: 110,
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "missing field named `blows`")]
    fn missing_blows_is_a_format_error() {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    speed: 110,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "invalid hit_points field")]
    fn invalid_hit_dice_panics_with_the_entry_id() {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    speed: 110,
                    hit_points: "2D2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
            ]"#,
        );
    }

    #[test]
    #[should_panic(expected = "invalid blow damage")]
    fn invalid_blow_dice_panics_with_the_entry_id() {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "giant_white_rat",
                    speed: 110,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "0d3" ) ],
                ),
            ]"#,
        );
    }

    /// Spec-alignment test: the real vocabulary file on disk declares
    /// the giant white rat at standard speed.
    #[test]
    fn vocabulary_declares_the_giant_white_rat() {
        let text = std::fs::read_to_string(MONSTER_TABLE_PATH).unwrap();
        let monster_registry = parse_monster_registry(MONSTER_TABLE_PATH, &text);
        let rat = monster_registry
            .get_index("giant_white_rat")
            .expect("giant_white_rat is declared");
        let rat_kind = monster_registry.monster_kind(rat);
        assert_eq!(rat_kind.speed, Speed(110));
        assert_eq!(rat_kind.hit_points, Dice { n: 2, m: 2 });
        assert_eq!(rat_kind.armor_class, 7);
        assert_eq!(rat_kind.level, 4);
        assert_eq!(
            rat_kind.blows,
            vec![MonsterBlow {
                damage: Dice { n: 1, m: 3 }
            }]
        );
    }
}
