//! The class handle: a class's slot in the vocabulary registry, carried
//! by creature entities whose vocation refines their race. Optional —
//! open to players and monsters alike.

use bevy::prelude::*;

/// Runtime class handle: the class's index in the issuing registry,
/// assigned in file order at load time. Not an identity — the same
/// class's value changes when the file's entry order changes, so no code
/// may construct or compare specific values, and persisted data must go
/// through the id mapping table instead. Carried as a component: the
/// display side resolves the figure through its binding registry when
/// the handle is added to an entity.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ClassIndex(u16);

impl ClassIndex {
    /// The handle for the registry slot `index`. Crate-internal: only
    /// the registry assigns handles.
    pub(crate) fn from_index(index: usize) -> Self {
        ClassIndex(index as u16)
    }
}
