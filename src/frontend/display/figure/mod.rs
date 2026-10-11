pub mod components;
pub mod resources;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use self::resources::creature_figure_registry::CreatureFigureRegistry;
use self::resources::figure_registry::FigureRegistry;
use crate::frontend::display::display_phase::DisplayPhase;

pub fn register(app: &mut App) {
    app.init_resource::<FigureRegistry>()
        .init_resource::<CreatureFigureRegistry>()
        .add_systems(
            Update,
            (
                (
                    systems::attach_monster_figure_index::attach_monster_figure_index,
                    systems::attach_race_figure_index::attach_race_figure_index,
                ),
                systems::attach_figure::attach_figure,
            )
                .chain()
                .in_set(DisplayPhase::Attach),
        );
}
