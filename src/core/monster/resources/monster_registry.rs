//! The monster registry: every monster id resolved to its runtime
//! handle and its game data. One theme, one file: the resource, its
//! construction from the vocabulary file, and the file's parsing and
//! validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::creature::components::speed::Speed;
use crate::core::creature::resources::class_registry::ClassRegistry;
use crate::core::creature::resources::race_registry::RaceRegistry;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::monster::types::monster_entry::MonsterEntry;
use crate::core::monster::types::monster_kind::MonsterKind;
use crate::core::time::constants::action_point_rate::ACTION_POINT_RATES;

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
}

impl FromWorld for MonsterRegistry {
    /// Build from the vocabulary file, resolving races and classes
    /// against their vocabularies — pulling them into existence if not
    /// built yet, so registration order never matters.
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(MONSTER_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read monster table '{MONSTER_TABLE_PATH}': {e}"));
        world.get_resource_or_init::<RaceRegistry>();
        world.get_resource_or_init::<ClassRegistry>();
        // Both dependencies exist now (a pull builds them if missing);
        // shared reads suffice.
        let race_registry = world.resource::<RaceRegistry>();
        let class_registry = world.resource::<ClassRegistry>();
        parse_monster_registry(MONSTER_TABLE_PATH, &text, race_registry, class_registry)
    }
}

