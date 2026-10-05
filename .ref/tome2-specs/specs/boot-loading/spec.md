# boot-loading Specification

## Purpose

Boot loading: `src/init2.c` carries the game startup load chain — directory layout, the
binary image (raw) caching scheme for the word-lists and the loader template, array
allocation (`init_other`), monster/object allocation table construction (`init_alloc`),
miscellaneous initialization (`init_misc`: stat-power/quest/god initial values and
misc.txt), and the load order of the main entry `init_angband`.

## Requirements

### Requirement: Directory Layout

`init_file_paths` SHALL derive eighteen subdirectory globals from the base path (apex/bone/
core/dngn/data/edit/file/help/info/mods/patch/scpt/pref/save/user/note/cmov/xtra, with the
base path minus its trailing separator stored in ANGBAND_DIR); under PRIVATE_USER_PATH the
USER/NOTE/CMOV directories are recompiled to point into the private directory plus version
suffix, SAVE follows into the private directory with savefile_setuid zeroed, and
PRIVATE_USER_PATH_MODULES/APEX/DATA redirect their directories (DATA with a /data suffix).

- **Anchors**: `src/init2.c:72-311`

### Requirement: Word-List Loading Template

Every word-list SHALL run the same three-stage flow (init_X_info): first try X_info.raw
under lib/data — read and verify the header's eight fields (version, info_num, info_len,
head_size, info_size — init_X_info_raw), compare the txt modification time under
CHECK_MODIFICATION_TIME, and accept the raw on a hit; on failure build a fake array
(FAKE_NAME_SIZE/FAKE_TEXT_SIZE) and parse X_info.txt under lib/edit (the init1.c parser),
reporting the line and record numbers with the nine err_str messages on error and quitting;
then fd_write the header/info/name/text sections into a fresh raw and free the fake array;
finally force a reload from the raw as the final data. Special cases SHALL: the raw fast
paths of al_info and v_info are disabled by `#if 0` (fd always -1) — every start reparses
the txt, writes the raw, and reads it back; the al loader stuffs the a_select_flags
structure array into the text_size slot on both write and read; the p_info loader
concatenates three headers (rp/rmp/c) plus bg plus meta_class (including the per-element
classes sub-arrays) plus the four gen_skill arrays, and its error message reads
"df 'p_info.txt'" as shipped (the spelling defect is recorded as-is); the _raw loaders
support DELAY_LOAD_*_TEXT to skip loading the text section.

- **Anchors**: `src/init2.c:408-684` (f template prototype), `:336-347` (err_str), `:355-388` (modification-time check), `:2956-3113` (p triple raw), `:3779-4127` (p loader), `:5283-5566` (al special case), `:5571-5853` (v special case)

### Requirement: Miscellaneous Initialization

`init_misc` SHALL: allocate the quark pool and the five message ring-buffer arrays (tail
set to MESSAGE_BUF); copy the power table from powers_type_init with POWER_MAX_INIT and
build the ownership bitmap via reinit_powers_type; copy the quest table from
quest_init_tome with MAX_Q_IDX_INIT — when the Lua `get_module_info` C_quest value is
false, every quest init hook is replaced by quest_disable_init_hook (QUEST_RANDOM exempted
when rand_quest allows); copy the god table from deity_info_init with MAX_GODS_INIT; zero
the spell and school tables; raise the HOOK_INIT_GAME "begin" hook; and finally parse
misc.txt via process_dungeon_file (the capacity and flag directives) and zero the effects.
`init_basic` SHALL pre-allocate the three macro arrays plus the macro action buffer,
cli_info, and the scansubdir result table.

- **Anchors**: `src/init2.c:5858-5887` (basic and hooks), `:5893-5967` (misc)

### Requirement: Towns Wilderness And Growth

`init_towns` SHALL: allocate the town table of max_towns entries, mark 1 through
max_real_towns as TOWN_REAL, and give each town max_st_idx store slots (st_idx set to the
slot index, stock_size zero). `create_stores_stock` SHALL restock lazily — the stocked flag guards
re-entry, the stock is sized from the st_info max_obj. `init_wilderness` SHALL allocate
wild_map row-pointer style (max_wild_y x max_wild_x) and clear generate_encounter. The
growth helpers SHALL: reinit_powers_type (growing the p_ptr->powers bitmap along with the
table), reinit_quests, and reinit_gods all grow by "copy the old table into a new one and
swap the pointer" (comments state realloc is unreliable); init_spells/init_schools/
init_corruptions allocate on first use and record their max_*.

- **Anchors**: `src/init2.c:5973-6049` (towns and wilderness), `:6005-6023` (lazy restock), `:6055-6136` (growth)

### Requirement: Runtime Arrays And Option Defaults

`init_other` SHALL: allocate the three special-allowance bitmaps (m/k/a_allow_special,
sized by max_r/k/a_idx), the vinfo line-of-sight precomputation table, the three entity
lists o_list/m_list/km_list, max_dlv per-dungeon deepest records, the two-dimensional
special_lvl special-level markers, and the cave row array; pre-record the ten feeling
quarks (cursed/broken/average/good/excellent/worthless/special/terrible/uncursed/on sale);
scan option_info writing each default into option_flag and registering each entry in
option_mask; register window_mask bits for the named positions in window_flag_desc;
register the three level generators (dungeon/maze/life, each with the stairs/monsters/
objects/miscs switches all true); and pre-heat the format() buffer with a warm-up call.

- **Anchors**: `src/init2.c:6141-6263`

### Requirement: Allocation Tables

`init_alloc` SHALL build the two allocation tables: the object table expands each
locale/chance pair of k_info (up to four pairs) — prob1/2/3 all set to 100/chance — and
locates entries by prefix sums over depth groups (a missing zero-depth entry quits with
"No town objects!"); the monster table expands r_info by rarity (prob = 100/rarity, a
missing zero-depth entry quits with "No town monsters!"). Both tables share one shape
(alloc_entry: index/level/prob1/prob2/prob3) for the runtime get_obj_num/get_mon_num draws.

- **Anchors**: `src/init2.c:6270-6460`

### Requirement: Set Backfill And Guardian Marks

`init_sets_aux` SHALL set every artifact's set field to -1 and then backfill the set
number from the set_info member tables. `init_guardians` SHALL scan d_info: dungeons with
a final_guardian get RF9_SPECIAL_GENE set on the guardian monster, and final_artifact and
final_object get TR4_SPECIAL_GENE; when a dungeon has neither, its guardian gets
RF7_DROP_RANDART (making it drop a random artifact instead).

- **Anchors**: `src/init2.c:6463-6520`

### Requirement: Main Entry Load Order

`init_angband` SHALL run in order: init_basic; select_module; the news file picked by
time(NULL)%2 as news.txt or news2.txt, verified and displayed (display_message parses the
color codes); scores.raw verified or created (failure runs init_angband_aux reporting a
broken lib directory and exits); zsock_init; init_misc; wipe_hooks; init_lua and
init_lua_init; then the fixed word-list load order — s, ab, al, player, f, k, a, set (with
init_sets_aux), e, ra, r, re, d (with init_guardians), ba, ow, st, wf, wilderness, towns,
t, other, alloc; build_prob builds the random artifact name pool; the four pref files load
(pref.prf, pref-SYS.prf, user.prf, user-SYS.prf); tome_dofile_anywhere loads auto.lua and
automat.atm; HOOK_INIT_GAME "end" fires.

- **Anchors**: `src/init2.c:6596-6891`
