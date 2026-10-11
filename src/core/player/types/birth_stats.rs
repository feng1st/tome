use crate::core::stats::types::stat_array::StatArray;

#[derive(Debug)]
pub struct BirthStats {
    pub max_stats: StatArray<i32>,
    pub curr_stats: StatArray<i32>,
    #[allow(dead_code)]
    pub use_stats: StatArray<i32>,
}
