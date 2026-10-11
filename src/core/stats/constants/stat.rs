/// ToME2, defines.h, A_*
#[derive(Clone, Copy, Debug)]
pub enum Stat {
    Strength,
    Intelligence,
    Wisdom,
    Dexterity,
    Constitution,
    Charisma,
}

impl Stat {
    pub const COUNT: usize = 6;

    pub const ALL: [Stat; Self::COUNT] = [
        Stat::Strength,
        Stat::Intelligence,
        Stat::Wisdom,
        Stat::Dexterity,
        Stat::Constitution,
        Stat::Charisma,
    ];

    pub(crate) fn index(self) -> usize {
        self as usize
    }
}
