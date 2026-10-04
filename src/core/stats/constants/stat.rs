//! The six statistics: the fixed vocabulary every stat-carrying array
//! is indexed by.

/// The six statistics a playable creature is rolled at birth, in their
/// fixed vocabulary order — the array order of every stat-carrying
/// table and component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stat {
    Strength,
    Intelligence,
    Wisdom,
    Dexterity,
    Constitution,
    Charisma,
}

impl Stat {
    /// Every statistic, in vocabulary order.
    pub const ALL: [Stat; 6] = [
        Stat::Strength,
        Stat::Intelligence,
        Stat::Wisdom,
        Stat::Dexterity,
        Stat::Constitution,
        Stat::Charisma,
    ];

    /// The array slot this statistic occupies in stat-carrying arrays.
    pub fn index(self) -> usize {
        self as usize
    }
}
