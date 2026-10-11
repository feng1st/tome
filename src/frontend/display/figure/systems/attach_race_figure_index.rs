use bevy::prelude::*;

use crate::core::class::components::class_index::ClassIndex;
use crate::core::race::components::race_index::RaceIndex;
use crate::frontend::display::figure::resources::creature_figure_registry::CreatureFigureRegistry;

pub fn attach_race_figure_index(
    mut commands: Commands,
    creature_figure_registry: Res<CreatureFigureRegistry>,
    targets: Query<(Entity, &RaceIndex, Option<&ClassIndex>), Added<RaceIndex>>,
) {
    for (entity, &race_index, class_index) in &targets {
        let figure_index = creature_figure_registry
            .get_race_figure(race_index, class_index.copied())
            .unwrap_or_else(|| {
                panic!(
                    "creature has no matching figure binding (race {:?}, class {:?})",
                    race_index, class_index
                )
            });
        commands.entity(entity).insert(figure_index);
    }
}
