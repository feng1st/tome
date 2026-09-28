//! The animation clip value type.

/// One animation clip: a frame sequence into the entity's sprite sheet
/// and its playback rate. A pure value type — an appearance's anim table
/// (`Appearance`) stores these; per-frame lookups go through
/// `Appearance::clip()` with `Idle` fallback. The frame table is
/// data-owned (`Vec`, straight out of serde): nobody clones clips —
/// systems read them by reference — and a `Rc` could not sit inside a
/// `Send + Sync` resource anyway; if clips are ever shared across
/// appearances, upgrade to `Arc<[usize]>`.
#[derive(Clone)]
pub struct AnimClip {
    pub frames: Vec<usize>,
    pub fps: f32,
}
