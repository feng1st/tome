//! The terrain handle: a terrain's slot in the registry.

/// Runtime terrain handle: the terrain's index in the issuing registry,
/// assigned in file order at load time. Not an identity — the same
/// terrain's value changes when the file's entry order changes, so no
/// code may construct or compare specific values, and persisted data
/// must go through the id mapping table instead.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TerrainIndex(u16);

impl TerrainIndex {
    /// The handle for the registry slot `index`. Crate-internal: only the
    /// registry assigns handles.
    pub(crate) fn from_index(index: usize) -> Self {
        TerrainIndex(index as u16)
    }

    /// The registry slot this handle points into.
    pub(crate) fn index(self) -> usize {
        self.0 as usize
    }
}
