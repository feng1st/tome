//! Display-side phase labels: ordering within `GameLoop::Display`.
//! Presentation is mode-agnostic — the same pipeline serves world map and
//! local maps — so these phases are shared and mode differences are
//! expressed by `run_if(in_state(...))` on systems, not per-mode phases.

use bevy::prelude::*;

/// Sub-phases within `GameLoop::Display`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisplayPhase {
    /// Structural assembly: newly spawned entities get their renderable
    /// parts (sprite, camera target) via `Added<>` reactions.
    Attach,
    /// Core state flows into existing presentation components: transform
    /// sync, playback intent.
    Sync,
    /// Sprite and terrain animations advance.
    Animate,
    /// Camera follows its target.
    Camera,
}
