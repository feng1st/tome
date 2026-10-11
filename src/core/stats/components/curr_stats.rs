use bevy::prelude::*;

use crate::core::stats::types::stat_array::StatArray;

/// ToME2, types.h, player_type, stat_cur
#[derive(Component, Debug)]
pub struct CurrStats(pub StatArray<i32>);
