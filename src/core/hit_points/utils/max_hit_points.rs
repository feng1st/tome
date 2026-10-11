use crate::core::stats::components::curr_stats::CurrStats;
use crate::core::stats::constants::stat::Stat;
use crate::core::stats::constants::stat_adjustment_tables::CONSTITUTION_TO_HALF_HP_PER_LEVEL;
use crate::core::stats::utils::stat_index::stat_index;

// ToME2, xtra1.c, calc_hitpoints
pub fn max_hit_points(hit_dice_modifier: i32, curr_stats: &CurrStats) -> i32 {
    let constitution_index = stat_index(curr_stats.0[Stat::Constitution]);
    (hit_dice_modifier + CONSTITUTION_TO_HALF_HP_PER_LEVEL[constitution_index] / 2).max(1)
}
