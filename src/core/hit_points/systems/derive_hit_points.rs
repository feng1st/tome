use bevy::prelude::*;

use crate::core::class::components::class_index::ClassIndex;
use crate::core::class::resources::class_registry::ClassRegistry;
use crate::core::hit_points::components::hit_points::HitPoints;
use crate::core::hit_points::utils::max_hit_points::max_hit_points;
use crate::core::race::components::race_index::RaceIndex;
use crate::core::race::resources::race_registry::RaceRegistry;
use crate::core::stats::components::curr_stats::CurrStats;

pub fn derive_hit_points(
    mut targets: Query<(&mut HitPoints, &CurrStats, &RaceIndex, &ClassIndex), Changed<CurrStats>>,
    race_registry: Res<RaceRegistry>,
    class_registry: Res<ClassRegistry>,
) {
    for (mut hit_points, curr_stats, &race_index, &class_index) in &mut targets {
        let hit_dice_modifier = race_registry.race_kind(race_index).hit_dice_modifier
            + class_registry.class_kind(class_index).hit_dice_modifier;
        let new_value = max_hit_points(hit_dice_modifier, curr_stats);
        hit_points.max = new_value;
        if hit_points.current > new_value {
            hit_points.current = new_value;
        }
    }
}
