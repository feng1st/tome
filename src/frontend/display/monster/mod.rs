//! The display side of monsters: which figure a monster kind presents
//! (the binding registry), and attaching that figure to monster
//! entities. Rendering the figure is the appearance domain's generic
//! pipeline.

pub mod resources;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use self::resources::monster_figure_registry::MonsterFigureRegistry;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the monster display domain: the binding registry builds at
/// app build time (`FromWorld`); figures attach in the Attach phase.
pub fn register(app: &mut App) {
    app.init_resource::<MonsterFigureRegistry>().add_systems(
        Update,
        systems::attach_figure::attach_figure.in_set(DisplayPhase::Attach),
    );
}