/// Parse and validate monster vocabulary text, resolving each entry's
/// race and class against their vocabularies. Split from file IO
/// (`FromWorld`) so tests can exercise it with inline documents.
pub(crate) fn parse_monster_registry(
    path: &str,
    text: &str,
    race_registry: &RaceRegistry,
    class_registry: &ClassRegistry,
) -> MonsterRegistry {
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
        let Some(race_index) = race_registry.get_index(&entry.race) else {
            panic!(
                "monster table '{path}': monster '{}' has unknown race '{}'",
                entry.monster, entry.race
            );
        };
        assert!(
            entry.speed < ACTION_POINT_RATES.len(),
            "monster table '{path}': monster '{}' has speed {} outside the rate table",
            entry.monster,
            entry.speed
        );
        let class_index = entry.class.map(|class| {
            class_registry.get_index(&class).unwrap_or_else(|| {
                panic!(
                    "monster table '{path}': monster '{}' has unknown class '{}'",
                    entry.monster, class
                )
            })
        });
        if let Some(unique_id) = &entry.unique_id {
            assert!(
                !unique_id.is_empty(),
                "monster table '{path}': monster '{}' has an empty unique id",
                entry.monster
            );
        }
        monster_kinds.push(MonsterKind {
            race: race_index,
            class: class_index,
            unique_id: entry.unique_id,
            speed: Speed(entry.speed),
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
    use crate::core::creature::resources::class_registry::{
        parse_class_registry, CLASS_TABLE_PATH,
    };
    use crate::core::creature::resources::race_registry::{parse_race_registry, RACE_TABLE_PATH};

    fn race_registry() -> RaceRegistry {
        parse_race_registry(
            "test",
            r#"[ ( race: "dog" ), ( race: "giant_white_rat" ) ]"#,
        )
    }

    fn class_registry() -> ClassRegistry {
        parse_class_registry("test", r#"[ ( class: "warrior" ) ]"#)
    }

    const DOC: &str = r#"[
        ( monster: "giant_white_rat", race: "giant_white_rat", speed: 110 ),
        ( monster: "grip", race: "dog", speed: 110, class: "warrior", unique_id: "grip" ),
    ]"#;

    #[test]
    fn ids_resolve_to_handles() {
        let monster_registry =
            parse_monster_registry("test", DOC, &race_registry(), &class_registry());
        let rat = monster_registry.get_index("giant_white_rat").unwrap();
        let grip = monster_registry.get_index("grip").unwrap();
        // Same id, same handle; different ids, different handles. Concrete
        // values are the internal table index: file order decides them,
        // and logic must never depend on them.
        assert_eq!(rat, monster_registry.get_index("giant_white_rat").unwrap());
        assert_ne!(rat, grip);
        assert!(monster_registry.get_index("wolf").is_none());
    }

    #[test]
    fn identities_resolve_per_entry() {
        let race_registry = race_registry();
        let class_registry = class_registry();
        let monster_registry = parse_monster_registry("test", DOC, &race_registry, &class_registry);
        let rat_kind =
            monster_registry.monster_kind(monster_registry.get_index("giant_white_rat").unwrap());
        assert_eq!(
            rat_kind.race,
            race_registry.get_index("giant_white_rat").unwrap()
        );
        assert_eq!(rat_kind.class, None);
        assert_eq!(rat_kind.unique_id, None);
        let grip_kind = monster_registry.monster_kind(monster_registry.get_index("grip").unwrap());
        assert_eq!(grip_kind.race, race_registry.get_index("dog").unwrap());
        assert_eq!(
            grip_kind.class,
            Some(class_registry.get_index("warrior").unwrap())
        );
        assert_eq!(grip_kind.unique_id.as_deref(), Some("grip"));
    }

    #[test]
    #[should_panic(expected = "duplicate monster id 'giant_white_rat'")]
    fn duplicate_id_panics() {
        let doc = r#"[
            ( monster: "giant_white_rat", race: "giant_white_rat", speed: 110 ),
            ( monster: "giant_white_rat", race: "giant_white_rat", speed: 110 ),
        ]"#;
        parse_monster_registry("test", doc, &race_registry(), &class_registry());
    }

    #[test]
    #[should_panic(expected = "empty monster id")]
    fn empty_id_panics() {
        parse_monster_registry(
            "test",
            r#"[ ( monster: "", race: "giant_white_rat", speed: 110 ) ]"#,
            &race_registry(),
            &class_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "has unknown race 'wolf'")]
    fn unknown_race_panics() {
        parse_monster_registry(
            "test",
            r#"[ ( monster: "giant_white_rat", race: "wolf", speed: 110 ) ]"#,
            &race_registry(),
            &class_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn missing_speed_panics() {
        parse_monster_registry(
            "test",
            r#"[ ( monster: "giant_white_rat", race: "giant_white_rat" ) ]"#,
            &race_registry(),
            &class_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "outside the rate table")]
    fn speed_outside_rate_table_panics() {
        parse_monster_registry(
            "test",
            r#"[ ( monster: "giant_white_rat", race: "giant_white_rat", speed: 300 ) ]"#,
            &race_registry(),
            &class_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "has unknown class 'mage'")]
    fn unknown_class_panics() {
        parse_monster_registry(
            "test",
            r#"[ ( monster: "giant_white_rat", race: "giant_white_rat", speed: 110, class: "mage" ) ]"#,
            &race_registry(),
            &class_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "has an empty unique id")]
    fn empty_unique_id_panics() {
        parse_monster_registry(
            "test",
            r#"[ ( monster: "giant_white_rat", race: "giant_white_rat", speed: 110, unique_id: "" ) ]"#,
            &race_registry(),
            &class_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_monster_registry(
            "test",
            "this is not ron",
            &race_registry(),
            &class_registry(),
        );
    }

    /// Spec-alignment test: the real vocabulary file on disk declares the
    /// giant white rat, and its race resolves against the real race
    /// vocabulary.
    #[test]
    fn vocabulary_declares_the_giant_white_rat() {
        let race_text = std::fs::read_to_string(RACE_TABLE_PATH).unwrap();
        let race_registry = parse_race_registry(RACE_TABLE_PATH, &race_text);
        let class_text = std::fs::read_to_string(CLASS_TABLE_PATH).unwrap();
        let class_registry = parse_class_registry(CLASS_TABLE_PATH, &class_text);
        let text = std::fs::read_to_string(MONSTER_TABLE_PATH).unwrap();
        let monster_registry =
            parse_monster_registry(MONSTER_TABLE_PATH, &text, &race_registry, &class_registry);
        let rat_kind =
            monster_registry.monster_kind(monster_registry.get_index("giant_white_rat").unwrap());
        assert_eq!(
            rat_kind.race,
            race_registry.get_index("giant_white_rat").unwrap()
        );
    }
}
