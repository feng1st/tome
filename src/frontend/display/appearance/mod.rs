//! The appearance domain's display side: how a creature looks — sprite
//! sheet, atlas layout, and anim table, resolved from the core's
//! `AppearanceKind` key. Appearances are data-file bound; this domain owns
//! the registry and the attach/loading systems.

pub mod constants;
pub mod resources;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the appearance domain: the registry is built on entering
/// `Game`; looks attach in the Attach phase — handles need no pixel
/// readiness, the renderer waits (brief pop-in accepted).
pub fn register(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::Game),
        systems::load_appearances::load_appearances,
    )
    .add_systems(
        Update,
        systems::attach_appearance::attach_appearance.in_set(DisplayPhase::Attach),
    );
}
