use crate::core::stats::constants::stat::Stat;

#[derive(Clone, Copy, Debug, Default)]
pub struct StatArray<T>([T; Stat::COUNT]);

impl<T> StatArray<T> {
    pub const fn new(values: [T; Stat::COUNT]) -> Self {
        Self(values)
    }
}

impl TryFrom<Vec<i32>> for StatArray<i32> {
    type Error = Vec<i32>;

    fn try_from(values: Vec<i32>) -> Result<Self, Self::Error> {
        Ok(Self::new(values.try_into()?))
    }
}

impl<T> std::ops::Index<Stat> for StatArray<T> {
    type Output = T;

    fn index(&self, stat: Stat) -> &T {
        &self.0[stat.index()]
    }
}

impl<T> std::ops::IndexMut<Stat> for StatArray<T> {
    fn index_mut(&mut self, stat: Stat) -> &mut T {
        &mut self.0[stat.index()]
    }
}
