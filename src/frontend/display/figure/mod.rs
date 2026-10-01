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

use self::resources::figure_registry::FigureRegistry;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the figure domain: the registry builds at app build time
/// (`FromWorld`); appearances attach in the Attach phase — handles need
/// no pixel readiness, the renderer waits (brief pop-in accepted).
pub fn register(app: &mut App) {
    app.init_resource::<FigureRegistry>().add_systems(
        Update,
        systems::attach_appearance::attach_appearance.in_set(DisplayPhase::Attach),
    );
}
