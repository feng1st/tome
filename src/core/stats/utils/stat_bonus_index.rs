//! The compressed statistic index: the shared reading convention of
//! every stat bonus table.

/// The bracket a statistic value occupies in the 38-bracket stat bonus
/// tables: 3–18 map one to one (anything below 3 shares the first
/// bracket), 18/x maps one bracket per ten points of x, and 18/220 and
/// beyond share the last bracket.
pub fn stat_bonus_index(value: i32) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scale_floor_shares_the_first_bracket() {
        assert_eq!(stat_bonus_index(3), 0);
        assert_eq!(stat_bonus_index(0), 0);
        assert_eq!(stat_bonus_index(-10), 0);
    }

    #[test]
    fn one_bracket_per_point_up_to_18() {
        assert_eq!(stat_bonus_index(4), 1);
        assert_eq!(stat_bonus_index(17), 14);
        assert_eq!(stat_bonus_index(18), 15, "the sixteenth bracket");
    }

    #[test]
    fn one_bracket_per_ten_points_of_x_above_18() {
        // Above 18 the value carries x directly: 28 is 18/10, 118 is
        // 18/100, and consecutive tens land on their own brackets.
        assert_eq!(stat_bonus_index(28), 16, "18/10");
        assert_eq!(stat_bonus_index(38), 17, "18/20");
        assert_eq!(stat_bonus_index(118), 25, "18/100");
        assert_eq!(stat_bonus_index(237), 36, "18/219");
    }

    #[test]
    fn the_last_bracket_is_shared() {
        assert_eq!(stat_bonus_index(238), 37, "18/220");
        assert_eq!(stat_bonus_index(999), 37, "18/220+ saturates");
    }
}
