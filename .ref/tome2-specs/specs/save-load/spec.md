# save-load Specification

## Purpose

Savefile reading and writing: `src/loadsave.c` carries the whole savefile format — the
byte-layer serialization helpers (the do_* family and the version-gated variants), the full
player state (do_extra), objects/monsters/memory/stores/dungeons (do_item/do_monster/
do_lore/do_xtra/do_store/do_dungeon/do_grid), the main flows (save_player/load_player/
do_savefile_aux), and the Lua extension slots. BZ_SAVES compressed savefiles are a
compile-time option and undefined by default.

## Requirements

### Requirement: Byte-Layer Serialization Helpers

`sf_get`/`sf_put` SHALL be the only byte gates (under BZ_SAVES they go through the bz2
stream instead; errors note and exit). do_byte/do_u16b/do_s16b/do_u32b/do_s32b SHALL read
and write little-endian byte by byte (a flag other than LS_LOAD/LS_SAVE prints FATAL and
exits); do_string SHALL write a NUL-terminated variable-length string and truncate to max
on read, forcing termination. The version-gated helpers SHALL: do_ver_* (use the default
and skip the read when vernum < version) and skip_ver_* (read and discard when
vernum <= version). `my_sentinel` SHALL write a check value at fixed spots (a mismatch on
read reports "Savefile broken <location>" for troubleshooting); `note` scrolls the loading
progress over screen rows 2-23.

- **Anchors**: `src/loadsave.c:87-126` (sf gates), `:1468-1604` (do_*), `:1613-1737` (ver helpers), `:3918-3940` (sentinel), `:1744-1756` (note)

### Requirement: Object Serialization

`do_item` SHALL store k_idx/iy/ix/tval/sval/pval/pval2/pval3/discount/number/weight/name1/
name2/name2b/timeout/to_h/to_d/to_a/ac/dd/ds/ident/marked/art_flags1-5 plus art_esp plus
the five obvious mirror groups/held_m_idx/xtra1/xtra2/elevel/exp/sense/found and aux1-4,
then store the inscription and random artifact name as strings (quark refill on read).
The read-side repair SHALL: refill tval/sval from k_info (TV_RANDART exempt from the sval
refill); for non-wearable items (wearable_p tests thirty-odd tvals) reset the whole field
group from k_info and return; zero invalid name1/name2 (no such a_info/e_info entry);
refill ac/dd/ds from k_info and then from a_info for artifacts, while egos and random
artifacts restore the dd/ds as read.

- **Anchors**: `src/loadsave.c:1762-1809` (wearable_p), `:1818-2053`

### Requirement: Monsters Memory And Stores

`do_monster` SHALL store r_idx/ego/coordinates/HP/sleep/mspeed/energy/the three states/
smart/status/possessor/speed/level/ac/exp/target/bleeding/poisoned/mflag (masked by
PERM_MFLAG_MASK on read), the four attacks, the mind pointer presence bit (a true bit is
MAKEd on read), and the sr_ptr presence bit — when true, a whole inline monster_race is
serialized (name/text offsets, dice, ac, speed, mexp, weight, freq, flags1-9, the four
attacks, body parts, level/rarity, graphics, max_num/cur_num). `do_lore` SHALL store the
monster memory fields (sights/deaths/pkills/tkills/wake/ignore/xtra1-2/the two drop
counts/the two cast counts/the four blow slots/flags1-9/max_num/on_saved), with the read
side ANDing r_flags1-6 with the live flags as repair. `do_xtra` SHALL store the k_info
sense bitmaps (aware/tried/know/artifact, four bits in one byte). `do_store` SHALL store
store_open/insult_cur/owner/stock_num/good_buy/bad_buy/last_visit plus each stock item
(discarded on read beyond STORE_INVEN_MAX or stock_size).

- **Anchors**: `src/loadsave.c:2061-2182` (monster), `:2191-2249` (memory), `:175-197` (xtra), `:2257-2307` (store)

### Requirement: Full Player State

