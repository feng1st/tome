use bevy::prelude::*;

use crate::core::common::by_value::ByValue;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RaceIndex(usize);

impl ByValue for RaceIndex {}

impl RaceIndex {
    pub(crate) fn from_index(index: usize) -> Self {
        RaceIndex(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}
