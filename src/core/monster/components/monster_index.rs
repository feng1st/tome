use bevy::prelude::*;

use crate::core::common::by_value::ByValue;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MonsterIndex(usize);

impl ByValue for MonsterIndex {}

impl MonsterIndex {
    pub(crate) fn from_index(index: usize) -> Self {
        MonsterIndex(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}
