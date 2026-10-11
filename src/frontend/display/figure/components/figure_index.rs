use bevy::prelude::*;

use crate::core::common::by_value::ByValue;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FigureIndex(usize);

impl ByValue for FigureIndex {}

impl FigureIndex {
    pub(crate) fn from_index(index: usize) -> Self {
        FigureIndex(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}
