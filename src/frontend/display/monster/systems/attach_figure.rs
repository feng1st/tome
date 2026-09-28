//! Attaches a monster's figure. The core spawns monsters as pure game
//! data (a `MonsterIndex` kind handle + `Position`); this system
//! resolves the kind through the figure binding registry and inserts the
//! figure handle, which the generic creature pipeline then renders — it
//! stays monster-agnostic.

use bevy::prelude::*;

use crate::core::monster::components::monster_index::MonsterIndex;
use crate::frontend::display::monster::resources::monster_figure_registry::MonsterFigureRegistry;

/// Insert the bound figure handle onto every newly spawned monster.
/// Shares the Attach phase with the appearance attachment without an
/// explicit order: a monster missed in the same frame is picked up the
/// next (`Added` persists until observed), so a spawn shows its sprite
/// at worst one frame later — invisible at startup, where all spawns
/// happen today.
pub fn attach_figure(
    mut commands: Commands,
    monster_figure_registry: Res<MonsterFigureRegistry>,
    query: Query<(Entity, &MonsterIndex), Added<MonsterIndex>>,
) {
    for (entity, monster_index) in &query {
        let figure_index = monster_figure_registry.figure(*monster_index);
        commands.entity(entity).insert(figure_index);
    }
}
