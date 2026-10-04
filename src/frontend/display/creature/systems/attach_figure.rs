//! Attaches a creature's figure. The core spawns creatures as pure game
//! data (identity handles + `CellCoord`); this system resolves the
//! identity through the binding registry and inserts the figure handle,
//! which the figure domain then renders — it stays identity-agnostic.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::creature::components::class_index::ClassIndex;
use crate::core::creature::components::race_index::RaceIndex;
use crate::core::creature::components::unique_index::UniqueIndex;
use crate::frontend::display::creature::resources::creature_figure_registry::CreatureFigureRegistry;

/// One creature's identity handles: what the binding resolution reads.
#[derive(QueryData)]
pub struct IdentityQuery {
    pub entity: Entity,
    pub race_index: &'static RaceIndex,
    pub class_index: Option<&'static ClassIndex>,
    pub unique_index: Option<&'static UniqueIndex>,
}

/// Insert the bound figure handle onto every creature entering the world
/// with a new identity. Resolution happens once, at attach: identities
/// are immutable for a creature's lifetime. An identity matching no
/// binding (a classless member of a race that defines only class-keyed
/// figures) is a content error and panics here — all spawns happen at
/// startup today, so it surfaces at launch. Shares the Attach phase with
/// the appearance attachment without an explicit order: a creature
/// missed in the same frame is picked up the next (`Added` persists
/// until observed), so a spawn shows its sprite at worst one frame
/// later — invisible at startup, where all spawns happen today.
pub fn attach_figure(
    mut commands: Commands,
    creature_figure_registry: Res<CreatureFigureRegistry>,
    query: Query<IdentityQuery, Added<RaceIndex>>,
) {
    for item in &query {
        let figure_index = creature_figure_registry
            .figure(
                *item.race_index,
                item.class_index.copied(),
                item.unique_index.copied(),
            )
            .unwrap_or_else(|| {
                panic!(
                    "creature has no matching figure binding (race {:?}, class {:?}, unique {:?})",
                    item.race_index, item.class_index, item.unique_index
                )
            });
        commands.entity(item.entity).insert(figure_index);
    }
}
