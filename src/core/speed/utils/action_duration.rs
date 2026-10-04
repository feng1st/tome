//! Action duration: how many ticks an action kind takes a creature of
//! a given speed. Computed at action-creation time and written into the
//! actor's next turn; fixed at birth, never re-evaluated.

use crate::core::speed::constants::speed::SPEED_RATE_TABLE;
use crate::core::speed::constants::speed::STANDARD_SPEED_RATE;

/// Ticks an action of `base` ticks (at standard speed) takes a creature
/// of `speed`. Duration varies the price, never the pacing: a creature
/// twice as fast pays half per action. Integer rounding is round-half-
/// up; the rate table's cap (49) keeps every duration >= 20 ticks, so
/// no action is ever free. Infallible by construction: vocabulary files
/// reject out-of-range speeds at load time.
pub fn action_duration(base: u32, speed: usize) -> i64 {
    let speed_rate = SPEED_RATE_TABLE[speed] as u32;
    ((base * STANDARD_SPEED_RATE + speed_rate / 2) / speed_rate) as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::speed::constants::action_duration::STANDARD_ACTION_DURATION;

    #[test]
    fn landmark_durations() {
        assert_eq!(action_duration(STANDARD_ACTION_DURATION, 110), 100);
        assert_eq!(action_duration(STANDARD_ACTION_DURATION, 125), 40);
        assert_eq!(action_duration(STANDARD_ACTION_DURATION, 100), 200);
    }

    #[test]
    fn rounds_half_up() {
        // rate 15: 1000/15 = 66.67 -> 67; rate 36: 1000/36 = 27.78 -> 28.
        assert_eq!(action_duration(STANDARD_ACTION_DURATION, 115), 67);
        assert_eq!(action_duration(STANDARD_ACTION_DURATION, 137), 28);
    }

    #[test]
    fn no_action_is_ever_free() {
        for speed in 0..300 {
            assert!(
                action_duration(STANDARD_ACTION_DURATION, speed) >= 20,
                "speed {speed}"
            );
        }
    }
}
