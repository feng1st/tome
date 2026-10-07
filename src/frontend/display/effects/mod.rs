//! The effects domain: the visual effects that hang off combat facts —
//! the flash of a wound, the splash of blood, the floating damage
//! number, the heavy hit's camera shake, and the death fade that
//! walks a claimed body to transparency. Facts come from the core;
//! effects call downward: tweens for fades, particles for sprays,
//! the camera for shakes.

pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use self::resources::text_stacks::TextStacks;
use crate::frontend::display::display_phase::DisplayPhase;

/// Register the effects domain: the stacking resource exists; the
/// presentations run in the Sync phase (reading the facts the core
/// wrote this frame) and the advancers in the Animate phase (time
/// belongs to presentation there).
pub fn register(app: &mut App) {
    app.init_resource::<TextStacks>().add_systems(
        Update,
        (
            systems::flash::flash,
            systems::show_status::show_status,
            systems::splash::splash,
            systems::shake::shake,
            systems::dead_fade::start_dead_fade,
            systems::dead_fade::release_dead_fade,
        )
            .in_set(DisplayPhase::Sync),
    );
    app.add_systems(
        Update,
        (
            systems::flash::update_flash,
            systems::show_status::update_floating_text,
        )
            .in_set(DisplayPhase::Animate),
    );
}
