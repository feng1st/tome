//! The race kind value type: a race's birth data.

/// A race's birth data as loaded from the race table: the statistic
/// modifiers merged into a played creature's rolled base statistics,
/// and the race's share of the hit die. Not an entity: a played
/// creature carries the race handle and its own rolled values. The id
/// lives in the registry's lookup map; logic queries data, never
/// identity.
pub struct RaceKind {
    /// One modifier per statistic, in vocabulary order.
    pub stat_modifiers: [i32; 6],
    /// The race's share of the hit die; the class adds its own.
    pub hit_die: u16,
}
