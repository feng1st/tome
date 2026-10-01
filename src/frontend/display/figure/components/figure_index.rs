//! The figure handle: a figure's slot in the figure registry, carried by
//! creature entities to declare the figure they present.

use bevy::prelude::*;

/// Runtime figure handle: the figure's index in the issuing registry,
/// assigned in file order at load time. Not an identity — the same
/// figure's value changes when the file's entry order changes, so no
/// code may construct or compare specific values, and persisted data
/// must go through the id mapping table instead. Carried as a component:
/// the binding registry inserts it after resolving the creature's
/// identity, and `attach_appearance` then resolves the render data
/// through the figure registry.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FigureIndex(u16);

impl FigureIndex {
    /// The handle for the registry slot `index`. Crate-internal: only
    /// the registry assigns handles.
    pub(crate) fn from_index(index: usize) -> Self {
        FigureIndex(index as u16)
    }

    /// The registry slot this handle points into. Crate-internal: only
    /// the issuing registry dereferences handles.
    pub(crate) fn index(self) -> usize {
        self.0 as usize
    }
}
