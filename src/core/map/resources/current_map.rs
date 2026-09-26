// TODO: pending cleanup review — remove once stabilized
//! The map currently in play.

use bevy::prelude::*;

use crate::core::map::types::grid_map::GridMap;

/// Resource holding the active map. Holds the `GridMap` directly today;
/// when map switching lands (persistent dungeon levels, world map), it
/// becomes a handle into a map store — readers go through `map()` and are
/// unaffected.
#[derive(Resource)]
pub struct CurrentMap(GridMap);

impl CurrentMap {
    pub fn new(map: GridMap) -> Self {
        CurrentMap(map)
    }

    /// The active map's data.
    pub fn map(&self) -> &GridMap {
        &self.0
    }
}
