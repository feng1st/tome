//! The unique handle: an individual's slot in the vocabulary registry,
//! carried by named creatures — quest NPCs and unique monsters — whose
//! identity is one specific individual rather than a kind.

use bevy::prelude::*;

/// Runtime unique handle: the individual's index in the issuing
/// registry, assigned in file order at load time. Not an identity — the
/// same individual's value changes when the file's entry order changes,
/// so no code may construct or compare specific values, and persisted
/// data must go through the id mapping table instead. Carried as a
/// component: the display side resolves the figure through its binding
/// registry when the handle is added to an entity.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct UniqueIndex(u16);

impl UniqueIndex {
    /// The handle for the registry slot `index`. Crate-internal: only
    /// the registry assigns handles.
    pub(crate) fn from_index(index: usize) -> Self {
        UniqueIndex(index as u16)
    }
}
