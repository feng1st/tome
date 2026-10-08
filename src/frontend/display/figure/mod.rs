//! The figure domain: the figure vocabulary — what a creature may
//! present — and how each figure renders (sprite sheet, atlas layout,
//! anim table). One domain, one table file, one registry: ids resolve
//! to handles, handles to appearances. This domain owns the registry
//! and the attach system that mounts render data onto entities.

pub mod components;
pub mod resources;
pub mod systems;
pub mod types;
pub mod utils;

use bevy::prelude::*;

use self::resources::creature_figure_registry::CreatureFigureRegistry;
use self::resources::figure_registry::FigureRegistry;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the figure domain: both registries build at app build time
/// (`FromWorld`); the Attach phase runs the figure attachment (a
/// creature's identity resolves to its figure handle — monsters by
/// kind, humanoids by race and class) and the appearance attachment
/// (the handle resolves to render parts) — handles need no pixel
/// readiness, the renderer waits (brief pop-in accepted).
pub fn register(app: &mut App) {
    app.init_resource::<FigureRegistry>()
        .init_resource::<CreatureFigureRegistry>()
        .add_systems(
            Update,
            (
                systems::attach_appearance::attach_appearance,
                systems::attach_monster_figure::attach_monster_figure,
                systems::attach_race_figure::attach_race_figure,
            )
                .in_set(DisplayPhase::Attach),
        );
}