`do_extra` SHALL store, in order: player_name/died_from/the four history rows;
special_lvl as the dungeon-depth matrix (dimensions self-described) with
generate_special_feeling; do_quick_start (the thirteen previous_char scalar fields plus the six
stats plus luck plus chaos_patron plus weapon plus quick_ok plus the four history rows); do_subrace (the
full player_race_mod under the SUBRACE_SAVE index — title/desc/place/stats/luck/mana/the
eight skills/HP/experience/age/body size/infra/the four powers slots/body parts/flags1-2/
PY_MAX_LEVEL+1 groups of six level flags with opval/graphics/four skill entries per
MAX_SKILLS); lives/prace/pracem/pclass/pspec/psex/two discarded u16bs/mimic_form with
level/hitdie/expfact/age/ht/wt; stat_max/cur/cnt/los, six entries times four groups; the
skill points and top skill/melee_style/use_piercing_shots and the five skill fields per
MAX_SKILLS (with the old_max_s_idx truncation for compatibility); abilities acquired; the
two luck values; alchemy knowledge (six u32 artifact bitmaps plus thirty-two u32 ego
bitmaps plus the gained counter — the as-shipped comment notes it occupies the original 24
free bytes); au/exp/lev/town_num; the arena counter/inside_arena/inside_quest/exit_bldg;
the spellbinder (count/trigger/the four spell indexes); the full plots table and the full
random_quests table (type/r_idx/done); oldpx/oldpy; mhp/chp/chp_frac/hp_mod;
msane/csane/csane_frac; msp/csp/csp_frac; four placeholder s16bs (where the tank points
used to be, per the source note at the anchor); the four god fields (grace/praying/
melkor_sacrifice/pgod); max_plv plus the per-dungeon max_dlv table (repaired on read when
max_plv < lev); help.enabled/help1; roughly ninety timed states and status fields
(sc/blind/paralyzed/confused/food/energy/fast/slow/afraid/cut/stun/poisoned/image/protevil/
protundead/invuln/hero/shero/the shield family/the control family/the tim_thunder family/
the tim_project family/the two breathing fields/the tim_roots family/invis/the two recall
fields/infra/the nine oppose fields/tim_esp/wraith/ffall, then tim_fly/tim_poison/
tim_regen/tim_regen_pow under the do_ver_s16b gate, then fire_aura/resist_magic/invisible/inv_pow/
mimic/lightspeed/lite/holy/walk_water/mental_barrier/immov_cntr/strike/meditation/reflect/
res_time/deadly/prob_travel/disrupt_shield/the two parasite fields/the two loan fields/
absorb_soul); chaos_patron; the corruptions table (count self-described);
confusing/black_breath/fate_flag/searching/maximize/preserve/special/ambush_flag/
allow_one_death/xtra_spells; vanilla_town/no_breeds/protgood; the auxiliary u32b block (mimic/antimagic/three druid
fields/two music fields/two necro fields/seven race fields — sixteen u32b in all); body_monster/disembodied/astral; the powers_mod table (count
self-described with POWER_MAX_INIT); the skip_ver_byte(100) placeholder;
tactic/movement/companion_killed/no_mortal; the bounties table with total_bounties;
spell_num with the full random_spells table (do_spells: name/desc/mana/fail/proj_flags/GF/
radius/dice/level/untried); rune_num with the rune_spells table (name/type/rune2/mana);
the three random seeds (dungeon/flavor/town); panic_save/total_winner/has_won/noscore;
death; the module compatibility check (call_lua module_savefile_loadable — a mismatch
reports "Bad game module" and fails); feeling/old_turn/turn.

- **Anchors**: `src/loadsave.c:202-316` (quick start and subrace), `:322-815` (do_extra main chain)

### Requirement: Dungeon Serialization

