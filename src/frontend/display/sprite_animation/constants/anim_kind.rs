//! The canonical animation vocabulary: every anim an appearance may
//! define.

use serde::Deserialize;
use serde::Serialize;

/// The superset of anims, shared by all appearances. References to
/// anims originate in code (a standing order implies `Run`; combat
/// facts imply `Attack` and `Die`), so the key type is an enum, not a
/// string — data files map these names to frame tables via serde
/// (lowercase). Each appearance defines only the subset it supports;
/// lookups fall back to `Idle`.
// `Hit` has no construction site yet: no appearance ships hit frames,
// and the vocabulary entry is reserved for the day one does.
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimKind {
    /// Standing still. Mandatory in every appearance (the fallback target).
    Idle,
    /// Fast locomotion.
    Run,
    /// Melee or ranged attack.
    Attack,
    /// Taking a hit.
    Hit,
    /// Death.
    Die,
}
