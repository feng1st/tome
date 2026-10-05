# dungeon-generation Specification

## Purpose

Level generation: `src/generate.c` carries the whole level-generation pipeline —
the generation entry `generate_cave` (savefile level load / level type dispatch /
size selection / feeling conversion / auto_scum re-rolls), the dungeon orchestrator
`cave_gen` (fill / generator dispatch / staircases / monster and object quotas /
fates / final guardian with its treasure / DF1_DOUBLE doubling), the classic room
builder `level_generate_dungeon` (room family draws / tunnel chains / doors /
rivers and streamers), the twelve room building families, the seven random
vaults, the fractal caverns, the terrain placement primitives, grid mana, and
special levels (process_dungeon_file map loading). Wilderness generation
(wilderness_gen) is specified in `specs/wilderness/spec.md`.

## Requirements

### Requirement: Generation Entry and Level Dispatch

`generate_cave` SHALL: initialize dungeon_flags1/2 from the d_info flags (the
surface uses DUNGEON_WILDERNESS); match embedded town levels by t_level; run
save_all_friends then wipe_m_list; when seed_dungeon is nonzero switch to
Rand_quick seeded with seed_dungeon + dun_level (town levels use the town_info
seed instead); trigger HOOK_GEN_LEVEL_BEGIN; first try loading a savefile level
(load_dungeon), and on failure enter the generation re-roll loop — each round
wipes all arrays (MAX_HGT x MAX_WID terrain / objects / monsters / traps /
mimics / effects / inscriptions / flow), resets o_max to 1, resets the player and
panel, resets monster_level (DUNGEON_DEATH special-cases plev x 2 + 10 +
rand_int(40)) and object_level, zeroes rating / good_item_flag / ambush_flag /
fate_flag, and then dispatches: arena (arena_gen), quest level (quest_gen — only
raises the HOOK_GEN_QUEST hook), special level (build_special_level), surface
(wilderness_gen, or wilderness_gen_small under wild_mode), ordinary dungeon
(after size selection calls cave_gen; a false return re-rolls with
"could not place player").

#### Scenario: Seeded level generation

- **WHEN** `seed_dungeon` is nonzero and a new level generates
- **THEN** `Rand_quick` switches to the `seed_dungeon + dun_level` seed (town
 levels use the `town_info` seed instead)

#### Scenario: Quest level dispatch

- **WHEN** the dispatch round picks a quest level
- **THEN** `quest_gen` runs and only raises the `HOOK_GEN_QUEST` hook

- **Anchors**: `src/generate.c:8508-8620` (entry and load), `:8622-8879` (re-roll
 loop and dispatch)

### Requirement: Level Size, Feeling, and Re-roll Criteria

Size selection SHALL decide in order: explicit d_info size_x (size_y/size_x as
multiples of the screen); DF1_SMALLEST (one panel, no DF1_BIG); small levels
(no DF1_BIG and a draw among always_small_level / DF1_SMALL / the small_levels
option — one to three rows and one to two columns of screens: `rand_range(1,
MAX_HGT / SCREEN_HGT)` and `rand_range(1, MAX_WID / SCREEN_WID - 1)`); otherwise the full
MAX_HGT x MAX_WID; every tier except DF1_SMALLEST converts to panel counts as
(cur/SCREEN) x 2 - 2 (DF1_SMALLEST sets both panel counts to 1 directly) and
initializes the panel bounds to out-of-range values. Feeling conversion SHALL:
map rating bands >100/80/60/40/30/20/10 to feelings 2-10; give 1 (special feeling)
when good_item_flag is set and preserve is off; give 0 within the first 1000 turns
after entering; always give 0 on the surface. Re-roll criteria SHALL: object or
monster overflow (o_max >= max_o_idx, m_max >= max_m_idx) and auto_scum (limited
to 100 tries, non-quest levels below the surface, with the feeling ceiling relaxed across five depth
bands — ceilings 9/8/7/6/5 at depth 0/5/10/20/40) both force a re-roll; overflow
reasons are always reported, while the auto_scum "boring level" reason string is
only assigned when a cheat flag or precognition is on;
a re-roll wipes the object and monster lists, clears the fates icky flags, and
calls wipe_special_level. Town levels matched by t_level are exempt from
re-rolling (the round breaks on `okay || town_level`).

#### Scenario: Special feeling with preserve off

- **WHEN** `good_item_flag` is set and preserve is off
- **THEN** the level feeling is 1 (special feeling)

