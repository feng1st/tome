//! The stat bonus table bias: the zero point shared by every stat
//! bonus table.

/// The zero point of every stat bonus table: entries carry this bias,
/// and a reader subtracts it to obtain the bonus itself.
pub const STAT_BONUS_TABLE_BIAS: i32 = 128;
