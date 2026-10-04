//! The speed scale: vocabulary values and the rate table that
//! translates them into pacing.

/// The speed at which a creature acts exactly once per standard action
/// window: one step, one attack, one standard action duration.
/// Vocabulary files express every creature's speed relative to this
/// reference.
pub const STANDARD_SPEED: usize = 110;

/// The pace rate a raw speed value buys, indexed by speed. The curve is
/// flat at the low end, near-linear around standard, and saturates at 49
/// for high speeds. Vocabulary files declare raw speed values; a value
/// outside this table is rejected at load.
pub const SPEED_RATE_TABLE: [u8; 300] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4,
    4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 7, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
    23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 40,
    41, 41, 41, 42, 42, 42, 43, 43, 43, 44, 44, 44, 44, 45, 45, 45, 45, 45, 46, 46, 46, 46, 46, 47,
    47, 47, 47, 47, 48, 48, 48, 48, 48, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49,
    49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49,
    49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49,
    49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49,
    49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49,
    49, 49, 49, 49, 49, 49, 49, 49, 49,
];

/// The pace rate at the standard speed: the normalization factor of
/// action pricing. Read straight off the table — the table is the
/// single source of the pace scale.
pub const STANDARD_SPEED_RATE: u32 = SPEED_RATE_TABLE[STANDARD_SPEED] as u32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn landmark_rates() {
        assert_eq!(SPEED_RATE_TABLE[100], 5);
        assert_eq!(SPEED_RATE_TABLE[120], 20);
        assert_eq!(SPEED_RATE_TABLE[137], 36);
        assert_eq!(SPEED_RATE_TABLE[150], 42);
    }

    #[test]
    fn saturates_at_high_speed() {
        assert_eq!(SPEED_RATE_TABLE[180], 49);
        assert_eq!(SPEED_RATE_TABLE[299], 49);
    }

    #[test]
    fn nondecreasing() {
        for speed in 0..(SPEED_RATE_TABLE.len() - 1) {
            assert!(SPEED_RATE_TABLE[speed] <= SPEED_RATE_TABLE[speed + 1]);
        }
    }
}