#### Scenario: Overflow forces a re-roll

- **WHEN** `o_max >= max_o_idx` or `m_max >= max_m_idx` holds at the end of a
 generation round
- **THEN** the level re-rolls and the reason is reported to cheaters and
 precognition

- **Anchors**: `src/generate.c:8773-8871` (size), `:8881-8899` (feeling),
 `:8902-8965` (re-roll)

### Requirement: Finalization

Generation finalization SHALL: assign mana to every grid with generate_grid_mana;
unless the player is in wild_mode, return group-marked monsters with
replace_all_friends;
clean up fates — FATE_FIND_A with an a_idx sets a_info cur_num to 1 and reverts the
fate to FATE_NONE; mark the special level generated via finalise_special_level;
reset last_teleportation_y/x to -1; set TOWN_KNOWN on town levels; set
character_dungeon true and record the current turn in old_turn; restore
seed_dungeon; give the astral player wiz_lite_extra over the whole level (only
off the surface); and set
p_ptr->energy to 100 (the player moves first on entry).

#### Scenario: Fate cleanup for a found artifact

- **WHEN** finalization cleans up a `FATE_FIND_A` fate carrying an `a_idx`
- **THEN** the artifact's `a_info` `cur_num` is set to 1 and the fate reverts to
 `FATE_NONE`

#### Scenario: Player moves first on entry

- **WHEN** finalization completes on a generated level
- **THEN** `p_ptr->energy` is set to 100

- **Anchors**: `src/generate.c:8967-9024`

### Requirement: Grid Mana

`generate_grid_mana` SHALL: draw XTRA_MAGIC to make a magical level (announcing
"Magical level" to cheaters and precognition), then set every grid's mana to
multiplier x m_bonus(255, dun_level) / 2 — multiplier 2 normally, 3 on a magical
level, magical levels add 10 + rand_int(10) — clamped to 0-255.

#### Scenario: Magical level draw

- **WHEN** the `XTRA_MAGIC` draw triggers for a level
- **THEN** "Magical level" is announced to cheaters and precognition, and every
 grid's mana uses multiplier 3 plus the `10 + rand_int(10)` bonus, clamped to
 0-255

- **Anchors**: `src/generate.c:8464-8498`

### Requirement: Dungeon Orchestrator

`cave_gen` SHALL: take the generator name from the level's saved command file
(the 'G' command via get_dungeon_generator), falling back to the d_info
generator name when the file gives none; halve the desired size first when
DF1_DOUBLE is set; fill the floor/wall family percent tables with init_feat_info;
run set_mon_num_hook and get_mon_num_prep; decrement max_vault_ok for each missing
panel dimension; run fill_level (per d_info fill_method) then set_bounders;
dispatch a registered level generator by name (the list registered through
add_level_generator) — a generator function returning false fails cave_gen; build
the staircase set — a branch puts five FEAT_MORE
(the branch parameter), a father branch puts five FEAT_LESS, and the regular set
places 3-4 down staircases and 0-1 down shafts when below maxdepth (or at
maxdepth with DF1_FORCE_DOWN), 1-2 up staircases and 0-1 up shafts when above
mindepth (or at mindepth without DF1_NO_UP), with FEAT_WAY_* replacing the
staircases under DF1_FLAT and DF2_NO_SHAFT disabling shafts; raise HOOK_GEN_LEVEL;
switch monster and object rolls to the global RNG when seed_dungeon is set; take
the quota base k = dun_level/3 clamped to 2-10; and when the generator wants
default monsters, place min_m_alloc_level (scaled down for small levels by area,
with a floor) plus randint(8) plus k rounds of alloc_monster.

- **Quirk:** when the requested generator name is registered nowhere, the dispatch
 loop walks off the list leaving the generator pointer NULL, and the following
 dereference is undefined; shipped data never exercises this path.

- **Discrepancy:** the father-branch comment says "Place 1 down stair" while the
 code places five up-staircases — `FEAT_LESS` is the correct direction toward the
 father dungeon.

#### Scenario: Branch staircases

- **WHEN** the generated dungeon is a branch
- **THEN** five `FEAT_MORE` grids are placed (a father branch instead places
 five `FEAT_LESS`)

#### Scenario: Flat level ways

- **WHEN** the dungeon has `DF1_FLAT` and the staircase set is built
- **THEN** `FEAT_WAY_*` grids replace the staircases

