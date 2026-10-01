//! The creature figure registry: every creature identity resolved to the
//! figure it presents. One theme, one file: the resource, its
//! construction from the binding file, and the file's parsing and
//! validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::creature::components::class_index::ClassIndex;
use crate::core::creature::components::race_index::RaceIndex;
use crate::core::creature::components::unique_index::UniqueIndex;
use crate::core::creature::resources::class_registry::ClassRegistry;
use crate::core::creature::resources::race_registry::RaceRegistry;
use crate::core::creature::resources::unique_registry::UniqueRegistry;
use crate::frontend::display::creature::types::creature_figure_entry::CreatureFigureEntry;
use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;

/// The creature figure binding file loaded at startup.
pub const CREATURE_FIGURES_PATH: &str = "data/graphic/creature_figures.ron";

/// Registry of all creature figure bindings, built once at startup from
/// the binding file. Resolution takes the most specific matching key:
/// unique first, then race+class, then race alone — and may find
/// nothing: a race of classless creatures (animals) needs a race-key
/// default, while a race whose members always bear a class (people) may
/// exist only through race+class entries, so a classless identity
/// legitimately has no binding and the caller fails loudly at attach.
/// Load-time validation guarantees only that every declared race is
/// reachable by at least one entry.
#[derive(Resource)]
pub struct CreatureFigureRegistry {
    by_unique: HashMap<UniqueIndex, FigureIndex>,
    by_race_and_class: HashMap<(RaceIndex, ClassIndex), FigureIndex>,
    by_race: HashMap<RaceIndex, FigureIndex>,
}

impl CreatureFigureRegistry {
    /// The figure a creature presents, given its identity handles —
    /// `None` when no key matches (a classless member of a race that
    /// only defines class-keyed figures, for instance).
    pub fn figure(
        &self,
        race_index: RaceIndex,
        class_index: Option<ClassIndex>,
        unique_index: Option<UniqueIndex>,
    ) -> Option<FigureIndex> {
        if let Some(unique_index) = unique_index {
            if let Some(figure_index) = self.by_unique.get(&unique_index) {
                return Some(*figure_index);
            }
        }
        if let Some(class_index) = class_index {
            if let Some(figure_index) = self.by_race_and_class.get(&(race_index, class_index)) {
                return Some(*figure_index);
            }
        }
        self.by_race.get(&race_index).copied()
    }
}

impl FromWorld for CreatureFigureRegistry {
    /// Build from the binding file, resolving ids against the identity
    /// vocabularies and the figure table. Vocabulary registries are
    /// pulled into existence if not built yet, so registration order
    /// never matters; the unique registry is assembled by the core root
    /// (from content vocabularies) and must already exist — the core
    /// side registers before the display side.
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(CREATURE_FIGURES_PATH).unwrap_or_else(|e| {
            panic!("cannot read creature figure bindings '{CREATURE_FIGURES_PATH}': {e}")
        });
        world.get_resource_or_init::<RaceRegistry>();
        world.get_resource_or_init::<ClassRegistry>();
        world.get_resource_or_init::<FigureRegistry>();
        // All dependencies exist now (a pull builds them if missing);
        // shared reads suffice.
        build_creature_figure_registry(
            CREATURE_FIGURES_PATH,
            &text,
            world.resource::<RaceRegistry>(),
            world.resource::<ClassRegistry>(),
            world.resource::<UniqueRegistry>(),
            world.resource::<FigureRegistry>(),
        )
    }
}

