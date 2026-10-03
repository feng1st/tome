//! The action point rate lookup: speed index to points per tick.

use crate::core::time::constants::action_point_rate::ACTION_POINT_RATES;

/// Action points gained per tick at `speed`. Infallible by construction:
/// vocabulary files reject out-of-range speeds at load time.
pub fn action_point_rate(speed: usize) -> u32 {
    ACTION_POINT_RATES[speed] as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::creature::constants::standard_speed::STANDARD_SPEED;

    #[test]
    fn landmark_rates() {
        assert_eq!(action_point_rate(100), 5);
        assert_eq!(action_point_rate(STANDARD_SPEED), 10);
        assert_eq!(action_point_rate(120), 20);
        assert_eq!(action_point_rate(137), 36);
        assert_eq!(action_point_rate(150), 42);
    }

    #[test]
    fn saturates_at_high_speed() {
        assert_eq!(action_point_rate(180), 49);
        assert_eq!(action_point_rate(299), 49);
    }

    #[test]
    fn nondecreasing() {
        for speed in 0..(ACTION_POINT_RATES.len() - 1) {
            assert!(action_point_rate(speed) <= action_point_rate(speed + 1));
        }
    }
}
