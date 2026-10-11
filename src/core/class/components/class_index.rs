use bevy::prelude::*;

use crate::core::common::by_value::ByValue;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ClassIndex(usize);

impl ByValue for ClassIndex {}

impl ClassIndex {
    pub(crate) fn from_index(index: usize) -> Self {
        ClassIndex(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}