- **Anchors**: `src/generate.c:7614-7730` (orchestration), `:1033-1123`
 (alloc_stairs — against walls, relaxing after SAFE_MAX_ATTEMPTS of 5000 tries,
 DF1_FLAT uses place_new_way, branch stairs always placed with matching
 FEAT_LESS/MORE connectivity flags)

### Requirement: Object and Terrain Quotas

The cave_gen quota section SHALL: use k as above; place traps randint(k x 2)
(ALLOC_SET_BOTH) and rubble randint(k) (corridors) under the generator's
default_miscs flag; place room objects randnor(DUN_AMT_ROOM, 3), scattered
objects randnor(DUN_AMT_ITEM, 3), and gold randnor(DUN_AMT_GOLD, 3) under
default_objects (all three disabled in DUNGEON_DEATH); place altars, between
gates, and fountains at randnor(DUN_AMT_*, 3) each under default_miscs; when
final_guardian is set and dun_level equals maxdepth, temporarily raise
m_allow_special for the guardian race to place the guardian monster; while the
guardian lives, forge the final artifact — only if its a_info cur_num is still 0 —
(allowed through a_allow_special, apply_magic(-1, TRUE, TRUE, TRUE), found
recorded as OBJ_FOUND_MONSTER) and the final object — only if its k_info
artifact flag is still FALSE — (same k_allow_special route,
apply_magic(1, FALSE, FALSE, FALSE), k_info marked artifact = TRUE to prevent
regeneration) and push each onto the guardian's carry stack; and on an empty
level, run wiz_lite when the randint(DARK_EMPTY) draw misses (not 1), or — on a
hit — when a further randint(100) draw exceeds dun_level (so on a DARK_EMPTY
hit the level stays dark only when randint(100) does not exceed dun_level —
certainly dark at depth 100 or more, less often as depth decreases).

#### Scenario: Final guardian on maxdepth

- **WHEN** `final_guardian` is set and `dun_level` equals maxdepth
- **THEN** the guardian monster is placed with `m_allow_special` temporarily
 set

#### Scenario: Empty level lit

- **WHEN** the generated level is empty and the `randint(DARK_EMPTY)` draw misses
- **THEN** `wiz_lite` runs; a `DARK_EMPTY` hit still lights the level when a
 further `randint(100)` draw exceeds `dun_level`

- **Anchors**: `src/generate.c:7925-8107`

### Requirement: DF1_DOUBLE Doubling

DF1_DOUBLE SHALL, at the end of cave_gen, copy the generated level from the
bottom-right corner toward the top-left into a 2x2 set of four copies (border
permanent walls included), translating FEAT_BETWEEN special coordinates by doubling
them and adding the quadrant offset; objects and monsters each land in one random
quadrant of the four with ix/iy and fy/fx written back; finally cur_wid, cur_hgt,
py, and px are all doubled.

#### Scenario: Quadrant doubling

- **WHEN** `cave_gen` finishes on a dungeon with `DF1_DOUBLE`
- **THEN** the level content appears as a 2x2 set of four copies and `cur_wid`,
 `cur_hgt`, `py`, and `px` are all doubled

- **Anchors**: `src/generate.c:8133-8202`

### Requirement: Classic Generator

`level_generate_dungeon` SHALL: decide the empty level (DF1_EMPTY or the
empty_levels option drawing EMPTY_LEVEL — fill_level relays the whole floor); when
DF1_CAVERN and rand_int(dun_level/2) exceeds DUN_CAVERN, build_cavern carves a
central fractal cavity; decide the destroyed level (deeper than ten and
rand_int(DUN_DEST) == 0; quest levels, non-full-size levels, and DF1_NO_DESTROY are
exempt); initialize the room table at BLOCK size with dungeon_align nudging column
positions; raise HOOK_BUILD_ROOM1; try DUN_ROOMS times to build a room in a picked
block — destroyed levels weigh fractal caves (type10) by depth, otherwise plain
rooms; on non-town levels with ironman_rooms or rand_int(DUN_UNUSUAL) below
dun_level, draw among unusual rooms (always zero under ironman): type8 large vault
(k < 10, requires max_vault_ok > 1), type7 lesser vault (k < 25, requires
max_vault_ok > 0), type5 nest (k < 40), type6 mound (k < 55), type11 random vault
(k < 60); draw among normal rooms: type4 large room (k < 25), type3 cross (k < 45),
type2 overlapping rectangles (k < 65), type10 fractal cave (k < 80), type9 circular
room or type1 simple room (k < 90; swapped under DF1_CIRCULAR_ROOMS), type12 crypt
(k < 100); fall back to a plain room at the end of each try (type10 under
DF1_CAVE, otherwise 9/1 by the circular flag); force-build a type1 if no room
succeeded;
shuffle the room order and chain build_tunnel between pairs, restore outer walls to
FEAT_WALL_SOLID, and run try_doors over the dun->door candidates.

