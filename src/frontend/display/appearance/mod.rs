//! The appearance domain's display side: how a creature looks — sprite
//! sheet, atlas layout, and anim table, resolved from the core's
//! `AppearanceKind` key. This domain owns the appearance registry and
//! the attach system.

pub mod constants;
pub mod resources;
pub mod systems;
pub mod types;

use bevy::prelude::*;

use self::resources::appearance_registry::AppearanceRegistry;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the appearance domain: the registry builds at app build time
/// (`FromWorld`); looks attach in the Attach phase — handles need no
/// pixel readiness, the renderer waits (brief pop-in accepted).
pub fn register(app: &mut App) {
    app.init_resource::<AppearanceRegistry>().add_systems(
        Update,
        systems::attach_appearance::attach_appearance.in_set(DisplayPhase::Attach),
    );
}
