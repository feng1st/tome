//! The display side of creatures: which figure a creature presents
//! (the binding registry), and attaching that figure to creatures.
//! Rendering the figure is the figure domain's generic pipeline.
//! This domain is slated to merge into the figure domain — binding
//! is figure selection, and selection belongs beside the table it
//! reads; the merge lands when the next change touches either side.

pub mod resources;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use self::resources::creature_figure_registry::CreatureFigureRegistry;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the creature display domain: the binding registry builds
/// at app build time; figures attach in the Attach phase — monsters
/// through their kind, humanoid creatures through race and class. The
/// attach systems touch disjoint entities, so they need no order
/// between themselves.
pub fn register(app: &mut App) {
    app.init_resource::<CreatureFigureRegistry>().add_systems(
        Update,
        (
            systems::attach_race_figure::attach_race_figure,
            systems::attach_monster_figure::attach_monster_figure,
        )
            .in_set(DisplayPhase::Attach),
    );
}
