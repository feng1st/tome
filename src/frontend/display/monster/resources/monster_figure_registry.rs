//! The monster figure registry: every monster resolved to the figure it
//! presents. One theme, one file: the resource, its construction from
//! the binding file, and the file's parsing and validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::figure::components::figure_index::FigureIndex;
use crate::core::figure::resources::figure_registry::FigureRegistry;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::monster::resources::monster_registry::MonsterRegistry;
use crate::frontend::display::monster::types::monster_figure_entry::MonsterFigureEntry;

/// The monster figure binding file loaded at startup.
pub const MONSTER_FIGURES_PATH: &str = "data/graphic/monster_figures.ron";

/// Registry of all monster figure bindings, built once at startup from
/// the binding file. Every monster is guaranteed a binding (checked at
/// load), so a missing key at runtime is a load-time bug, not a runtime
/// case.
#[derive(Resource)]
pub struct MonsterFigureRegistry {
    figures: HashMap<MonsterIndex, FigureIndex>,
}

impl MonsterFigureRegistry {
    pub(crate) fn new(figures: HashMap<MonsterIndex, FigureIndex>) -> Self {
        MonsterFigureRegistry { figures }
    }

    /// The figure a monster presents.
    pub fn figure(&self, monster_index: MonsterIndex) -> FigureIndex {
        *self
            .figures
            .get(&monster_index)
            .expect("every declared monster has a figure binding (checked at load)")
    }
}

impl FromWorld for MonsterFigureRegistry {
    /// Build from the binding file, resolving ids against the monster and
    /// figure vocabularies — pulling both into existence if they are not
    /// built yet, so registration order never matters.
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(MONSTER_FIGURES_PATH).unwrap_or_else(|e| {
            panic!("cannot read monster figure bindings '{MONSTER_FIGURES_PATH}': {e}")
        });
        world.get_resource_or_init::<MonsterRegistry>();
        world.get_resource_or_init::<FigureRegistry>();
        // Both dependencies exist now (a pull builds them if missing);
        // shared reads suffice.
        let monster_registry = world.resource::<MonsterRegistry>();
        let figure_registry = world.resource::<FigureRegistry>();
        build_monster_figure_registry(
            MONSTER_FIGURES_PATH,
            &text,
            monster_registry,
            figure_registry,
        )
    }
}

/// Resolve and validate binding text against both vocabularies. Split
/// from file IO (`FromWorld`) so tests can exercise it with inline
/// documents.
pub(crate) fn build_monster_figure_registry(
    path: &str,
    text: &str,
    monster_registry: &MonsterRegistry,
    figure_registry: &FigureRegistry,
) -> MonsterFigureRegistry {
    let entries: Vec<MonsterFigureEntry> = ron::from_str(text)
        .unwrap_or_else(|e| panic!("monster figure bindings '{path}' is not valid RON: {e}"));
    let mut figures = HashMap::with_capacity(entries.len());
    for (i, entry) in entries.iter().enumerate() {
        let Some(monster_index) = monster_registry.get_index(&entry.monster) else {
            panic!(
                "monster figure bindings '{path}': unknown monster '{}'",
                entry.monster
            );
        };
        let Some(figure_index) = figure_registry.get_index(&entry.figure) else {
            panic!(
                "monster figure bindings '{path}': unknown figure '{}'",
                entry.figure
            );
        };
        assert!(
            !entries[..i].iter().any(|e| e.monster == entry.monster),
            "monster figure bindings '{path}': duplicate binding for monster '{}'",
            entry.monster
        );
        figures.insert(monster_index, figure_index);
    }
    for (_, id) in monster_registry.iter() {
        assert!(
            entries.iter().any(|e| e.monster == id),
            "monster figure bindings '{path}': monster '{id}' has no figure binding"
        );
    }
    MonsterFigureRegistry::new(figures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::figure::resources::figure_registry::parse_figure_registry;
    use crate::core::monster::resources::monster_registry::parse_monster_registry;

    fn figure_registry() -> FigureRegistry {
        parse_figure_registry("test", r#"[ "warrior", "giant_white_rat" ]"#)
    }

    fn monster_registry() -> MonsterRegistry {
        parse_monster_registry("test", r#"[ ( monster: "giant_white_rat" ) ]"#)
    }

    const DOC: &str = r#"[ ( monster: "giant_white_rat", figure: "giant_white_rat" ) ]"#;

    #[test]
    fn bindings_resolve_to_handles() {
        let figure_registry = figure_registry();
        let registry =
            build_monster_figure_registry("test", DOC, &monster_registry(), &figure_registry);
        let monster_index = monster_registry().get_index("giant_white_rat").unwrap();
        assert_eq!(
            registry.figure(monster_index),
            figure_registry.get_index("giant_white_rat").unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "unknown monster 'wolf'")]
    fn unknown_monster_panics() {
        let doc = r#"[ ( monster: "wolf", figure: "giant_white_rat" ) ]"#;
        build_monster_figure_registry("test", doc, &monster_registry(), &figure_registry());
    }

    #[test]
    #[should_panic(expected = "unknown figure 'wolf'")]
    fn unknown_figure_panics() {
        let doc = r#"[ ( monster: "giant_white_rat", figure: "wolf" ) ]"#;
        build_monster_figure_registry("test", doc, &monster_registry(), &figure_registry());
    }

    #[test]
    #[should_panic(expected = "duplicate binding for monster 'giant_white_rat'")]
    fn duplicate_binding_panics() {
        let doc = r#"[
            ( monster: "giant_white_rat", figure: "giant_white_rat" ),
            ( monster: "giant_white_rat", figure: "warrior" ),
        ]"#;
        build_monster_figure_registry("test", doc, &monster_registry(), &figure_registry());
    }

    #[test]
    #[should_panic(expected = "monster 'giant_white_rat' has no figure binding")]
    fn missing_binding_panics() {
        build_monster_figure_registry("test", "[]", &monster_registry(), &figure_registry());
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        build_monster_figure_registry(
            "test",
            "this is not ron",
            &monster_registry(),
            &figure_registry(),
        );
    }

    /// Spec-alignment test: the real binding file on disk binds the giant
    /// white rat to the figure of the same id.
    #[test]
    fn giant_white_rat_binding_matches_the_reference_presentation() {
        let figure_text = std::fs::read_to_string(
            crate::core::figure::resources::figure_registry::FIGURE_TABLE_PATH,
        )
        .unwrap();
        let figure_registry = parse_figure_registry("figures", &figure_text);
        let monster_text = std::fs::read_to_string(
            crate::core::monster::resources::monster_registry::MONSTER_TABLE_PATH,
        )
        .unwrap();
        let monster_registry = parse_monster_registry("monsters", &monster_text);

        let text = std::fs::read_to_string(MONSTER_FIGURES_PATH).unwrap();
        let registry = build_monster_figure_registry(
            MONSTER_FIGURES_PATH,
            &text,
            &monster_registry,
            &figure_registry,
        );
        let monster_index = monster_registry.get_index("giant_white_rat").unwrap();
        assert_eq!(
            registry.figure(monster_index),
            figure_registry.get_index("giant_white_rat").unwrap()
        );
    }
}
