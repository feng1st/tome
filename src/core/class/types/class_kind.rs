use crate::core::stats::types::stat_array::StatArray;

/// ToME2, types.h, player_class
#[derive(Debug)]
pub struct ClassKind {
    // .., c_adj
    pub stat_modifiers: StatArray<i32>,
    // .., c_mhp
    pub hit_dice_modifier: i32,
}