#### Scenario: Cavern carve

- **WHEN** `DF1_CAVERN` is set and `rand_int(dun_level/2)` exceeds `DUN_CAVERN`
- **THEN** `build_cavern` carves a central fractal cavity

#### Scenario: Forced simple room

- **WHEN** none of the `DUN_ROOMS` room-building tries succeeded
- **THEN** a type1 simple room is force-built

- **Anchors**: `src/generate.c:6777-7049` (main flow and draws), `:6661-6721`
 (room_build — room type dispatch type1-12, roomdep gate, crowded nest/mound
 refusal)

### Requirement: Rivers and Streamers

After connectivity and doors SHALL, in order: ToME-module Mordor and Angband lay
DUN_STR_MAG magma veins and DUN_STR_QUA quartz veins (build_streamer, chances
DUN_STR_MC / DUN_STR_QC); DF1_SAND_VEIN lays a sand vein on a one-in-four draw
(build_streamer FEAT_SANDWALL, chance DUN_STR_SC); destroyed levels run
destroy_level here; embedded town levels run town_gen; DF1_WATER_RIVER /
DF1_LAVA_RIVER each start a recursive river at one chance in four (add_river with
a two-terrain deep/shallow pair); DF1_WATER_RIVERS tries 3 + rand_int(2) river
slots and lays a water river on each rand_int(3) == 0 slot, DF1_LAVA_RIVERS tries
2 + rand_int(2) slots the same way for lava; DF1_NO_STREAMERS disables streamers —
when streamers run, DF1_FLAT with randint(20) > 15 lays randint(DUN_STR_QUA)
FEAT_SMALL_TREES tree bands (build_streamer2, killwall 1); at depth over 33 a
randint(20) > 15 draw lays shallow-water bands (randint(DUN_STR_QUA - 1),
killwall 0) and then, on a further randint(20) > 15, deep-water bands
(randint(DUN_STR_QUA), killwall 1); when that first draw misses and depth is
over 33, a randint(20) > 15 draw lays shallow-lava bands
(randint(DUN_STR_QUA), killwall 0) and then, on a further randint(20) > 15,
deep-lava bands (randint(DUN_STR_QUA - 1), killwall 1), otherwise the
shallow/deep water bands run again; set_bounders then new_player_spot close the pipeline (a
new_player_spot failure fails the generator).

- **Discrepancy:** the code comments claim water streamers for "Levels 1 -- 33"
 and lava for "Levels 34 --", but the conditions (`!(dun_level <= 33)` and
 `else if (dun_level > 33)`) restrict BOTH water and lava streamer bands to
 depth over 33 — no water streamers ever appear at depth 33 or below.

#### Scenario: Water river chance

- **WHEN** `DF1_WATER_RIVER` (or `DF1_LAVA_RIVER`) is set
- **THEN** one recursive river starts on a one-in-four draw, added with a
 two-terrain pair

#### Scenario: Destroyed level reshuffle

- **WHEN** a destroyed level passes the sand-vein stage
- **THEN** `destroy_level` runs here, before `town_gen` (wall digging plus object
 and monster reshuffle)

- **Anchors**: `src/generate.c:7100-7262` (rivers and streamers), `:1258-1423`
 (recursive_river/add_river), `:1424-1500` (build_streamer — random start
 advancing along a ddd direction, per-grid chance roll turning to ore),
 `:1629-1723` (destroy_level — wall digging plus object and monster reshuffle)

### Requirement: Room Building Families

