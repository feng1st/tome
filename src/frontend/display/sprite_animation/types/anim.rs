//! The animation value type.

/// One animation: a frame sequence into the entity's sprite sheet, its
/// playback rate, and whether it loops. A looping animation wraps its
/// playback; a non-looping animation clamps at its last frame — the
/// held end state of a one-shot action. A pure value type — an
/// appearance's anim table (`Appearance`) stores these; per-frame
/// lookups go through `Appearance::anim()` with `Idle` fallback.
/// Nobody clones anims — systems read them by reference — and a `Rc`
/// could not sit inside a `Send + Sync` resource anyway; if anims are
/// ever shared across appearances, upgrade to `Arc<[usize]>`.
#[derive(Clone)]
pub struct Anim {
    pub frames: Vec<usize>,
    pub fps: f32,
    pub looped: bool,
}
