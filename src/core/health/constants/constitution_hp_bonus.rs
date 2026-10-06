//! The constitution hit-point bonus table: the shared lookup data of
//! the hit-point derivation.

/// The hit-point bonus per constitution bracket, indexed by the
/// compressed statistic index: one entry per point from 3 to 17, one
/// per ten-point bracket of 18/x up to 18/219, and one for 18/220 and
/// beyond. Entries carry the 128 bias (the stats domain's
/// `STAT_BONUS_TABLE_BIAS`); a reader subtracts the bias to obtain the
/// bonus.
pub(crate) const CONSTITUTION_HP_BONUS: [i32; 38] = [
    123, 125, 126, 127, 128, 128, 128, 128, 128, 128, 128, 128, 129, 129, 130, 131, 132, 132, 132,
    132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 146, 148, 150, 153, 154, 155,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::stats::constants::stat_bonus_table::STAT_BONUS_TABLE_BIAS;

    #[test]
    fn table_covers_every_bracket() {
        assert_eq!(CONSTITUTION_HP_BONUS.len(), 38);
        // Entries carry the bias: the deepest floor and the ceiling
        // match the bonus scale once the bias is subtracted.
        assert_eq!(CONSTITUTION_HP_BONUS[0] - STAT_BONUS_TABLE_BIAS, -5);
        assert_eq!(
            CONSTITUTION_HP_BONUS[37] - STAT_BONUS_TABLE_BIAS,
            27,
            "18/220+"
        );
    }
}
