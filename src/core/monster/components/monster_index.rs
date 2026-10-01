//! The monster handle: a monster kind's slot in the vocabulary registry,
//! carried by monster entities to declare their kind.

use bevy::prelude::*;

/// Runtime monster handle: the kind's index in the issuing registry,
/// assigned in file order at load time. Not an identity — the same
/// kind's value changes when the file's entry order changes, so no code
/// may construct or compare specific values, and persisted data must go
/// through the id mapping table instead. Carried as a component: the
/// display side resolves the figure through its binding registry when
/// the handle is added to an entity.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MonsterIndex(u16);

impl MonsterIndex {
    /// The handle for the registry slot `index`. Crate-internal: only
    /// the registry assigns handles.
    pub(crate) fn from_index(index: usize) -> Self {
        MonsterIndex(index as u16)
    }

    /// The registry slot this handle points into. Crate-internal: only
    /// the issuing registry dereferences handles.
    pub(crate) fn index(self) -> usize {
        self.0 as usize
    }
}