The twelve room types SHALL each follow their anchors: type1 simple room (outer
wall / inner floor / random hidden doors), type2 overlapping rectangles (several
random rectangles), type3 cross (a central crossing hall), type4 large room
(including the crowded variant — a central monster pack), type5 monster nest (13
theme hooks vault_aux_*: jelly/animal/undead/chapel/kennel/treasure/clone/symbol/
orc/troll/giant/dragon/demon, with clone and symbol picking a template monster by
depth and rejecting uniques and out-of-depth monsters), type6 monster mound (theme
by depth, awake monsters placed in concentric rectangular ranks by sorted
level), type7 lesser vault and type8
large vault (text taken from v_info and parsed glyph by glyph by build_vault —
% granite outer wall, # granite inner wall, X permanent wall, * object-or-trap,
+ secret door, ^ trap, &/@/9/8 monsters at dun_level+5/+11/+9/+40, `,` a monster
and/or an object, A an object at dun_level+12, digits 0-7 paired between gates,
p/a/b/c/d/P/B pattern grids, G a glass wall, I an illusion wall; blank columns
are skipped), type9 circular room (walls on a radius
formula), type10 fractal cave (generate_hmap height-map recursive subdivision +
generate_fracave thresholding + fill_hack connectivity repair), type11 random vault
(seven-way dispatch: bubble/room/cave/maze/mini_c/castle/target), type12 crypt
(coffin walls on a grid mask). Room placement is gated by the roomdep minimum
depths (types 1-12: 1/1/3/3/5/5/5/10/1/3/10/10) unless ironman_rooms is set.
Once a nest or mound (type5/type6) builds, room_alloc sets dun->crowded and
room_build refuses further type5/type6 draws — at most one nest or mound per
level; the type7/type8 draws are additionally gated by max_vault_ok > 0 and
> 1.

- **Quirk:** max_vault_ok is never decremented after a vault builds (and the
 panel-based decrements in cave_gen touch a separate local that is never read),
 so the vault gates stay open for the whole level. Treasure inside rooms is
 laid out per the fill_treasure difficulty parameter.

#### Scenario: One nest or mound per level

- **WHEN** a type5 nest or type6 mound has already built on the level
- **THEN** further type5/type6 draws are refused while `dun->crowded` stays set

- **Anchors**: `src/generate.c:2058-2419` (type1-4), `:2944-3185` (type5 nest),
 `:3186-3820` (type6 mound), `:3821-3943` (type7/8 vault), `:3944-4050` (type9
 circular), `:4597-4652` (type10 wrapper), `:5975-6067` (type11 random vault),
 `:6068-6188` (type12 crypt), `:3543-3700` (build_vault text parsing),
 `:4053-4556` (fractal caverns), `:4751-4902` (fill_treasure), `:1969-2000`
 (room_alloc), `:324-339` (roomdep table), `:6661-6717` (room_build — room type
 dispatch type1-12, roomdep gate, crowded nest/mound refusal)

### Requirement: The Seven Random Vaults

The seven random vault forms SHALL: bubble (stacked bubbles + 500 random door
tries + fill_treasure), room (standard nested room with convert_extra reverting to
wall), cave (CAVE icky marking + fill_treasure), maze (r_visit recursive
divide-and-conquer maze), mini_c (a miniature cross keep), castle
(build_recursive_room recursive keep with decreasing power), target (concentric
rings around a sealed inner walled cell at the center — no door into it — filled
by fill_treasure at difficulty randint(3) + 3). Objects (vault_objects — 75%
object, 25% gold), traps (vault_traps — scattered by vault_trap_aux over the
area), and monsters (vault_monsters — enemy monsters placed sleeping and in
groups with monster_level temporarily at dun_level + 2) are each laid in per
their quotas.

#### Scenario: Target vault rings

- **WHEN** the target vault form builds
- **THEN** concentric rings are laid around the sealed inner walled cell holding
 the treasure fill

#### Scenario: Vault monster quotas

- **WHEN** `vault_monsters` fills a vault
- **THEN** enemy monsters are placed sleeping and in groups at
 `monster_level = dun_level + 2`

- **Anchors**: `src/generate.c:4903-5031` (bubble), `:5056-5164` (room),
 `:5165-5232` (cave), `:5233-5411` (maze), `:5412-5507` (mini_c), `:5508-5718`
 (recursive), `:5719-5844` (castle), `:5845-5974` (target), `:1822-1968` (vault
 objects / traps / monsters)

### Requirement: Terrain Placement Primitives