/// Resolve and validate binding text against the vocabularies and the
/// figure table. Split from file IO (`FromWorld`) so tests can exercise
/// it with inline documents.
pub(crate) fn build_creature_figure_registry(
    path: &str,
    text: &str,
    race_registry: &RaceRegistry,
    class_registry: &ClassRegistry,
    unique_registry: &UniqueRegistry,
    figure_registry: &FigureRegistry,
) -> CreatureFigureRegistry {
    let entries: Vec<CreatureFigureEntry> = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(text)
        .unwrap_or_else(|e| panic!("creature figure bindings '{path}' is not valid RON: {e}"));
    let mut by_unique = HashMap::new();
    let mut by_race_and_class = HashMap::new();
    let mut by_race = HashMap::new();
    for entry in entries {
        let figure_index = figure_registry.get_index(&entry.figure).unwrap_or_else(|| {
            panic!(
                "creature figure bindings '{path}': unknown figure '{}'",
                entry.figure
            )
        });
        // The present fields decide the key shape: unique alone,
        // race+class, or race alone; every other combination is a format
        // error.
        match (entry.unique, entry.race, entry.class) {
            (Some(unique), None, None) => {
                let unique_index = unique_registry.get_index(&unique).unwrap_or_else(|| {
                    panic!("creature figure bindings '{path}': unknown unique '{unique}'")
                });
                assert!(
                    by_unique.insert(unique_index, figure_index).is_none(),
                    "creature figure bindings '{path}': duplicate binding for unique '{unique}'"
                );
            }
            (None, Some(race), class) => {
                let race_index = race_registry.get_index(&race).unwrap_or_else(|| {
                    panic!("creature figure bindings '{path}': unknown race '{race}'")
                });
                match class {
                    Some(class) => {
                        let class_index = class_registry.get_index(&class).unwrap_or_else(|| {
                            panic!("creature figure bindings '{path}': unknown class '{class}'")
                        });
                        assert!(
                            by_race_and_class.insert((race_index, class_index), figure_index).is_none(),
                            "creature figure bindings '{path}': duplicate binding for race '{race}' with class '{class}'"
                        );
                    }
                    None => {
                        assert!(
                            by_race.insert(race_index, figure_index).is_none(),
                            "creature figure bindings '{path}': duplicate binding for race '{race}'"
                        );
                    }
                }
            }
            (unique, race, class) => {
                panic!(
                    "creature figure bindings '{path}': invalid key shape (unique: {unique:?}, race: {race:?}, class: {class:?}) — expected unique alone, race+class, or race alone"
                );
            }
        }
    }
    // Every declared race must be reachable: a race-key default, or at
    // least one race+class entry (a people-race whose members always
    // bear a class). A race with neither means a forgotten binding.
    for (race_index, id) in race_registry.iter() {
        assert!(
            by_race.contains_key(&race_index)
                || by_race_and_class.keys().any(|(race, _)| *race == race_index),
            "creature figure bindings '{path}': race '{id}' has no binding (needs a race-key default or at least one race+class entry)"
        );
    }
    CreatureFigureRegistry {
        by_unique,
        by_race_and_class,
        by_race,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::creature::resources::class_registry::parse_class_registry;
    use crate::core::creature::resources::race_registry::parse_race_registry;
    use crate::core::creature::resources::unique_registry::UniqueRegistry;

    fn race_registry() -> RaceRegistry {
        parse_race_registry("test", r#"[ ( race: "human" ), ( race: "rat" ) ]"#)
    }

    fn class_registry() -> ClassRegistry {
        parse_class_registry("test", r#"[ ( class: "warrior" ) ]"#)
    }

    fn unique_registry() -> UniqueRegistry {
        let mut unique_registry = UniqueRegistry::empty();
        unique_registry.extend(vec!["grip".to_string()]);
        unique_registry
    }

    fn figure_registry() -> FigureRegistry {
        FigureRegistry::for_test(&["warrior", "rat", "grip"])
    }

    /// Every layer populated: unique beats race+class, race+class beats the
    /// race default.
    const DOC: &str = r#"[
        ( unique: "grip", figure: "grip" ),
        ( race: "human", class: "warrior", figure: "warrior" ),
        ( race: "human", figure: "rat" ),
        ( race: "rat", figure: "rat" ),
    ]"#;

    fn registry() -> CreatureFigureRegistry {
        build_creature_figure_registry(
            "test",
            DOC,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        )
    }

    #[test]
    fn unique_key_wins_over_less_specific_keys() {
        let registry = registry();
        let grip = unique_registry().get_index("grip").unwrap();
        let human = race_registry().get_index("human").unwrap();
        let warrior = class_registry().get_index("warrior").unwrap();
        assert_eq!(
            registry.figure(human, Some(warrior), Some(grip)),
            Some(figure_registry().get_index("grip").unwrap())
        );
    }

    #[test]
    fn race_and_class_key_wins_over_race_default() {
        let registry = registry();
        let human = race_registry().get_index("human").unwrap();
        let warrior = class_registry().get_index("warrior").unwrap();
        assert_eq!(
            registry.figure(human, Some(warrior), None),
            Some(figure_registry().get_index("warrior").unwrap())
        );
    }

    #[test]
    fn race_default_is_the_fallback() {
        let registry = registry();
        let human = race_registry().get_index("human").unwrap();
        let rat = race_registry().get_index("rat").unwrap();
        // A human without a class falls to the human default; the rat
        // carries no class at all.
        assert_eq!(
            registry.figure(human, None, None),
            Some(figure_registry().get_index("rat").unwrap())
        );
        assert_eq!(
            registry.figure(rat, None, None),
            Some(figure_registry().get_index("rat").unwrap())
        );
    }

    /// A people-race with only a race+class entry: classed members resolve,
    /// classless members match nothing.
    const RACE_AND_CLASS_ONLY_DOC: &str = r#"[
        ( race: "human", class: "warrior", figure: "warrior" ),
        ( race: "rat", figure: "rat" ),
    ]"#;

    #[test]
    fn race_and_class_only_race_leaves_classless_members_unmatched() {
        let registry = build_creature_figure_registry(
            "test",
            RACE_AND_CLASS_ONLY_DOC,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
        let human = race_registry().get_index("human").unwrap();
        let warrior = class_registry().get_index("warrior").unwrap();
        assert_eq!(
            registry.figure(human, Some(warrior), None),
            Some(figure_registry().get_index("warrior").unwrap())
        );
        assert_eq!(registry.figure(human, None, None), None);
    }

    #[test]
    #[should_panic(expected = "unknown unique 'wolf'")]
    fn unknown_unique_panics() {
        build_creature_figure_registry(
            "test",
            r#"[ ( unique: "wolf", figure: "rat" ), ( race: "human", figure: "rat" ), ( race: "rat", figure: "rat" ) ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "unknown race 'elf'")]
    fn unknown_race_panics() {
        build_creature_figure_registry(
            "test",
            r#"[ ( race: "elf", figure: "rat" ) ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "unknown class 'mage'")]
    fn unknown_class_panics() {
        build_creature_figure_registry(
            "test",
            r#"[ ( race: "human", class: "mage", figure: "rat" ) ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "unknown figure 'wolf'")]
    fn unknown_figure_panics() {
        build_creature_figure_registry(
            "test",
            r#"[ ( race: "human", figure: "wolf" ) ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "invalid key shape")]
    fn class_without_race_panics() {
        build_creature_figure_registry(
            "test",
            r#"[ ( class: "warrior", figure: "rat" ) ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "invalid key shape")]
    fn unique_mixed_with_race_panics() {
        build_creature_figure_registry(
            "test",
            r#"[ ( unique: "grip", race: "rat", figure: "rat" ) ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "duplicate binding for unique 'grip'")]
    fn duplicate_unique_key_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( unique: "grip", figure: "grip" ),
                ( unique: "grip", figure: "rat" ),
                ( race: "human", figure: "rat" ),
                ( race: "rat", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "duplicate binding for race 'human'")]
    fn duplicate_race_key_panics() {
        build_creature_figure_registry(
            "test",
            r#"[ ( race: "human", figure: "warrior" ), ( race: "human", figure: "rat" ), ( race: "rat", figure: "rat" ) ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "duplicate binding for race 'human' with class 'warrior'")]
    fn duplicate_race_and_class_key_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( race: "human", class: "warrior", figure: "warrior" ),
                ( race: "human", class: "warrior", figure: "warrior" ),
                ( race: "human", figure: "warrior" ),
                ( race: "rat", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "race 'rat' has no binding")]
    fn race_without_any_binding_panics() {
        // The rat is declared but bound nowhere — neither a race default
        // nor any race+class entry mentions it.
        build_creature_figure_registry(
            "test",
            r#"[ ( race: "human", class: "warrior", figure: "warrior" ) ]"#,
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        build_creature_figure_registry(
            "test",
            "this is not ron",
            &race_registry(),
            &class_registry(),
            &unique_registry(),
            &figure_registry(),
        );
    }

    /// Spec-alignment test: the real binding file on disk resolves the
    /// hero and the rat to their established figures.
    #[test]
    fn real_bindings_reproduce_the_established_presentation() {
        use crate::core::creature::resources::class_registry::CLASS_TABLE_PATH;
        use crate::core::creature::resources::race_registry::RACE_TABLE_PATH;
        use crate::frontend::display::figure::resources::figure_registry::parse_figure_entries;
        use crate::frontend::display::figure::resources::figure_registry::FIGURE_TABLE_PATH;

        let race_text = std::fs::read_to_string(RACE_TABLE_PATH).unwrap();
        let race_registry = parse_race_registry(RACE_TABLE_PATH, &race_text);
        let class_text = std::fs::read_to_string(CLASS_TABLE_PATH).unwrap();
        let class_registry = parse_class_registry(CLASS_TABLE_PATH, &class_text);
        let unique_registry = UniqueRegistry::empty();
        let figure_text = std::fs::read_to_string(FIGURE_TABLE_PATH).unwrap();
        let figure_entries = parse_figure_entries(FIGURE_TABLE_PATH, &figure_text);
        let figure_ids: Vec<&str> = figure_entries.iter().map(|e| e.figure.as_str()).collect();
        let figure_registry = FigureRegistry::for_test(&figure_ids);

        let text = std::fs::read_to_string(CREATURE_FIGURES_PATH).unwrap();
        let registry = build_creature_figure_registry(
            CREATURE_FIGURES_PATH,
            &text,
            &race_registry,
            &class_registry,
            &unique_registry,
            &figure_registry,
        );
        let human = race_registry.get_index("human").unwrap();
        let warrior = class_registry.get_index("warrior").unwrap();
        let rat = race_registry.get_index("giant_white_rat").unwrap();
        let warrior_figure = figure_registry.get_index("warrior").unwrap();
        let rat_figure = figure_registry.get_index("giant_white_rat").unwrap();
        // The hero resolves through the race+class key (a classless human has
        // no figure — there is no naked-human art); the rat through its
        // race default.
        assert_eq!(
            registry.figure(human, Some(warrior), None),
            Some(warrior_figure)
        );
        assert_eq!(registry.figure(human, None, None), None);
        assert_eq!(registry.figure(rat, None, None), Some(rat_figure));
    }
}
