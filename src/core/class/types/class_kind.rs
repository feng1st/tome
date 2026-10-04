//! The class kind value type: a class's birth data.

/// A class's birth data as loaded from the class table: the statistic
/// modifiers merged into a classed creature's rolled base statistics,
/// and the class's share of the hit die. Not an entity: a classed
/// creature carries the class handle and its own rolled values. The id
/// lives in the registry's lookup map; logic queries data, never
/// identity.
pub struct ClassKind {
    /// One modifier per statistic, in vocabulary order.
    pub stat_modifiers: [i32; 6],
    /// The class's share of the hit die; the race adds its own.
    pub hit_die: u16,
}
