//! Identity registries: every vocabulary id resolved to its runtime
//! handle. One file per vocabulary: the resource, its construction from
//! the vocabulary file, and the file's parsing and validation.

pub mod class_registry;
pub mod race_registry;
pub mod unique_registry;
