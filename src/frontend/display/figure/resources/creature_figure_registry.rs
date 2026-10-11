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

pub const CREATURE_FIGURE_TABLE_PATH: &str = "data/graphic/creature_figures.ron";

#[derive(Resource)]
pub struct CreatureFigureRegistry {
    by_monster: HashMap<MonsterIndex, FigureIndex>,
    by_race_and_class: HashMap<(RaceIndex, ClassIndex), FigureIndex>,
    by_race: HashMap<RaceIndex, FigureIndex>,
}

impl CreatureFigureRegistry {
    pub fn get_monster_figure(&self, monster_index: MonsterIndex) -> Option<FigureIndex> {
        self.by_monster.get(&monster_index).copied()
    }

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
    fn from_world(world: &mut World) -> Self {
        let creature_figure_table_content = fs::read_to_string(CREATURE_FIGURE_TABLE_PATH)
            .unwrap_or_else(|e| {
                panic!("cannot read creature figure bindings '{CREATURE_FIGURE_TABLE_PATH}': {e}")
            });
        world.get_resource_or_init::<RaceRegistry>();
        world.get_resource_or_init::<ClassRegistry>();
        world.get_resource_or_init::<MonsterRegistry>();
        world.get_resource_or_init::<FigureRegistry>();
        parse_creature_figure_table(
            CREATURE_FIGURE_TABLE_PATH,
            &creature_figure_table_content,
            world.resource::<RaceRegistry>(),
            world.resource::<ClassRegistry>(),
            world.resource::<MonsterRegistry>(),
            world.resource::<FigureRegistry>(),
        )
    }
}

fn parse_creature_figure_table(
    path: &str,
    creature_figure_table_content: &str,
    race_registry: &RaceRegistry,
    class_registry: &ClassRegistry,
    monster_registry: &MonsterRegistry,
    figure_registry: &FigureRegistry,
) -> CreatureFigureRegistry {
    let entries: Vec<CreatureFigureEntry> = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(creature_figure_table_content)
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
