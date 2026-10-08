//! The creature figure registry: every creature identity resolved to the
//! figure it presents. One theme, one file: the resource, its
//! construction from the binding file, and the file's parsing and
//! validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::class::components::class_index::ClassIndex;
use crate::core::class::resources::class_registry::ClassRegistry;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::monster::resources::monster_registry::MonsterRegistry;
use crate::core::race::components::race_index::RaceIndex;
use crate::core::race::resources::race_registry::RaceRegistry;
use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::figure::types::creature_figure_entry::CreatureFigureEntry;

/// The creature figure binding file loaded at startup.
pub const CREATURE_FIGURES_PATH: &str = "data/graphic/creature_figures.ron";

/// Registry of all creature figure bindings, built once at startup from
/// the binding file. Monsters resolve through their kind; humanoid
/// creatures resolve through race and class, the class pair taking
/// precedence over the bare race default — and a resolution may find
/// nothing: a race of classless creatures (animals) needs a race-key
/// default, while a race whose members always bear a class (people) may
/// exist only through race+class entries, so a classless identity
/// legitimately has no binding and the caller fails loudly at attach.
/// Load-time validation guarantees every declared race and every
/// declared monster is reachable by at least one entry.
#[derive(Resource)]
pub struct CreatureFigureRegistry {
    by_monster: HashMap<MonsterIndex, FigureIndex>,
    by_race_and_class: HashMap<(RaceIndex, ClassIndex), FigureIndex>,
    by_race: HashMap<RaceIndex, FigureIndex>,
}

impl CreatureFigureRegistry {
    /// The figure a monster presents, resolved through its kind.
    pub fn get_monster_figure(&self, monster_index: MonsterIndex) -> Option<FigureIndex> {
        self.by_monster.get(&monster_index).copied()
    }

    /// The figure a humanoid creature presents: the race+class pair when
    /// bound, else the race default.
    pub fn get_race_figure(
        &self,
        race_index: RaceIndex,
        class_index: Option<ClassIndex>,
    ) -> Option<FigureIndex> {
        if let Some(class_index) = class_index {
            if let Some(figure_index) = self.by_race_and_class.get(&(race_index, class_index)) {
                return Some(*figure_index);
            }
        }
        self.by_race.get(&race_index).copied()
    }
}

impl FromWorld for CreatureFigureRegistry {
    /// Build from the binding file, resolving keys against the race,
    /// class, and monster vocabularies and the figure table. All are
    /// pulled into existence if not built yet, so registration order
    /// never matters.
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(CREATURE_FIGURES_PATH).unwrap_or_else(|e| {
            panic!("cannot read creature figure bindings '{CREATURE_FIGURES_PATH}': {e}")
        });
        world.get_resource_or_init::<RaceRegistry>();
        world.get_resource_or_init::<ClassRegistry>();
        world.get_resource_or_init::<MonsterRegistry>();
        world.get_resource_or_init::<FigureRegistry>();
        // All dependencies exist now (a pull builds them if missing);
        // shared reads suffice.
        build_creature_figure_registry(
            CREATURE_FIGURES_PATH,
            &text,
            world.resource::<RaceRegistry>(),
            world.resource::<ClassRegistry>(),
            world.resource::<MonsterRegistry>(),
            world.resource::<FigureRegistry>(),
        )
    }
}

