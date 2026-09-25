//! Display-side phase labels: ordering within `FramePhase::Display`.
//! Presentation is mode-agnostic — the same pipeline serves world map and
//! local maps — so these phases are shared and mode differences are
//! expressed by `run_if(in_state(...))` on systems, not per-mode phases.

use bevy::prelude::*;

/// Sub-phases within `FramePhase::Display`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisplayPhase {
    /// Core state flows into presentation: appearance attach, transform
    /// sync, chunk builds.
    Sync,
    /// Sprite and terrain animations advance.
    Animate,
    /// Camera follows its target.
    Camera,
}
