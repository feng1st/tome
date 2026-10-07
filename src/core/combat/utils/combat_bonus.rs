//! Combat bonus derivation: the table reads that turn the current
//! statistics into the three melee bonuses.

use crate::core::combat::constants::stat_bonus_tables::{
    DEXTERITY_ARMOR_BONUS, DEXTERITY_HIT_BONUS, STRENGTH_DAMAGE_BONUS, STRENGTH_HIT_BONUS,
};
use crate::core::stats::components::stats::Stats;
use crate::core::stats::constants::stat::Stat;
use crate::core::stats::constants::stat_bonus_table::STAT_BONUS_TABLE_BIAS;
use crate::core::stats::utils::stat_bonus_index::stat_bonus_index;

/// The bonus a table entry grants for a statistic value: the entry at
/// the value's bracket minus the table bias.
fn table_bonus(table: &[i32; 38], value: i32) -> i32 {
    table[stat_bonus_index(value)] - STAT_BONUS_TABLE_BIAS
}

/// The melee hit bonus: the strength-to-hit and dexterity-to-hit
/// entries summed, read by the current values.
pub fn hit_bonus(stats: &Stats) -> i32 {
    table_bonus(&STRENGTH_HIT_BONUS, stats.current[Stat::Strength.index()])
        + table_bonus(&DEXTERITY_HIT_BONUS, stats.current[Stat::Dexterity.index()])
}

/// The damage bonus: the strength-to-damage entry, read by the
/// current strength.
pub fn damage_bonus(stats: &Stats) -> i32 {
    table_bonus(
        &STRENGTH_DAMAGE_BONUS,
        stats.current[Stat::Strength.index()],
    )
}

/// The armor bonus: the dexterity-to-armor entry, read by the current
/// dexterity.
pub fn armor_bonus(stats: &Stats) -> i32 {
    table_bonus(
        &DEXTERITY_ARMOR_BONUS,
        stats.current[Stat::Dexterity.index()],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stats(strength: i32, dexterity: i32) -> Stats {
        Stats {
            max: [strength, 10, 10, dexterity, 10, 10],
            current: [strength, 10, 10, dexterity, 10, 10],
        }
    }

    #[test]
    fn hand_computed_bonuses() {
        // Strength 18/10 (28) and dexterity 18/10 (28): hit 1 + 3,
        // damage 2, armor 2.
        let stats = stats(28, 28);
        assert_eq!(hit_bonus(&stats), 4);
        assert_eq!(damage_bonus(&stats), 2);
        assert_eq!(armor_bonus(&stats), 2);
    }

    #[test]
    fn low_statistics_yield_negative_bonuses() {
        // Strength 3 and dexterity 3: hit -3 + -3, damage -2,
        // armor -4.
        let stats = stats(3, 3);
        assert_eq!(hit_bonus(&stats), -6);
        assert_eq!(damage_bonus(&stats), -2);
        assert_eq!(armor_bonus(&stats), -4);
    }

    #[test]
    fn midrange_statistics_yield_no_bonus() {
        let stats = stats(10, 14);
        assert_eq!(hit_bonus(&stats), 0);
        assert_eq!(damage_bonus(&stats), 0);
        assert_eq!(armor_bonus(&stats), 0);
    }

    #[test]
    fn reads_take_the_current_values() {
        // The maximum may sit above the current set: reads follow the
        // current values.
        let mut stats = stats(16, 16);
        stats.current[Stat::Strength.index()] = 3;
        stats.current[Stat::Dexterity.index()] = 3;
        assert_eq!(hit_bonus(&stats), -6);
    }
}