The placement primitives SHALL: place_up_stairs (FEAT_LESS, or FEAT_SHAFT_UP on
a one-in-three draw unless DF2_NO_SHAFT) / place_down_stairs (FEAT_MORE, or
FEAT_SHAFT_DOWN when dun_level + 4 <= maxdepth and a one-in-three draw and no
DF2_NO_SHAFT), both leaving special = 0;
place_magical_stairs (a magical special staircase toward a given next dungeon),
place_random_stairs (random up or down), place_new_way (path grids on flat
levels, walked inward from a level edge and accepted once is_safe_floor — TRUE
only for the d_info floor1/floor2/floor3 features — sees a connection);
place_rubble / place_altar / place_fountain (fountain sval drawn from potion
kinds whose level <= dun_level and which carry TR4_FOUNTAIN, 30% empty fountains,
else damroll(3, 4) doses); place_between (paired between gates whose special fields
point at each other); place_locked_door (FEAT_DOOR_HEAD + randint(7)),
place_secret_door, place_random_door (per mille: 300 open, 100 broken, 200
secret, 300 closed, 99 locked, 1 jammed);
new_player_spot (player landing — under DF1_FLAT a place_new_way grid, else up to
5000 draws of a naked non-icky floor; when dungeon_stair is set and DF2_NO_STAIR
clear and not a branch entry, a staircase under the player matching the travel
direction — up when arriving deeper, down when arriving shallower; a failure
fails the generator). alloc_object picks
qualifying grids per ALLOC_SET_ROOM/CORR/BOTH and the ALLOC_TYP_* categories.

#### Scenario: Random door mix

- **WHEN** `place_random_door` places a door
- **THEN** the per-mille draw is 300 open, 100 broken, 200 secret, 300 closed,
 99 locked (`FEAT_DOOR_HEAD + randint(7)`), and 1 jammed

#### Scenario: Between gates pair up

- **WHEN** `place_between` lays a between gate
- **THEN** the paired gates' special fields point at each other

- **Anchors**: `src/generate.c:383-460` (staircases), `:461-783` (ways and player
 landing), `:801-928` (rubble / altar / fountain / between / random stairs),
 `:929-1030` (door family), `:1125-1222` (alloc_object)

### Requirement: Special Levels and Hooks

`build_special_level` SHALL: stay limited to inside dungeons; look up the
special_lvl generated table by dun_level - mindepth (only ungenerated levels
pass); fetch the map file name with get_dungeon_special; use the full
MAX_HGT x MAX_WID with permanent walls as the base, set_mon_num_hook +
get_mon_num_prep, init_flags to INIT_CREATE_DUNGEON|INIT_POSITION, and set
process_dungeon_file_full true before reading the map with process_dungeon_file;
set the REGEN_HACK marker plus generate_special_feeling and good_item_flag, and add
forty to rating. `wipe_special_level` (raising HOOK_LEVEL_REGEN and clearing the
REGEN_HACK table) and `finalise_special_level` (raising HOOK_LEVEL_END_GEN and
turning REGEN_HACK into generated) pair together — inside the re-roll loop the
REGEN_HACK state allows generating again. The map command formats (D/B/%/A/Q lines
and friends) and process_dungeon_file are specified in `specs/map-format/spec.md`.

#### Scenario: Already-generated rejection

- **WHEN** `build_special_level` looks up its `special_lvl` entry and the level
 is already marked generated
- **THEN** the special level is refused (only ungenerated levels pass)

#### Scenario: Re-roll pairing

- **WHEN** `wipe_special_level` runs
- **THEN** `HOOK_LEVEL_REGEN` is raised and the `REGEN_HACK` table is cleared,
 and inside the re-roll loop the `REGEN_HACK` state allows generating again

- **Anchors**: `src/generate.c:8346-8459` (the special level trio), `:8337-8340`
 (quest_gen)

### Requirement: Arena Level

`arena_gen` SHALL: relay the full level with permanent walls (GLOW|MARK pre-lit);
light the floor area by the half-day cycle derived from turn (CAVE_GLOW, with
view_perma_grids adding MARK by day); `build_arena` draws an FEAT_PERM_EXTRA
octagonal wall ring around a screen-center offset and leaves one FEAT_SHOP grid
on the innermost row of the top (north) wall band, with the player placed
directly below it inside the ring; place_monster_aux puts
arena_monsters[p_ptr->arena_number] five grids below the player.

#### Scenario: Arena ring and shop grid

- **WHEN** `build_arena` draws the arena
- **THEN** an `FEAT_PERM_EXTRA` octagonal ring forms around the screen-center
 offset and one `FEAT_SHOP` grid sits on the north side with the player
 directly below it

#### Scenario: Arena monster placement

- **WHEN** the arena level's monster is placed
- **THEN** `place_monster_aux` puts `arena_monsters[p_ptr->arena_number]` five
 grids below the player

- **Anchors**: `src/generate.c:8211-8331`
