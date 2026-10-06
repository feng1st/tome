//! The stat bonus tables: the shared lookup data of the combat bonus
//! derivation — one strength table and one dexterity table per
//! combat bonus.

/// The strength-to-hit bonus table, indexed by the compressed
/// statistic index. Entries carry the 128 bias (the stats domain's
/// `STAT_BONUS_TABLE_BIAS`); a reader subtracts the bias to obtain the
/// bonus.
pub(crate) const STRENGTH_HIT_BONUS: [i32; 38] = [
    125, 126, 127, 127, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 129, 129, 129, 129,
    129, 129, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 143, 144,
];

/// The strength-to-damage bonus table, indexed by the compressed
/// statistic index. Entries carry the 128 bias (the stats domain's
/// `STAT_BONUS_TABLE_BIAS`); a reader subtracts the bias to obtain the
/// bonus.
pub(crate) const STRENGTH_DAMAGE_BONUS: [i32; 38] = [
    126, 126, 127, 127, 128, 128, 128, 128, 128, 128, 128, 128, 128, 129, 130, 130, 130, 131, 131,
    131, 131, 131, 132, 133, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 146, 148,
];

/// The dexterity-to-hit bonus table, indexed by the compressed
/// statistic index. Entries carry the 128 bias (the stats domain's
/// `STAT_BONUS_TABLE_BIAS`); a reader subtracts the bias to obtain the
/// bonus.
pub(crate) const DEXTERITY_HIT_BONUS: [i32; 38] = [
    125, 126, 126, 127, 127, 128, 128, 128, 128, 128, 128, 128, 128, 129, 130, 131, 131, 131, 131,
    131, 132, 132, 132, 132, 133, 134, 135, 136, 137, 137, 138, 139, 140, 141, 142, 143, 143, 144,
];

/// The dexterity-to-armor bonus table, indexed by the compressed
/// statistic index. Entries carry the 128 bias (the stats domain's
/// `STAT_BONUS_TABLE_BIAS`); a reader subtracts the bias to obtain the
/// bonus.
pub(crate) const DEXTERITY_ARMOR_BONUS: [i32; 38] = [
    124, 125, 126, 127, 128, 128, 128, 128, 128, 128, 128, 128, 129, 129, 129, 130, 130, 130, 130,
    130, 131, 131, 131, 132, 133, 134, 135, 136, 137, 137, 138, 139, 140, 141, 142, 143, 143, 144,
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Every entry is pinned as a literal: an accidental edit cannot
    /// slip through.
    #[test]
    fn strength_hit_bonus_matches_the_pinned_values() {
        assert_eq!(
            STRENGTH_HIT_BONUS,
            [
                125, 126, 127, 127, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 129,
                129, 129, 129, 129, 129, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139,
                140, 141, 142, 143, 143, 144,
            ]
        );
    }

    /// Every entry is pinned as a literal: an accidental edit cannot
    /// slip through.
    #[test]
    fn strength_damage_bonus_matches_the_pinned_values() {
        assert_eq!(
            STRENGTH_DAMAGE_BONUS,
            [
                126, 126, 127, 127, 128, 128, 128, 128, 128, 128, 128, 128, 128, 129, 130, 130,
                130, 131, 131, 131, 131, 131, 132, 133, 133, 134, 135, 136, 137, 138, 139, 140,
                141, 142, 143, 144, 146, 148,
            ]
        );
    }

    /// Every entry is pinned as a literal: an accidental edit cannot
    /// slip through.
    #[test]
    fn dexterity_hit_bonus_matches_the_pinned_values() {
        assert_eq!(
            DEXTERITY_HIT_BONUS,
            [
                125, 126, 126, 127, 127, 128, 128, 128, 128, 128, 128, 128, 128, 129, 130, 131,
                131, 131, 131, 131, 132, 132, 132, 132, 133, 134, 135, 136, 137, 137, 138, 139,
                140, 141, 142, 143, 143, 144,
            ]
        );
    }

    /// Every entry is pinned as a literal: an accidental edit cannot
    /// slip through.
    #[test]
    fn dexterity_armor_bonus_matches_the_pinned_values() {
        assert_eq!(
            DEXTERITY_ARMOR_BONUS,
            [
                124, 125, 126, 127, 128, 128, 128, 128, 128, 128, 128, 128, 129, 129, 129, 130,
                130, 130, 130, 130, 131, 131, 131, 132, 133, 134, 135, 136, 137, 137, 138, 139,
                140, 141, 142, 143, 143, 144,
            ]
        );
    }
}
