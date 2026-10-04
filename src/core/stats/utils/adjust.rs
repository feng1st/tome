//! Modifier merging: how a race-and-class modifier reshapes one base
//! statistic.

use rand::Rng;

/// Merge one statistic's modifier into its base value. Below 18 every
/// point of modifier is one point of statistic; from 18 up the merge
/// moves in larger steps (random draws on the default birth path), and
/// a negative modifier drains to 18 faster than it drops below it. The
/// floor is 3: no statistic drains below the scale's bottom.
pub fn adjust_stat(value: i32, modifier: i32) -> i32 {
    let mut value = value;
    if modifier < 0 {
        for _ in 0..(-modifier) {
            if value >= 28 {
                value -= 10;
            } else if value > 18 {
                value = 18;
            } else if value > 3 {
                value -= 1;
            }
        }
    } else if modifier > 0 {
        let mut rng = rand::rng();
        for _ in 0..modifier {
            if value < 18 {
                value += 1;
            } else if value < 88 {
                value += rng.random_range(1..=15) + 5;
            } else if value < 108 {
                value += rng.random_range(1..=6) + 2;
            } else if value < 118 {
                value += 1;
            }
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_modifier_keeps_the_base() {
        assert_eq!(adjust_stat(13, 0), 13);
        assert_eq!(adjust_stat(17, 0), 17);
    }

    #[test]
    fn positive_modifier_never_below_base_plus_modifier() {
        // Every merged point adds at least one, whatever bracket it
        // lands in.
        for base in [3, 10, 17] {
            for modifier in 1..=6 {
                assert!(adjust_stat(base, modifier) >= base + modifier);
            }
        }
    }

    #[test]
    fn crossing_eighteen_moves_in_larger_steps() {
        // The second and later points above 18 each add at least six.
        let merged = adjust_stat(17, 2);
        assert!(merged >= 24, "17 +2 merged to {merged}, below 24");
        let merged = adjust_stat(17, 5);
        assert!(merged >= 42, "17 +5 merged to {merged}, below 42");
    }

    #[test]
    fn negative_modifier_never_exceeds_the_base() {
        for base in [5, 12, 20, 40] {
            for modifier in 1..=5 {
                assert!(adjust_stat(base, -modifier) <= base);
            }
        }
    }

    #[test]
    fn negative_drains_to_the_floor() {
        assert_eq!(adjust_stat(5, -10), 3);
        assert_eq!(
            adjust_stat(20, -10),
            9,
            "20 snaps to 18 on the first point, then drains by one"
        );
    }
}
