//! Shore autotiling: an autotile cell renders as one of 16 shoreline
//! variants (16 consecutive tileset frames from its binding's base
//! frame) chosen by a 4-bit mask of its orthogonal neighbors. The bit is
//! set — no shoreline toward that neighbor — when the neighbor is not
//! walkable or is liquid (walls, other liquid, the map edge), in bit
//! order up, right, down, left. Fully masked cells become the fully
//! transparent open-liquid tile that lets the scrolling layer show
//! through.

use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::cell_coord::CellCoord;
use crate::core::map::types::local_map::LocalMap;

/// Tileset frame for the autotile cell at (x, y): base frame + 4-bit
/// neighbor mask.
pub fn shore_index(
    local_map: &LocalMap,
    terrain_registry: &TerrainRegistry,
    x: usize,
    y: usize,
    base: u16,
) -> u16 {
    // Out-of-bounds neighbors count as no-shore: rooms are wall-bordered
    // and nothing renders past the edge anyway.
    let no_shore = |x: i32, y: i32| {
        local_map
            .get(CellCoord::new(x, y))
            .and_then(|terrain_index| terrain_registry.get(terrain_index))
            .is_none_or(|terrain| !terrain.walkable() || terrain.liquid())
    };
    let (x, y) = (x as i32, y as i32);
    let mut mask = 0u16;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;

    const BASE: u16 = 48;

    fn terrain_registry() -> TerrainRegistry {
        parse_terrain_registry(
            "test",
            r#"[
                ( terrain: "floor", flags: ["PASSABLE"] ),
                ( terrain: "water", flags: ["LIQUID"] ),
            ]"#,
        )
    }

    /// A 3x3 map; `water` lists the cells drawn as liquid.
    fn map_with(water: &[(usize, usize)]) -> LocalMap {
        let mut rows = ["...".to_string(), "...".to_string(), "...".to_string()];
        for &(x, y) in water {
            rows[y].replace_range(x..x + 1, "~");
        }
        let doc = format!(
            "( legend: {{ '.': \"floor\", '~': \"water\" }}, rows: [ \"{}\", \"{}\", \"{}\" ], )",
            rows[0], rows[1], rows[2]
        );
        parse_local_map("test", &doc, &terrain_registry())
    }

    #[test]
    fn isolated_cell_has_shore_on_all_sides() {
        let local_map = map_with(&[(1, 1)]);
        assert_eq!(
            shore_index(&local_map, &terrain_registry(), 1, 1, BASE),
            BASE
        );
    }

    #[test]
    fn open_liquid_is_transparent_tile() {
        // A 3x3 pool: the center's four neighbors are all liquid.
        let local_map = map_with(&[
            (1, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (1, 2),
            (0, 0),
            (2, 0),
            (0, 2),
            (2, 2),
        ]);
        assert_eq!(
            shore_index(&local_map, &terrain_registry(), 1, 1, BASE),
            BASE + 15
        );
    }

    #[test]
    fn bit_order_is_up_right_down_left() {
        // Liquid only above: bit 0 set, no shoreline toward it.
        let local_map = map_with(&[(1, 0), (1, 1)]);
        assert_eq!(
            shore_index(&local_map, &terrain_registry(), 1, 1, BASE),
            BASE + 1
        );
        // Liquid only to the left: bit 3 set.
        let local_map = map_with(&[(0, 1), (1, 1)]);
        assert_eq!(
            shore_index(&local_map, &terrain_registry(), 1, 1, BASE),
            BASE + 8
        );
    }
}
