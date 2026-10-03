//! Action cost: the price of an action kind at a creature's speed, in
//! ticks. Computed at action-creation time and written into
//! `Acting.until` — fixed at birth, never re-evaluated.

use crate::core::time::constants::action_point_rate::STANDARD_ACTION_COST;
use crate::core::time::constants::action_point_rate::STANDARD_RATE;
use crate::core::time::utils::action_point_rate::action_point_rate;

/// Ticks an action of `base` ticks (at standard speed) costs a creature
/// with `speed`. Cost varies the price, never the pacing: a creature
/// twice as fast pays half per action. Integer rounding is round-half-up;
/// the rate table's cap (49) keeps every cost >= 20 ticks, so no action
/// is ever free.
pub fn action_cost(base: u32, speed: usize) -> i64 {
    let rate = action_point_rate(speed);
    ((base * STANDARD_RATE + rate / 2) / rate) as i64
}

/// Ticks of the standard action (one step) at `speed`.
pub fn standard_action_cost(speed: usize) -> i64 {
    action_cost(STANDARD_ACTION_COST, speed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::creature::constants::standard_speed::STANDARD_SPEED;

    #[test]
    fn landmark_costs() {
        assert_eq!(standard_action_cost(STANDARD_SPEED), 100);
        assert_eq!(standard_action_cost(125), 40);
        assert_eq!(standard_action_cost(100), 200);
    }

    #[test]
    fn rounds_half_up() {
        // rate(115) = 15: 1000/15 = 66.67 -> 67; rate(137) = 36: 27.78 -> 28.
        assert_eq!(standard_action_cost(115), 67);
        assert_eq!(standard_action_cost(137), 28);
    }

    #[test]
    fn no_action_is_ever_free() {
        for speed in 0..300 {
            assert!(standard_action_cost(speed) >= 20, "speed {speed}");
        }
    }
}
