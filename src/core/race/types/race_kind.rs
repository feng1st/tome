use crate::core::stats::types::stat_array::StatArray;

/// ToME2, types.h, player_race
#[derive(Debug)]
pub struct RaceKind {
    // .., r_adj
    pub stat_modifiers: StatArray<i32>,
    // .., r_mhp
    pub hit_dice_modifier: i32,
}
