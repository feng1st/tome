use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::class::components::class_index::ClassIndex;
use crate::core::class::types::class_entry::ClassEntry;
use crate::core::class::types::class_kind::ClassKind;
use crate::core::stats::constants::stat::Stat;
use crate::core::stats::types::stat_array::StatArray;

pub const CLASS_TABLE_PATH: &str = "data/core/classes.ron";

#[derive(Resource)]
pub struct ClassRegistry {
    class_kinds: Vec<ClassKind>,
    by_id: HashMap<String, ClassIndex>,
}

impl ClassRegistry {
    pub fn get_index(&self, id: &str) -> Option<ClassIndex> {
        self.by_id.get(id).copied()
    }

    pub fn class_kind(&self, class_index: ClassIndex) -> &ClassKind {
        &self.class_kinds[class_index.index()]
    }
}

impl FromWorld for ClassRegistry {
    fn from_world(_world: &mut World) -> Self {
        let class_table_content = fs::read_to_string(CLASS_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read class table '{CLASS_TABLE_PATH}': {e}"));
        parse_class_table(CLASS_TABLE_PATH, &class_table_content)
    }
}

pub(crate) fn parse_class_table(path: &str, class_table_content: &str) -> ClassRegistry {
    let entries: Vec<ClassEntry> = ron::from_str(class_table_content)
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
        let stat_modifiers = StatArray::try_from(entry.stat_modifiers).unwrap_or_else(|values| {
            panic!(
                "class table '{path}': class '{}' has {} stat modifiers (expected {})",
                entry.class,
                values.len(),
                Stat::COUNT
            )
        });
        class_kinds.push(ClassKind {
            stat_modifiers,
            hit_dice_modifier: entry.hit_dice_modifier,
        });
    }
    ClassRegistry { class_kinds, by_id }
}
