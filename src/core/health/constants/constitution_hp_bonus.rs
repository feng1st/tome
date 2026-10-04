//! The constitution hit-point bonus table: the shared lookup data of
//! the hit-point derivation.

/// The hit-point bonus per constitution bracket, indexed by the
/// compressed statistic index: one entry per point from 3 to 17, one
/// per ten-point bracket of 18/x up to 18/219, and one for 18/220 and
/// beyond. The 128 baseline of the source table is already subtracted —
/// these are the bonuses themselves.
pub(crate) const CONSTITUTION_HP_BONUS: [i32; 38] = [
    -5, -3, -2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    14, 15, 16, 18, 20, 22, 25, 26, 27,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_covers_every_bracket() {
        assert_eq!(CONSTITUTION_HP_BONUS.len(), 38);
    }
}
