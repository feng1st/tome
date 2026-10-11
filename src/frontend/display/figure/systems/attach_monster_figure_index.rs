use bevy::prelude::*;

use crate::core::monster::components::monster_index::MonsterIndex;
use crate::frontend::display::figure::resources::creature_figure_registry::CreatureFigureRegistry;

pub fn attach_monster_figure_index(
    mut commands: Commands,
    creature_figure_registry: Res<CreatureFigureRegistry>,
    targets: Query<(Entity, &MonsterIndex), Added<MonsterIndex>>,
) {
    for (entity, &monster_index) in &targets {
        let figure_index = creature_figure_registry
            .get_monster_figure(monster_index)
            .unwrap_or_else(|| {
                panic!("monster has no matching figure binding (kind {monster_index:?})")
            });
        commands.entity(entity).insert(figure_index);
    }
}
