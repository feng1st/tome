# wilderness Specification

## Purpose

Wilderness and town generation: `src/wild.c` carries the plasma fractal wilderness
block generation (generate_area), the two full-view and small-view wilderness
generators (wilderness_gen / wilderness_gen_small), the map-writing exposure
(reveal_wilderness_around_player), and the three town layout forms (town_gen:
square / circle / hidden). The calling-side entry (the generate_cave dispatch) is
specified in `specs/dungeon-generation/spec.md`; the w_info/t_info map data is
specified in `specs/content-maps/spec.md`.

## Requirements

### Requirement: Plasma Fractal Blocks

The fractal pieces SHALL: perturb_point_mid take the four-corner average plus a
plus-or-minus rough perturbation (remainders carried), clamped to 0..depth_max,
written into cave.feat (used as a height cache); perturb_point_end mirror the same
structure on triangles; plasma_recursive subdivide the four quadrants recursively
until neighbors touch. `generate_area` SHALL: take town_num from the wf_info
entrance or the wild_map entrance; switch to Rand_quick with a seed so generation
is deterministic — the four corners get random heights, and when not a corner
the whole map is relayed with the median terrain then subdivided with
plasma_recursive(1,1, MAX-2); then convert heights to real terrain through the
wf_info[local] terrain
table; town checks (town_num 0-999) load through t_info.txt; per the wf_info road
bits lay four-way FEAT_FLOOR roads along the axes; when entrance >= 1000 put
FEAT_MORE with special = entrance - 1000 on a random interior point; when no floor
grid exists, plant grass at the center; set monster_level and object_level to the
wf_info level; and return the floor count.

#### Scenario: Dungeon entrance block

- **WHEN** `generate_area` runs a block whose entrance is >= 1000
- **THEN** `FEAT_MORE` with `special = entrance - 1000` lands on a random
 interior point

#### Scenario: Floorless block gets grass

- **WHEN** a generated block holds no floor grid at all
- **THEN** grass is planted at the center

- **Anchors**: `src/wild.c:32-121` (plasma), `:136-374` (generate_area)

### Requirement: The Two Wilderness States

`wilderness_gen` SHALL: initialize w_info.txt first; run generate_area over each
of the four neighbors and four corners (border mode counts only edges, corner mode
only corners) collecting the border structures, then generate the current block in
full; set the four edges to FEAT_PERM_SOLID with the neighbor blocks' edges as
mimic displays, corners likewise; fix day and night by the half-day cycle of turn —
by day the whole map gets CAVE_GLOW (view_perma_grids adds MARK), by night
non-FF1_REMEMBER grids lose GLOW/MARK; player_place(oldpy/oldpx); when not a
refresh, alloc_monster per MIN_M_ALLOC_TN (60 in encounter mode, with monsters
awake at distance zero instead of sleeping ones at distance three; every placed
monster is MSTATUS_ENEMY; the count is clamped to the floor count minus one) and
set ambush_flag; turn REWARDED quests into FINISHED; raise HOOK_WILD_GEN(FALSE).
`wilderness_gen_small` SHALL relay the whole map with FEAT_EKKAIA first, then after
the w_info initialization walk the wild_map placing entrance staircases (entrance
>= 1000 records special) or the wf_info feat per grid, lighting and memorizing
known grids; the player is placed at wilderness_x/y; the REWARDED conversion plus
HOOK_WILD_GEN(TRUE) work as before. `reveal_wilderness_around_player` SHALL mark a
circle of radius w when h is zero, otherwise a rectangle, as known, also lighting
it up under wild_mode.

#### Scenario: Day lights the wild

- **WHEN** `wilderness_gen` runs by day (by the half-day cycle of `turn`)
- **THEN** the whole map gets `CAVE_GLOW` (with `view_perma_grids` adding
 `MARK`); by night non-`FF1_REMEMBER` grids lose GLOW/MARK

#### Scenario: Encounter mode crowd

- **WHEN** `wilderness_gen` runs an encounter (not a refresh)
- **THEN** `alloc_monster` runs per `MIN_M_ALLOC_TN` (60), every placed monster
 is `MSTATUS_ENEMY` and awake at distance zero, and the count is clamped to the
 floor count minus one

- **Anchors**: `src/wild.c:385-576` (large map), `:582-642` (small map),
 `:645-702` (exposure)

### Requirement: Town Generation

The store pieces SHALL: `build_store` draw a random-bounded FEAT_PERM_EXTRA
rectangular building centered on (qy+yy*9+6, qx+xx*14+12), pick a door direction
out of four (re-rolled when it would sit against the map edge), and set the door
grid to FEAT_SHOP with special = store number plus CAVE_FREE; `build_store_circle`
draws a round building plus a floor channel from the center to the door;
`build_store_hidden` places a single-grid store. `get_shops` SHALL scan st_info
for SF1_RANDOM stores, rolling each against a base 50 modified by COMMON +30 /
RARE -20 / VERY_RARE -30. The border piece `set_border` SHALL turn floors or doors
into FEAT_DOOR_HEAD and walls into FEAT_PERM_SOLID, clearing mimic/special and
setting CAVE_ROOM; `town_borders` applies it along the four screen edges.
Townspeople SHALL use the create_townpeople_hook limited to d_char 't', placing a
townsperson on one percent of free CAVE_FREE grids (allowed through
m_allow_special then taken back, with low-level ones force-leveled to dun_level/2
experience).

The three `town_gen` forms SHALL (abandoning levels smaller than the screen,
center-aligned): `town_gen_hack` square — seven times in ten (magik of
`TOWN_NORMAL_FLOOR`, 70) floors are FEAT_FLOOR
and otherwise drawn from the floor_type table, the whole screen floored with
CAVE_ROOM|CAVE_FREE, then get_shops feeds two rows of four build_store calls, then
town_borders with the same seven-in-ten chance, then townspeople; `town_gen_circle`
round — a central horizontal band plus two half-circles (rad = half the screen
height) of border and floor, build_store_circle in two rows of four, no outer
wall, townspeople as before; `town_gen_hidden` hidden — no borders and no floors,
with n = rand_int(store count/2) + store count/2 stores scattered at random onto
empty grids. The form is drawn with rand_int(3), wizard mode announces the form
and seed, and p_ptr->town_num is set last.

#### Scenario: Store door re-rolled

- **WHEN** `build_store`'s door draw would sit against the map edge
- **THEN** the direction is re-rolled

#### Scenario: Hidden town scatters stores

- **WHEN** `town_gen_hidden` builds a hidden town
- **THEN** `rand_int(store count/2) + store count/2` stores are scattered at
 random onto empty grids, with no borders and no floors

- **Anchors**: `src/wild.c:724-882` (the three store pieces), `:885-912` (store
 selection), `:915-973` (borders), `:975-981` (townsperson hook), `:991-1080`
 (square), `:1082-1211` (circle), `:1214-1246` (hidden), `:1266-1316` (dispatch)