`do_dungeon` SHALL: run the leading sentinel 324; store dun_level/dungeon_type/num_repro/
py/px/cur_hgt/cur_wid/the two panel counts/dungeon_flags1-2/last_teleportation_y — as
shipped, y is written twice and x never reaches the file; the effects seven-tuples under
the MAX_EFFECTS count; floor_type/fill_type of one hundred entries each (guarding evolving
dungeons); on read, surface levels (dun_level zero and not a quest level) re-run w_info.txt
plus t_info.txt; do_grid nine RLE passes (info/feat/mimic/special/special2/t_idx/
inscription/mana/effect — the byte fields feat/mimic/mana are handled separately, RUN
lengths capped at 255); run compact_objects plus compact_monsters before saving; the object
section — a count (companions' held items deducted when no_companions) plus each item via
do_item (the read side checks the o_pop index and hangs the item on the monster stack by
held_m_idx or on the grid stack by iy/ix); the monster section — a count plus each monster
via do_monster (the read side checks the m_pop index, refills cave m_idx, puts
MFLAG_CONTROL monsters into p_ptr->control, and increments the r_info cur_num); the
companion-keep section km_list (the full table on save when no_companions is false, counted
on read); character_dungeon set true after reading. `save_dungeon`/`load_dungeon` SHALL
read and write the persistent level file under player_base plus an extension (get_dungeon_save
supplies the extension), with the load side restoring dun_level/dungeon_type on failure.

- **Anchors**: `src/loadsave.c:2688-2974` (do_dungeon), `:3655-3916` (both do_grid versions; the BZ version's case 8 is disabled with a parameter mismatch defect), `:818-851` (save_dungeon), `:2977-3032` (load_dungeon)

### Requirement: Savefile Main Flows

`save_player` SHALL: under SAFER_PANICS write the .pnc file directly in panic_save state;
on the normal path write savefile.new (fd_make to create, my_fopen to write, fd_kill on
failure), then rotate on success — savefile moves to .old, .new moves onto savefile, .old
is deleted, character_saved/character_loaded set true, .lok cleared under VERIFY_SAVEFILE,
and save_savefile_names runs at the end. `load_player` SHALL: pass through an empty
savefile name; announce and pass through a missing file; under VERIFY_SAVEFILE refuse when
.lok exists ("Savefile is currently in use."); under SAFER_PANICS prefer the .pnc file when
it exists, setting panicload; probe the version first (a u32b vernum plus one random byte);
read everything through rd_savefile; judge a zero turn as "Broken savefile"; under
VERIFY_TIMESTAMP refuse when sf_when and the file ctime differ by more than a hundred
seconds; let the HOOK_LOAD_END hook revive (returning character_loaded and death); on death
increment sf_lives and zero turn/old_turn; when alive reset died_from to
"(alive and well)" and delete the .pnc file in panicload state.

- **Anchors**: `src/loadsave.c:856-935` (save_aux), `:940-1056` (save_player), `:1098-1460` (load_player), `:3552-3619` (rd_savefile)

### Requirement: Savefile Overall Layout

`do_savefile_aux` SHALL, in order: refuse to load when vernum < 100; on the save side
record sf_when/sf_xtra/sf_saves; the version quadruple plus one extra byte (the read side
reads it again and discards it — matching the load_player probe); sf_xtra/sf_when/sf_lives/
sf_saves; the module name string; the RNG state (Rand_place plus RAND_DEG Rand_state
entries, Rand_quick set false on read); automatizer_enabled; do_options — delay_factor/
hitpoint_warn, the cheat bitmap u16b (wizard 0x0002, cheat_peek/hear/room/xtra/know/live at
0x0100 through 0x2000 in order), the three autosave entries, and the normal and window
options as 8x u32b flag words with masks (the read side applies the intersection of both
masks; the save side refreshes option_flag from the full option_info table); do_messages
(with compress_savefile and over forty messages, exactly forty are kept, stored in reverse
order); monster memory (a count plus do_lore); object memory (a count plus do_xtra) plus
junkinit (the arena/quest reset plus wild_map seed randomization); the town section
(max_towns, an exact TOWN_RANDOM match check, per-town destroyed, with seed/numstores/
flags stored for entries at or beyond TOWN_RANDOM, and create_stores_stock run on read for
real towns); the town-depth binding (the t_idx/t_level matrix plus t_num); the quest section
(status plus data[4], with the init hook called on read for HOOK_TYPE_C); the four
wilderness position fields plus wild_map per-grid seed/entrance/known (with a size check);
the random artifact table (MAX_RANDARTS seven-tuples); the artifact cur_num table; the
fates table (do_fate's eleven fields, clamped to MAX_FATES-1 on read when out of range); the
trap ident table; the inscription know table; do_extra; the player_hp table (PY_MAX_LEVEL);
morejunk (the five pointers sp/rp/rmp/cp/spp re-hung); the three pet settings;
do_inventory (a u16b slot number plus the item, with the 0xFFFF sentinel; equipment slots
stored in place with wield_set recording sets, the pack renumbered, and a full INVEN_PACK
refused); the real-town list plus each town's stores; the extension slots — the
extra_savefile_parts count, the save side raising HOOK_SAVE_GAME, the read side walking the
load_number_key key/value pairs raising HOOK_LOAD_GAME; do_dungeon only when not dead; a
single-byte safety pad at the end of the file.

- **Anchors**: `src/loadsave.c:3071-3546`

### Requirement: Junk Helpers And Extension Slots

The junk helpers SHALL: junkinit reset the arena and quest state and lay wild_map from a
random seed (with the as-shipped redundancy of exit_bldg written twice); morejunk re-hang
the five global pointers sp_ptr/rp_ptr/rmp_ptr/cp_ptr/spp_ptr; do_blocks is a placeholder
block for unhooked lines (sentinel 37). The extension slot helpers SHALL: register_savefile
increments extra_savefile_parts, and save_number_key/load_number_key use the "length byte
plus key string plus u32b value" format for Lua-side custom data.

- **Anchors**: `src/loadsave.c:3625-3652` (junk helpers), `:3034-3049` (do_blocks), `:3942-3977` (extension slots)
