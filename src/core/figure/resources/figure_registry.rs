//! The figure registry: every figure id resolved to its runtime handle.
//! One theme, one file: the resource, its construction from the
//! vocabulary file, and the file's parsing and validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::figure::components::figure_index::FigureIndex;

/// The figure vocabulary loaded at startup.
pub const FIGURE_TABLE_PATH: &str = "data/core/figures.ron";

/// Registry of all figures, built once at startup from the vocabulary
/// file. `FigureIndex` is the index into the internal table, assigned in
/// file order — an unstable runtime handle, never an identity: the same
/// figure's index changes when the file's entry order changes. Spawn
/// sites resolve ids to handles here; ids serve file references, error
/// messages, and save serialization only.
#[derive(Resource)]
pub struct FigureRegistry {
    ids: Vec<String>,
    by_id: HashMap<String, FigureIndex>,
}

impl FigureRegistry {
    /// The handle for a figure id, if the id is declared. Ids resolve
    /// only at content boundaries (spawn sites, display bindings).
    pub fn get_index(&self, id: &str) -> Option<FigureIndex> {
        self.by_id.get(id).copied()
    }

    /// All declared figure ids with their handles. Crate-internal: load
    /// validation that must cover every figure iterates here.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (FigureIndex, &str)> {
        self.ids
            .iter()
            .enumerate()
            .map(|(index, id)| (FigureIndex::from_index(index), id.as_str()))
    }
}

impl FromWorld for FigureRegistry {
    /// Build from the vocabulary file. The registry has no resource
    /// dependencies; dependent registries pull it via
    /// `World::get_resource_or_init`.
    fn from_world(_world: &mut World) -> Self {
        let text = fs::read_to_string(FIGURE_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read figure table '{FIGURE_TABLE_PATH}': {e}"));
        parse_figure_registry(FIGURE_TABLE_PATH, &text)
    }
}

/// Parse and validate figure vocabulary text. Split from file IO
/// (`FromWorld`) so tests can exercise it with inline documents.
pub(crate) fn parse_figure_registry(path: &str, text: &str) -> FigureRegistry {
    let ids: Vec<String> = ron::from_str(text)
        .unwrap_or_else(|e| panic!("figure table '{path}' is not valid RON: {e}"));
    let mut by_id = HashMap::with_capacity(ids.len());
    for (index, id) in ids.iter().enumerate() {
        assert!(!id.is_empty(), "figure table '{path}': empty figure id");
        assert!(
            by_id
                .insert(id.clone(), FigureIndex::from_index(index))
                .is_none(),
            "figure table '{path}': duplicate figure id '{id}'"
        );
    }
    FigureRegistry { ids, by_id }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"[ "warrior", "rat" ]"#;

    #[test]
    fn ids_resolve_to_handles() {
        let figure_registry = parse_figure_registry("test", DOC);
        let warrior = figure_registry.get_index("warrior").unwrap();
        let rat = figure_registry.get_index("rat").unwrap();
        // Same id, same handle; different ids, different handles. Concrete
        // values are the internal table index: file order decides them,
        // and logic must never depend on them.
        assert_eq!(warrior, figure_registry.get_index("warrior").unwrap());
        assert_ne!(warrior, rat);
        assert!(figure_registry.get_index("wolf").is_none());
    }

    #[test]
    fn iter_covers_every_id_in_file_order() {
        let figure_registry = parse_figure_registry("test", DOC);
        let ids: Vec<&str> = figure_registry.iter().map(|(_, id)| id).collect();
        assert_eq!(ids, ["warrior", "rat"]);
    }

    #[test]
    #[should_panic(expected = "duplicate figure id 'warrior'")]
    fn duplicate_id_panics() {
        parse_figure_registry("test", r#"[ "warrior", "warrior" ]"#);
    }

    #[test]
    #[should_panic(expected = "empty figure id")]
    fn empty_id_panics() {
        parse_figure_registry("test", r#"[ "" ]"#);
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_figure_registry("test", "this is not ron");
    }
}
