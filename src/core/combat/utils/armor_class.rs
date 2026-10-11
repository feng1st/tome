use crate::core::combat::utils::combat_bonus::armor_bonus;
use crate::core::stats::components::curr_stats::CurrStats;

const EQUIPMENT_ARMOR_BASE: i32 = 0;

/// ToME2, xtra1.c, calc_bonuses, p_ptr->ac, p_ptr->to_a
pub fn armor_class(curr_stats: &CurrStats) -> i32 {
    EQUIPMENT_ARMOR_BASE + armor_bonus(curr_stats)
}
