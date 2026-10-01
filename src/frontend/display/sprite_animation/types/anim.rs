//! The animation value type.

/// One animation: a frame sequence into the entity's sprite sheet and
/// its playback rate. A pure value type — an appearance's anim table
/// (`Appearance`) stores these; per-frame lookups go through
/// `Appearance::anim()` with `Idle` fallback. The frame table is
/// data-owned (`Vec`, straight out of serde): nobody clones anims —
/// systems read them by reference — and a `Rc` could not sit inside a
/// `Send + Sync` resource anyway; if anims are ever shared across
/// appearances, upgrade to `Arc<[usize]>`.
#[derive(Clone)]
pub struct Anim {
    pub frames: Vec<usize>,
    pub fps: f32,
}
