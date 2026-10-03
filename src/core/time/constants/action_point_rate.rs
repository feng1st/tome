//! The action point rate table: how many action points a creature gains per
//! tick at a given speed. This table is the single pacing authority for the
//! world clock; speed values in vocabulary files are table indices. Pure
//! constants — the lookup function lives in the domain's utils.

use crate::core::creature::constants::standard_speed::STANDARD_SPEED;

/// Action points gained per tick, indexed by speed. Standard speed is
/// [`STANDARD_SPEED`] (10 points per tick); the curve is flat at the low end,
/// near-linear around standard, and saturates at 49 for high speeds.
pub const ACTION_POINT_RATES: [u8; 300] = [
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

/// Ticks the standard action (one step, one attack) costs at standard
/// speed. Every action kind is priced relative to this base, and speed
/// scales the price — never the other way round.
pub const STANDARD_ACTION_COST: u32 = 100;

/// The rate at standard speed; tween durations scale by `STANDARD_RATE /
/// rate`, so a creature twice as fast crosses a cell in half the time.
pub const STANDARD_RATE: u32 = ACTION_POINT_RATES[STANDARD_SPEED] as u32;
