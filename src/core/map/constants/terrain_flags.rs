//! Terrain property flags: the only terrain attributes game logic may
//! query. Terrain identity (ids, indexes) never decides behavior.

use bitflags::bitflags;

bitflags! {
    /// Per-terrain property bits, loaded from the terrain table file.
    /// A flag's absence means the property does not hold.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct TerrainFlags: u8 {
        /// Creatures can step onto and path through the cell.
        const PASSABLE = 0b001;
        /// The cell is a liquid body (drives liquid visuals).
        const LIQUID = 0b010;
    }
}
