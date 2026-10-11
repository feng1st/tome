use bevy::prelude::*;

use crate::core::stats::types::stat_array::StatArray;

/// ToME2, types.h, player_type, stat_max
#[derive(Component, Debug)]
pub struct MaxStats(#[allow(dead_code)] pub StatArray<i32>);
