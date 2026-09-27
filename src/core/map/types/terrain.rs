//! Terrain value type: a terrain's game data.

use crate::core::map::constants::terrain_flags::TerrainFlags;

/// A terrain's game data as loaded from the terrain table. The id
/// exists for file references, error messages, and save serialization
/// only — logic queries the property methods, never identity.
pub struct Terrain {
    pub id: String,
    pub flags: TerrainFlags,
}

impl Terrain {
    /// Whether creatures can step onto and path through this terrain.
    pub fn walkable(&self) -> bool {
        self.flags.contains(TerrainFlags::PASSABLE)
    }

    /// Whether this terrain is a liquid body.
    pub fn liquid(&self) -> bool {
        self.flags.contains(TerrainFlags::LIQUID)
    }
}
