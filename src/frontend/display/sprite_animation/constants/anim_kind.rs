//! The canonical animation vocabulary: every anim a creature template may
//! define.

/// The superset of anims, shared by all creature templates. References to
/// anims originate in code (a `MoveToCell` command implies `Run`, an
/// attack implies `Attack`), so the key type is an enum, not a string —
/// data files map these names to frame tables via serde. Each template
/// defines only the subset it supports; lookups fall back to `Idle`.
// The superset is defined ahead of its consumers: combat variants are
// constructed once attack/hit/death commands land.
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AnimKind {
    /// Standing still. Mandatory in every template (the fallback target).
    Idle,
    /// Slow locomotion.
    Walk,
    /// Fast locomotion.
    Run,
    /// Melee or ranged attack.
    Attack,
    /// Taking a hit.
    Hit,
    /// Death.
    Die,
}
