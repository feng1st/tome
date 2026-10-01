//! The race handle: a race's slot in the vocabulary registry, carried by
//! every creature entity to declare its race.

use bevy::prelude::*;

/// Runtime race handle: the race's index in the issuing registry,
/// assigned in file order at load time. Not an identity — the same
/// race's value changes when the file's entry order changes, so no code
/// may construct or compare specific values, and persisted data must go
/// through the id mapping table instead. Carried as a component: the
/// display side resolves the figure through its binding registry when
/// the handle is added to an entity.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RaceIndex(u16);

impl RaceIndex {
    /// The handle for the registry slot `index`. Crate-internal: only
    /// the registry assigns handles.
    pub(crate) fn from_index(index: usize) -> Self {
        RaceIndex(index as u16)
    }
}
