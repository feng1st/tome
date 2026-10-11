use rand::Rng;

use crate::core::combat::utils::combat_bonus::{damage_bonus, hit_bonus};
use crate::core::stats::components::curr_stats::CurrStats;

const MELEE_STYLE_SKILL_STANDIN: i32 = 1;

const COMBAT_SKILL_STANDIN: i32 = 2;

const ATTACK_QUALITY_STANDIN: i32 = 1;

/// ToME2, xtra1.c, calc_bonuses, p_ptr->skill_thn
pub fn melee_skill_to_hit(melee_style_skill: i32, combat_skill: i32) -> i32 {
    50 * ((7 * melee_style_skill + 3 * combat_skill) / 10) / 10
}

const CHANCE_PER_HIT_BONUS: i32 = 3;

/// ToME2, cmd1.c, py_attack, chance
pub fn attack_chance(curr_stats: &CurrStats) -> i32 {
    melee_skill_to_hit(MELEE_STYLE_SKILL_STANDIN, COMBAT_SKILL_STANDIN)
        + (hit_bonus(curr_stats) + ATTACK_QUALITY_STANDIN) * CHANCE_PER_HIT_BONUS
}

const UNARMED_DAMAGE_BASE: i32 = 1;

/// ToME2, cmd1.c, py_attack, bare hands do one damage
pub fn unarmed_damage(curr_stats: &CurrStats) -> i32 {
    (UNARMED_DAMAGE_BASE + damage_bonus(curr_stats)).max(0)
}

/// ToME2, cmd1.c, test_hit_norm; melee1.c, check_hit
pub fn test_hit_melee(chance: i32, armor_class: i32, rng: &mut impl Rng) -> bool {
    let percentile: i32 = rng.random_range(0..100);
    if percentile < 10 {
        return percentile < 5;
    }
    if chance <= 0 {
        return false;
    }
    rng.random_range(0..chance) >= armor_class * 3 / 4
}
