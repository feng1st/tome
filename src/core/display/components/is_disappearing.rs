//! IsDisappearing: the disappearance-in-progress flag protocol.

use bevy::prelude::*;

/// Marks that a dead creature's exit presentation — its death
/// animation and fade — is still running. Written only by the display
/// side (raised the moment death lands, held through the animation and
/// the fade, dropped when both end) and read only by health's
/// `despawn_dead`: the body leaves the world once its disappearance
/// completes. The driver never carries the flag — its death
/// presentation is the frozen world, not a disappearance — so a dead
/// driver stays until the death wrap-up owns it. With no display side
/// running, nobody raises the flag and the dead leave on the next
/// sweep: headless, death alone empties the world.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct IsDisappearing;
