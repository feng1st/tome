//! The animation clip value type.

/// One animation clip: a frame sequence into the entity's sprite sheet
/// and its playback rate. A pure value type — the appearance's anim table
/// (`Appearance`) stores these; per-frame lookups go through
/// `Appearance::clip()` with `Idle` fallback.
#[derive(Clone, Copy)]
pub struct AnimClip {
    pub frames: &'static [usize],
    pub fps: f32,
}