/// Resolve and validate binding text against the vocabularies and the
/// figure table. Split from file IO (`FromWorld`) so tests can exercise
/// it with inline documents.
fn build_creature_figure_registry(
    path: &str,
    text: &str,
    race_registry: &RaceRegistry,
    class_registry: &ClassRegistry,
    monster_registry: &MonsterRegistry,
    figure_registry: &FigureRegistry,
) -> CreatureFigureRegistry {
    let entries: Vec<CreatureFigureEntry> = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(text)
        .unwrap_or_else(|e| panic!("creature figure bindings '{path}' is not valid RON: {e}"));
    let mut by_monster = HashMap::new();
    let mut by_race_and_class = HashMap::new();
    let mut by_race = HashMap::new();
    for entry in entries {
        let figure_index = figure_registry.get_index(&entry.figure).unwrap_or_else(|| {
            panic!(
                "creature figure bindings '{path}': unknown figure '{}'",
                entry.figure
            )
        });
        // The present fields decide the key shape: monster alone,
        // race+class, or race alone; every other combination is a
        // format error.
        match (entry.monster, entry.race, entry.class) {
            (Some(monster), None, None) => {
                let monster_index = monster_registry.get_index(&monster).unwrap_or_else(|| {
                    panic!("creature figure bindings '{path}': unknown monster '{monster}'")
                });
                assert!(
                    by_monster.insert(monster_index, figure_index).is_none(),
                    "creature figure bindings '{path}': duplicate binding for monster '{monster}'"
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
                            by_race_and_class
                                .insert((race_index, class_index), figure_index)
                                .is_none(),
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
            (monster, race, class) => {
                panic!(
                    "creature figure bindings '{path}': invalid key shape (monster: {monster:?}, race: {race:?}, class: {class:?}) — expected monster alone, race+class, or race alone"
                );
            }
        }
    }
    // Every declared race must be reachable: a race-key default, or at
    // least one race+class entry (a people-race whose members always
    // bear a class). Every declared monster must be reachable through
    // its kind. A missing entry means a forgotten binding.
    for (race_index, id) in race_registry.iter() {
        assert!(
            by_race.contains_key(&race_index)
                || by_race_and_class.keys().any(|(race, _)| *race == race_index),
            "creature figure bindings '{path}': race '{id}' has no binding (needs a race-key default or at least one race+class entry)"
        );
    }
    for (monster_index, id) in monster_registry.iter() {
        assert!(
            by_monster.contains_key(&monster_index),
            "creature figure bindings '{path}': monster '{id}' has no binding (needs a monster-key entry)"
        );
    }
    CreatureFigureRegistry {
        by_monster,
        by_race_and_class,
        by_race,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::class::resources::class_registry::parse_class_registry;
    use crate::core::monster::resources::monster_registry::parse_monster_registry;
    use crate::core::race::resources::race_registry::parse_race_registry;

    fn race_registry() -> RaceRegistry {
        parse_race_registry(
            "test",
            r#"[ ( race: "human", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 10 ) ]"#,
        )
    }

    fn class_registry() -> ClassRegistry {
        parse_class_registry(
            "test",
            r#"[ ( class: "warrior", stat_modifiers: [0, 0, 0, 0, 0, 0], hit_die: 9 ) ]"#,
        )
    }

    fn monster_registry() -> MonsterRegistry {
        parse_monster_registry(
            "test",
            r#"[
                (
                    monster: "rat",
                    speed: 110,
                    hit_points: "2d2",
                    armor_class: 7,
                    level: 4,
                    blows: [ ( damage: "1d3" ) ],
                ),
            ]"#,
        )
    }

    fn figure_registry() -> FigureRegistry {
        FigureRegistry::for_test(&["warrior", "rat"])
    }

    /// Every layer populated: the monster kind resolves through its key,
    /// the classed human through the pair, a classless human through the
    /// race default.
    const DOC: &str = r#"[
        ( monster: "rat", figure: "rat" ),
        ( race: "human", class: "warrior", figure: "warrior" ),
        ( race: "human", figure: "rat" ),
    ]"#;

    fn registry() -> CreatureFigureRegistry {
        build_creature_figure_registry(
            "test",
            DOC,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        )
    }

    #[test]
    fn monster_key_resolves_the_kind() {
        let monster_registry = monster_registry();
        let rat = monster_registry.get_index("rat").unwrap();
        let registry = registry();
        assert_eq!(
            registry.get_monster_figure(rat),
            Some(figure_registry().get_index("rat").unwrap())
        );
    }

    #[test]
    fn race_and_class_key_wins_over_race_default() {
        let race_registry = race_registry();
        let class_registry = class_registry();
        let human = race_registry.get_index("human").unwrap();
        let warrior = class_registry.get_index("warrior").unwrap();
        let registry = registry();
        assert_eq!(
            registry.get_race_figure(human, Some(warrior)),
            Some(figure_registry().get_index("warrior").unwrap())
        );
    }

    #[test]
    fn race_default_is_the_fallback() {
        let race_registry = race_registry();
        let human = race_registry.get_index("human").unwrap();
        // A human without a class falls to the human default.
        assert_eq!(
            registry().get_race_figure(human, None),
            Some(figure_registry().get_index("rat").unwrap())
        );
    }

    /// A people-race with only a race+class entry: classed members
    /// resolve, classless members match nothing.
    const RACE_AND_CLASS_ONLY_DOC: &str = r#"[
        ( race: "human", class: "warrior", figure: "warrior" ),
        ( monster: "rat", figure: "rat" ),
    ]"#;

    #[test]
    fn race_and_class_only_race_leaves_classless_members_unmatched() {
        let registry = build_creature_figure_registry(
            "test",
            RACE_AND_CLASS_ONLY_DOC,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
        let race_registry = race_registry();
        let class_registry = class_registry();
        let human = race_registry.get_index("human").unwrap();
        let warrior = class_registry.get_index("warrior").unwrap();
        assert_eq!(
            registry.get_race_figure(human, Some(warrior)),
            Some(figure_registry().get_index("warrior").unwrap())
        );
        assert_eq!(registry.get_race_figure(human, None), None);
    }

    #[test]
    #[should_panic(expected = "unknown monster 'wolf'")]
    fn unknown_monster_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( monster: "wolf", figure: "rat" ),
                ( race: "human", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "unknown race 'elf'")]
    fn unknown_race_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( race: "elf", figure: "rat" ),
                ( monster: "rat", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "unknown class 'mage'")]
    fn unknown_class_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( race: "human", class: "mage", figure: "rat" ),
                ( monster: "rat", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "unknown figure 'wolf'")]
    fn unknown_figure_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( race: "human", figure: "wolf" ),
                ( monster: "rat", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "invalid key shape")]
    fn class_without_race_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( class: "warrior", figure: "rat" ),
                ( monster: "rat", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "invalid key shape")]
    fn monster_mixed_with_race_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( monster: "rat", race: "human", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "duplicate binding for monster 'rat'")]
    fn duplicate_monster_key_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( monster: "rat", figure: "rat" ),
                ( monster: "rat", figure: "warrior" ),
                ( race: "human", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "duplicate binding for race 'human'")]
    fn duplicate_race_key_panics() {
        build_creature_figure_registry(
            "test",
            r#"[
                ( race: "human", figure: "warrior" ),
                ( race: "human", figure: "rat" ),
                ( monster: "rat", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
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
                ( race: "human", class: "warrior", figure: "rat" ),
                ( monster: "rat", figure: "rat" ),
            ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "race 'human' has no binding")]
    fn race_without_any_binding_panics() {
        // The race is declared but bound nowhere — neither a race
        // default nor any race+class entry mentions it.
        build_creature_figure_registry(
            "test",
            r#"[ ( monster: "rat", figure: "rat" ) ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
            &figure_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "monster 'rat' has no binding")]
    fn monster_without_any_binding_panics() {
        // The kind is declared but no monster-key entry mentions it.
        build_creature_figure_registry(
            "test",
            r#"[ ( race: "human", figure: "warrior" ) ]"#,
            &race_registry(),
            &class_registry(),
            &monster_registry(),
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
            &monster_registry(),
            &figure_registry(),
        );
    }

    /// Spec-alignment test: the real binding file on disk resolves the
    /// player through the race+class key and the rat through its kind.
    #[test]
    fn real_bindings_reproduce_the_established_presentation() {
        use crate::core::class::resources::class_registry::CLASS_TABLE_PATH;
        use crate::core::monster::resources::monster_registry::MONSTER_TABLE_PATH;
        use crate::core::race::resources::race_registry::RACE_TABLE_PATH;
        use crate::frontend::display::figure::resources::figure_registry::parse_figure_entries;
        use crate::frontend::display::figure::resources::figure_registry::FIGURE_TABLE_PATH;

        let race_text = std::fs::read_to_string(RACE_TABLE_PATH).unwrap();
        let race_registry = parse_race_registry(RACE_TABLE_PATH, &race_text);
        let class_text = std::fs::read_to_string(CLASS_TABLE_PATH).unwrap();
        let class_registry = parse_class_registry(CLASS_TABLE_PATH, &class_text);
        let monster_text = std::fs::read_to_string(MONSTER_TABLE_PATH).unwrap();
        let monster_registry = parse_monster_registry(MONSTER_TABLE_PATH, &monster_text);
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
            &monster_registry,
            &figure_registry,
        );
        let human = race_registry.get_index("human").unwrap();
        let warrior = class_registry.get_index("warrior").unwrap();
        let rat = monster_registry.get_index("giant_white_rat").unwrap();
        let warrior_figure = figure_registry.get_index("warrior").unwrap();
        let rat_figure = figure_registry.get_index("giant_white_rat").unwrap();
        // The player resolves through the race+class key (a classless
        // human has no figure — there is no naked-human art); the rat
        // through its kind.
        assert_eq!(
            registry.get_race_figure(human, Some(warrior)),
            Some(warrior_figure)
        );
        assert_eq!(registry.get_race_figure(human, None), None);
        assert_eq!(registry.get_monster_figure(rat), Some(rat_figure));
    }
}
