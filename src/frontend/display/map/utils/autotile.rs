//! Shore autotiling: an autotile cell renders as one of 16 shoreline
//! variants (16 consecutive tileset frames from its binding's base
//! frame) chosen by a 4-bit mask of its orthogonal neighbors. The bit is
//! set — no shoreline toward that neighbor — when the neighbor is not
//! walkable or is liquid (walls, other liquid, the map edge), in bit
//! order up, right, down, left. Fully masked cells become the fully
//! transparent open-liquid tile that lets the scrolling layer show
//! through.

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::local_map::LocalMap;

/// Tileset frame for the autotile cell at (x, y): base frame + 4-bit
/// neighbor mask.
pub fn shore_index(
    local_map: &LocalMap,
    terrain_registry: &TerrainRegistry,
    x: i32,
    y: i32,
    base: usize,
) -> usize {
    // Out-of-bounds neighbors count as no-shore: rooms are wall-bordered
    // and nothing renders past the edge anyway.
    let no_shore = |x: i32, y: i32| {
        local_map
            .get(CellCoord::new(x, y))
            .and_then(|terrain_index| terrain_registry.get(terrain_index))
            .is_none_or(|terrain| !terrain.is_walkable() || terrain.is_liquid())
    };
    let mut mask = 0usize;
    if no_shore(x, y - 1) {
        mask |= 1;
    }
    if no_shore(x + 1, y) {
        mask |= 1 << 1;
    }
    if no_shore(x, y + 1) {
        mask |= 1 << 2;
    }
    if no_shore(x - 1, y) {
        mask |= 1 << 3;
    }
    base + mask
}
