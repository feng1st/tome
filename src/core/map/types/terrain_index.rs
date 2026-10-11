use crate::core::common::by_value::ByValue;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TerrainIndex(usize);

impl ByValue for TerrainIndex {}

impl TerrainIndex {
    pub(crate) fn from_index(index: usize) -> Self {
        TerrainIndex(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}
