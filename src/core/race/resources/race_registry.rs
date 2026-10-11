use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::race::components::race_index::RaceIndex;
use crate::core::race::types::race_entry::RaceEntry;
use crate::core::race::types::race_kind::RaceKind;
use crate::core::stats::constants::stat::Stat;
use crate::core::stats::types::stat_array::StatArray;

pub const RACE_TABLE_PATH: &str = "data/core/races.ron";

#[derive(Resource)]
pub struct RaceRegistry {
    race_kinds: Vec<RaceKind>,
    by_id: HashMap<String, RaceIndex>,
}

impl RaceRegistry {
    pub fn get_index(&self, id: &str) -> Option<RaceIndex> {
        self.by_id.get(id).copied()
    }

    pub fn race_kind(&self, race_index: RaceIndex) -> &RaceKind {
        &self.race_kinds[race_index.index()]
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (RaceIndex, &str)> {
        self.by_id.iter().map(|(id, index)| (*index, id.as_str()))
    }
}

impl FromWorld for RaceRegistry {
    fn from_world(_world: &mut World) -> Self {
        let race_table_content = fs::read_to_string(RACE_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read race table '{RACE_TABLE_PATH}': {e}"));
        parse_race_table(RACE_TABLE_PATH, &race_table_content)
    }
}

pub(crate) fn parse_race_table(path: &str, race_table_content: &str) -> RaceRegistry {
    let entries: Vec<RaceEntry> = ron::from_str(race_table_content)
        .unwrap_or_else(|e| panic!("race table '{path}' is not valid RON: {e}"));
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
        let stat_modifiers = StatArray::try_from(entry.stat_modifiers).unwrap_or_else(|values| {
            panic!(
                "race table '{path}': race '{}' has {} stat modifiers (expected {})",
                entry.race,
                values.len(),
                Stat::COUNT
            )
        });
        race_kinds.push(RaceKind {
            stat_modifiers,
            hit_dice_modifier: entry.hit_dice_modifier,
        });
    }
    RaceRegistry { race_kinds, by_id }
}
