//! The figure handle: a figure's slot in the vocabulary registry,
//! carried by creature entities to declare the figure they present.

use bevy::prelude::*;

/// Runtime figure handle: the figure's index in the issuing registry,
/// assigned in file order at load time. Not an identity — the same
/// figure's value changes when the file's entry order changes, so no
/// code may construct or compare specific values, and persisted data
/// must go through the id mapping table instead. Carried as a component:
/// the display side resolves the appearance through its registry when
/// the handle is added to an entity.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FigureIndex(u16);

impl FigureIndex {
    /// The handle for the registry slot `index`. Crate-internal: only
    /// the registry assigns handles.
    pub(crate) fn from_index(index: usize) -> Self {
        FigureIndex(index as u16)
    }
}
