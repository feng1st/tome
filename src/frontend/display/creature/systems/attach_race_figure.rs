//! Attach a humanoid creature's figure: race and class resolve through
//! the binding registry.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::class::components::class_index::ClassIndex;
use crate::core::race::components::race_index::RaceIndex;
use crate::frontend::display::creature::resources::creature_figure_registry::CreatureFigureRegistry;

/// A humanoid creature's identity handles: what the binding resolution
/// reads.
#[derive(QueryData)]
pub struct RaceIdentityQuery {
    pub entity: Entity,
    pub race_index: &'static RaceIndex,
    pub class_index: Option<&'static ClassIndex>,
}

/// Insert the bound figure handle onto every humanoid creature entering
/// the world with a new race. Resolution happens once, at attach:
/// identities are immutable for a creature's lifetime. An identity
/// matching no binding (a classless member of a race that defines only
/// class-keyed figures) is a content error and panics here — all spawns
/// happen at startup today, so it surfaces at launch. Shares the Attach
/// phase with the monster-side attachment without an explicit order: a
/// creature missed in the same frame is picked up the next (`Added`
/// persists until observed), so a spawn shows its sprite at worst one
/// frame later — invisible at startup, where all spawns happen today.
pub fn attach_race_figure(
    mut commands: Commands,
    creature_figure_registry: Res<CreatureFigureRegistry>,
    query: Query<RaceIdentityQuery, Added<RaceIndex>>,
) {
    for item in &query {
        let figure_index = creature_figure_registry
            .get_race_figure(*item.race_index, item.class_index.copied())
            .unwrap_or_else(|| {
                panic!(
                    "creature has no matching figure binding (race {:?}, class {:?})",
                    item.race_index, item.class_index
                )
            });
        commands.entity(item.entity).insert(figure_index);
    }
}
