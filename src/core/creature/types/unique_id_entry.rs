//! Serde layout of the unique registry's view into shared content files
//! (the monster table today, `npc.ron` when the NPC domain lands).

use serde::Deserialize;

/// One entry as the unique registry reads it: only the `unique_id`
/// field — serde skips the rest. The file is the contract and each
/// parser reads its own part; this view lets the unique vocabulary
/// aggregate individuals without touching the owning domain's entry
/// type.
#[derive(Deserialize)]
pub struct UniqueIdEntry {
    #[serde(default)]
    pub unique_id: Option<String>,
}
