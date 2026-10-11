use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::dice::types::dice::Dice;
use crate::core::monster::components::monster_index::MonsterIndex;
use crate::core::monster::types::monster_blow::MonsterBlow;
use crate::core::monster::types::monster_entry::MonsterEntry;
use crate::core::monster::types::monster_kind::MonsterKind;
use crate::core::speed::components::speed::Speed;
use crate::core::speed::constants::speed::SPEED_RATE_TABLE;

pub const MONSTER_TABLE_PATH: &str = "data/core/monsters.ron";

#[derive(Resource)]
pub struct MonsterRegistry {
    monster_kinds: Vec<MonsterKind>,
    by_id: HashMap<String, MonsterIndex>,
}

impl MonsterRegistry {
    pub fn get_index(&self, id: &str) -> Option<MonsterIndex> {
        self.by_id.get(id).copied()
    }

    pub fn monster_kind(&self, monster_index: MonsterIndex) -> &MonsterKind {
        &self.monster_kinds[monster_index.index()]
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (MonsterIndex, &str)> {
        self.by_id.iter().map(|(id, index)| (*index, id.as_str()))
    }
}

impl FromWorld for MonsterRegistry {
    fn from_world(_world: &mut World) -> Self {
        let monster_table_content = fs::read_to_string(MONSTER_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read monster table '{MONSTER_TABLE_PATH}': {e}"));
        parse_monster_table(MONSTER_TABLE_PATH, &monster_table_content)
    }
}

pub(crate) fn parse_monster_table(path: &str, monster_table_content: &str) -> MonsterRegistry {
    let entries: Vec<MonsterEntry> = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(monster_table_content)
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
            entry.speed >= 0 && entry.speed < SPEED_RATE_TABLE.len() as i32,
            "monster table '{path}': monster '{}' has speed {} outside the rate table",
            entry.monster,
            entry.speed
        );
        let hit_points = entry.hit_points.parse::<Dice>().unwrap_or_else(|e| {
            panic!(
                "monster table '{path}': monster '{}' has an invalid hit_points field: {e}",
                entry.monster
            )
        });
        let blows = entry
            .blows
            .into_iter()
            .map(|blow| {
                let damage = blow.damage.parse::<Dice>().unwrap_or_else(|e| {
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
