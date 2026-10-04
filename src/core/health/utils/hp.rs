//! Hit-point derivation: a level-one character's hit-point ceiling.

use crate::core::health::constants::constitution_hp_bonus::CONSTITUTION_HP_BONUS;

/// The hit-point bonus a constitution value grants.
fn constitution_hp_bonus(constitution: i32) -> i32 {
    CONSTITUTION_HP_BONUS[stat_bonus_index(constitution)]
}

/// The compressed statistic index a value occupies, always inside the
/// bonus table: 3–18 map one to one (anything below 3 shares the first
/// bracket), 18/x map one bracket per ten points of x, and 18/220 and
/// beyond share the last bracket.
fn stat_bonus_index(value: i32) -> usize {
    if value <= 3 {
        0
    } else if value <= 18 {
        (value - 3) as usize
    } else if value <= 18 + 219 {
        (15 + (value - 18) / 10) as usize
    } else {
        37
    }
}

/// A level-one character's hit-point ceiling: the full hit die (race
/// and class shares summed) plus half the constitution bonus, integer
/// division.
pub fn max_hit_points(hit_die: i32, constitution: i32) -> i32 {
    hit_die + constitution_hp_bonus(constitution) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn landmark_values() {
        // One landmark per bracket shape: low floor, the 18 threshold,
        // an early 18/x bracket, deep into the brackets, and past the
        // last. Above 18 the value carries x directly: 28 is 18/10,
        // 118 is 18/100, 238 is 18/220.
        assert_eq!(constitution_hp_bonus(3), -5);
        assert_eq!(constitution_hp_bonus(17), 2);
        assert_eq!(constitution_hp_bonus(18), 3, "18/00");
        assert_eq!(constitution_hp_bonus(28), 4, "18/10");
        assert_eq!(constitution_hp_bonus(118), 10, "18/100");
        assert_eq!(constitution_hp_bonus(238), 27, "18/220");
        assert_eq!(constitution_hp_bonus(999), 27, "18/220+ saturates");
    }

    #[test]
    fn brackets_step_per_ten_points_of_x() {
        // One bracket per ten points of x: consecutive tens land on
        // their own entries.
        assert_eq!(constitution_hp_bonus(28), 4, "18/10");
        assert_eq!(constitution_hp_bonus(38), 4, "18/20");
        assert_eq!(constitution_hp_bonus(48), 4, "18/30");
        assert_eq!(constitution_hp_bonus(118), 10, "18/100");
    }

    #[test]
    fn below_the_scale_floor_clamps_to_the_first_bracket() {
        // The stat scale floors at 3; lower inputs (which it cannot
        // produce) still land on a valid table entry — the first bracket.
        assert_eq!(constitution_hp_bonus(2), -5);
        assert_eq!(constitution_hp_bonus(0), -5);
        assert_eq!(constitution_hp_bonus(-10), -5);
    }

    #[test]
    fn ceiling_is_die_plus_half_the_bonus() {
        // Human 10 + warrior 9 = 19 hit die.
        assert_eq!(max_hit_points(19, 14), 19, "bonus +1 halves to 0");
        assert_eq!(max_hit_points(19, 17), 20, "bonus +2 halves to 1");
        assert_eq!(max_hit_points(19, 8), 19, "no bonus");
        assert_eq!(max_hit_points(19, 3), 17, "bonus -5 halves to -2");
    }
}
