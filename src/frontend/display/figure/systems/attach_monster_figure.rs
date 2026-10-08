//! Attach a monster's figure: the kind resolves through the binding
//! registry.

use bevy::prelude::*;

use crate::core::monster::components::monster_index::MonsterIndex;
use crate::frontend::display::figure::resources::creature_figure_registry::CreatureFigureRegistry;

/// Insert the bound figure handle onto every monster entering the world
/// with a new kind. Resolution happens once, at attach: a monster's
/// kind never changes. A kind matching no binding is a content error
/// and panics here — all spawns happen at startup today, so it surfaces
/// at launch. Shares the Attach phase with the other attach systems
/// without an explicit order: a monster missed in the same frame is
/// picked up the next (`Added` persists until observed), so a spawn
/// shows its sprite at worst one frame later — invisible at startup,
/// where all spawns happen today.
pub fn attach_monster_figure(
    mut commands: Commands,
    creature_figure_registry: Res<CreatureFigureRegistry>,
    query: Query<(Entity, &MonsterIndex), Added<MonsterIndex>>,
) {
    for (entity, monster_index) in &query {
        let figure_index = creature_figure_registry
            .get_monster_figure(*monster_index)
            .unwrap_or_else(|| {
                panic!("monster has no matching figure binding (kind {monster_index:?})")
            });
        commands.entity(entity).insert(figure_index);
    }
}
