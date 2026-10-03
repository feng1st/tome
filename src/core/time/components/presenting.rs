//! Presenting: the presentation-gate protocol flag.

use bevy::prelude::*;

/// Marks that the world driver's step presentation is still playing.
/// Written only by the display side (attached when the step tween opens,
/// detached when it completes) and read only by `advance` — the single
/// sanctioned cross-layer write, standing where a flag suits better than
/// a message: the clock gates on a *state*, not an event.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Presenting;
