use crate::core::stats::components::curr_stats::CurrStats;
use crate::core::stats::constants::stat::Stat;
use crate::core::stats::constants::stat_adjustment_tables::{
    DEXTERITY_TO_ARMOR_CLASS, DEXTERITY_TO_HIT, STRENGTH_TO_DAMAGE, STRENGTH_TO_HIT,
};
use crate::core::stats::utils::stat_index::stat_index;

pub fn hit_bonus(curr_stats: &CurrStats) -> i32 {
    STRENGTH_TO_HIT[stat_index(curr_stats.0[Stat::Strength])]
        + DEXTERITY_TO_HIT[stat_index(curr_stats.0[Stat::Dexterity])]
}

pub fn damage_bonus(curr_stats: &CurrStats) -> i32 {
    STRENGTH_TO_DAMAGE[stat_index(curr_stats.0[Stat::Strength])]
}

pub fn armor_bonus(curr_stats: &CurrStats) -> i32 {
    DEXTERITY_TO_ARMOR_CLASS[stat_index(curr_stats.0[Stat::Dexterity])]
}
