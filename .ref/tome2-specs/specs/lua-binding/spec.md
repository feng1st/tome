# lua-binding Specification

## Purpose

Lua binding declaration family: the nine tolua interface declaration files `src/*.pkg` —
they define the constants, structs, variables, and functions exposed to Lua, from which
tolua generates the C sources at build time (the `w_*.c` files per the makefiles). This capability
registers the binding surface: the Lua-visible names and their C entity mappings. The
interpreter implementation (`src/script.c`) is covered in specs/lua-engine/spec.md and
the Lua-side runtime built on top of this API in specs/lua-core/spec.md; the hand-written
bridge `src/lua_bind.c` is itself checked into the tree.

## Requirements

### Requirement: tolua Declaration Format (Common Contract)

Every `.pkg` file SHALL follow one declaration contract:

- `$#include` pulls in C headers.
- `$static ... {}` embeds C helper function bodies in the generated source.
- Javadoc-style `@def`/`@fn`/`@struct`/`@var`/`@structvar`/`@param`/`@brief`/`@return`/`@note`/`@name`/`@{`/`@}`/`@dgonly` comment blocks are exported as documentation along with the generated code.
- typedef blocks map `cptr`/`errr`/`bool`/`byte`/`s16b`/`u16b`/`s32b`/`u32b` to the Lua types String/Number/Boolean.
- `#define` constants pass through to Lua by name.
- `extern` variables support `c_name@lua_name` aliases (the @-suffix is the Lua-visible name) and runtime-sized arrays bounded by the `max_*` globals.
- `extern ret name(args)` binds an existing C function; `@dgonly` marks a declaration as enabled only in the DarkGod build.
- When an array has an unusual shape, an embedded `$static` address-taking helper function wraps it instead.

- **Anchors**: `src/dungeon.pkg:1-48` (header and typedefs)

### Requirement: dungeon.pkg Binding Surface

`src/dungeon.pkg` SHALL expose the following binding surface.

Constants:

- `CAVE_*`: fifteen grid flags, `0x0001` through `0x4000` (`MARK`/`GLOW`/`ICKY`/`ROOM`/`SEEN`/`VIEW`/`TEMP`/`WALL`/`TRDT`/`IDNT`/`SPEC`/`FREE`/`DETECT`/`PLIT`/`MLIT`).
- `FEAT_*`: the full feature table `0x00`-`0xCD` (with comments marking the unused stretches, `BETWEEN` at `0xA0`, `ALTAR` at `0xA1`-`0xAB`, `EKKAIA` at `0xB6`, `TOWN` at `0xCB`).
- `DF1_*` (32 bits) and `DF2_*` (17 bits) dungeon flag sets.
- `MAX_HGT` 66 / `MAX_WID` 198.
- `TOWN_RANDOM` 20 / `TOWN_DUNGEON` 4 / `TOWN_CHANCE` 50.
- `TERRAIN_*`: the twelve wilderness terrains plus `MAX_WILD_TERRAIN` 18.

Aliases:

- Lua `level_flags1`/`level_flags2` map to C `dungeon_flags1`/`dungeon_flags2`.
- Lua `current_dungeon_idx` maps to C `dungeon_type`.

Structs: `border_type` (the north/south/east/west edges and the four corners),
`wilderness_type_info` (name/text/entrance/road/level/flags1/feat/terrain_idx/terrain[18]),
`wilderness_map` (feat/seed/entrance/known), `town_type`
(name/seed/store[max_st_idx]/numstores/flags/stocked/destroyed), `rule_type`
(mode/percent/mflags1-9/r_char[5]), `obj_theme` (four components), and
`dungeon_info_type` with all fields (three floor/fill pairs each carrying two percents,
`fill_method`, mindepth/maxdepth/principal/next/min_plev, two allocation counts, two
flag groups, size_x/y, `rule_percents[100]` and `rules[5]`, final_object/final_artifact/
final_guardian, the ix/iy/ox/oy in/out doors, `objs`, `d_dice`/`d_side`/`d_frequency`/
`d_type` each `[4]`, and `t_idx`/`t_level` each `[TOWN_DUNGEON]` plus `t_num`).

Variables: `max_towns`/`town_info`, `max_d_idx`/`d_info`/`d_name`/`d_text`,
`max_wild_x`/`max_wild_y`, `max_wf_idx`/`wf_info`/`wf_name`/`wf_text`, `DUNGEON_DEATH`.

Functions:

- `wild_map(y,x)` goes through the embedded `lua_get_wild_map` and returns a `wilderness_map*`.
- `place_trap`, `place_floor`, `place_filler`, `add_scripted_generator` (`@dgonly`), and `new_player_spot` (searches up to five thousand grids for a naked grid that is not anti-teleport, creating stairs or ways in/out when needed) are each bound.
- The `levels.c` family: `get_level_desc`/`get_level_flags`/`get_dungeon_name`/`get_dungeon_special`/`get_command` (`'A'` parent branch, `'B'` branch, `'D'` description, `'L'` parent level, `'N'` name, `'S'` savefile extension, `'U'` map name)/`get_branch`/`get_fbranch`/`get_flevel`/`get_dungeon_save`.

- **Anchors**: `src/dungeon.pkg:50-129` (CAVE), `:131-492` (FEAT), `:494-710` (DF flags and aliases), `:712-812` (sizes, towns, terrain), `:814-1445` (structs and variables), `:1447-1462` (wild_map), `:1464-1618` (functions), `src/generate.c:712-743` (new_player_spot implementation, 5000 attempts)

### Requirement: object.pkg Binding Surface

`src/object.pkg` SHALL expose the following binding surface.

Format increments:

- The global singletons `obj_forge` (an `object_type`) and `theme_forge` (an `obj_theme`) are exposed for Lua item creation through embedded `$static` accessors.
- Bindings may carry default argument values (the six out-parameters of `object_flags` default to `= 0`).
- Macros are exposed through embedded wrapper functions (`is_artifact`/`is_aware`/`is_known`/`set_aware`/`set_known`, five in total).
- A string table can be exported with a fake size (`sense_desc[1000]`; the comment itself calls it a tolua hack).

- **Quirk:** `sense_desc[1000]` is declared with size 1000 only so tolua accepts the export; the real table is smaller.

Constants:

- `TR1`-`TR5`: all five object flag groups, plus the fourteen `ESP_*` bits.
- `USE_*`: four bits (`get_item` channels).
- `INVEN_*`: the slot table (`WIELD` 24 for three weapons, `BOW` 27, `RING` 28 for six rings, `NECK` 34, `LITE` 36, `BODY`/`OUTER`/`ARM` for three arms/`HEAD` for two heads/`HANDS` for three hands/`FEET` for two feet, `CARRY` 49, `AMMO` 50, `TOOL` 51, `TOTAL` 52).
- `TV_*`: the full tval table (including the `TV_PARCHEMENT` compatibility alias).
- `SV_*`: the full sval families (ammo, musical instruments, trapkits, boomerangs, bows, digging tools, hafted weapons, axes, polearms, swords, shields, helms, boots, robes, gloves, soft armor, hard armor, dragon armor, light sources, amulets, rings, wands and staves, rod tips and rod bodies (the body's sval carries the max mana capacity), scrolls, potions and the potion2 family, food, batteries, the four corpse states, and the spell-book families).
- `IDENT_*`: seven bits.
- `OBJ_FOUND_*`: nine origin classes.
- `SENSE_*`: eleven pseudo-id grades.

Structs: `object_kind` with all fields (four allocation groups, btval/bsval, the artifact
bits, power); `artifact_type` (cur_num/max_num/rarity); `ego_item_type` (before, six
tval/sval range groups, `rar[5]` with the five flag/esp/fego groups, mrarity);
`object_type` with all fields (three pval slots, elevel/exp, name1/name2/name2b,
xtra1/xtra2, art_flags five groups plus esp, next_o_idx/held_m_idx, sense, found plus
aux1/aux2 — note the tolua declaration shows no aux3/aux4).

Variables: `o_list`/`k_info`/`k_name`/`k_text`, `a_info`/`a_name`/`a_text`,
`e_head`/`e_info`/`e_name`/`e_text`.

Functions:

- Object core: `m_bonus`/`wield_slot_ideal`/`wield_slot`/`object_flags`/`lua_object_desc@object_desc`/`object_out_desc`/`object_wipe`/`object_prep`/`object_copy`/`lookup_kind`/`get_object`/`new_object`/`end_object`/`get_slot`/`is_blessed`/`calc_total_weight`.
- Pack and floor items: `inven_item_describe`/`inven_item_increase`/`inven_item_optimize`, the three `floor_item_*` functions, `delete_object_idx`, `o_pop`, `inven_carry_okay`, `inven_carry`, `object_pickup`.
- Rolls and generation: `get_obj_num_prep`/`get_obj_num`/`apply_magic`/`make_object` with theme/`drop_near`.
- Identification and valuation: `ident_all`/`identify_pack_fully`/`value_check_aux1` and `value_check_aux1_magic`/`value_check_aux2` and `value_check_aux2_magic`/`select_sense`/`psychometry`/`remove_curse_object`.
- Selection: `get_item@get_item_aux`/`lua_set_item_tester`/`is_magestaff`.

- **Anchors**: `src/object.pkg:19-23` (singletons), `:25-217` (flags and slots), `:219-852` (TV/SV and IDENT/OBJ_FOUND), `:1085-1096` (SENSE), `:867-1083` (four structs), `:1098-1147` (variables and functions), `:1149-1171` (wrappers and valuation)

### Requirement: monster.pkg Binding Surface

`src/monster.pkg` SHALL expose the following binding surface.

Constants:

- `MSTATUS_*`: seven grades, `ENEMY` -2 through `COMPANION` 4.
- `RF1`-`RF9`: all nine monster flag groups (RF1 generation and drops, RF2 behavior including the reserved `BRAIN_1`-`BRAIN_8`, RF3 kinship and resistances, RF4 powers and breaths, RF5 balls/bolts and player-state spells, RF6 specials and summons, RF7 movement, ecology and AI, RF8 dungeon/wilderness habitat markers, RF9 corpse and generation limits).
- `MFLAG_*`: ten runtime state bits.
- `SUMMON_*`: 38 summon types (values 11 through 58, sparse — 19, 20 and 23-30 are unused — including `SUMMON_LUA` 58 for scripted summons).

Structs: `monster_blow` (a four-tuple); `monster_race` with all fields (the nine flag
groups, `blow[4]`, `body_parts[BODY_MAX]`, max_num/cur_num, the full `r_*` lore group,
`on_saved`, `total_visible`, the drops theme); `monster_type` with all fields (r_idx/ego,
`blow[4]`, the five energy/status fields, mflag, smart, status/target/possessor).

Variables: `m_list[max_m_idx]`, `m_max`, `summon_specific_level`, `summon_kin_type`, and
the `monster_forge` singleton.

Functions:

- `monster(m_idx)` goes through an embedded address-taking helper.
- `race_info_idx` (the ego race is folded into the returned mixed race).
- List functions: `delete_monster_idx`/`m_pop`.
- Roll functions: `get_mon_num_prep` (its documentation lists all 29 hook function names that can be assigned to `get_mon_num_hook`)/`get_mon_num`.
- Description functions: `lua_monster_desc@monster_desc` (a static wrapper returning a buffer, mode bits `0x01`-`0x80` fully documented) and `monster_race_desc` bound twice (a String-returning wrapper and a buffer-filling void version coexist under the same name).
- Placement functions: `monster_carry`/`place_monster_aux`/`place_monster`/`place_monster_one`.
- Allegiance functions: `is_friend`/`is_enemy`/`change_side`.
- Summoning functions: `find_position` (default arguments `=0`)/`summon_specific`/`summon_specific_friendly`/`lua_summon_monster@summon_monster_aux` (the `fct` argument is a Lua callback name).
- The rest: `can_create_companion`/`monster_set_level`/`do_control_reconnect`.

- **Anchors**: `src/monster.pkg:44-48` (singleton), `:50-1096` (MSTATUS, RF1-9, MFLAG), `:1098-1594` (three structs), `:1596-1610` (monster and m_list), `:1612-2106` (functions and hook list), `:2108-2307` (SUMMON table), `:2309-2324` (reconnect and m_max)

### Requirement: player.pkg Binding Surface

`src/player.pkg` SHALL expose the following binding surface.

Constants:

- `PY_MAX_LEVEL` 50 and the `player_exp` table.
- `A_STR`-`A_CHR` and `SUBRACE_SAVE` 9.
- `SEX_*`: three grades.
- `PR1_*` race flags, roughly two dozen bits, plus `PR2_ASTRAL`.
- The `PN_*`/`PU_*`/`PR_*`/`PW_*` four dispatch groups.

- **Quirk:** `PR_MH` is defined twice in this file.

- `BODY_*`: six body-part kinds plus `BODY_MAX`.
- `SPELLBINDER_HP75`/`SPELLBINDER_HP50`/`SPELLBINDER_HP25`: three grades.
- `SHIELD_NONE`/`SHIELD_COUNTER`/`SHIELD_FIRE`/`SHIELD_GREAT_FIRE`/`SHIELD_FEAR`: five shield options.
- `PY_FOOD_*`: six food thresholds.
- `GOD_ALL`/`GOD_NONE`/`GOD_ERU`/`GOD_MANWE`/`GOD_TULKAS`/`GOD_MELKOR`/`GOD_YAVANNA`: seven numbers.
- `PWR_*`: sixty-two power numbers (0-61).
- `WINNER_NORMAL`/`WINNER_ULTRA`.

Structs: `deity_type` (single field); `player_type` with all fields (identity and
location, the `inventory@inventory_real` alias, experience/level/wilderness landmarks,
the hp/mana/sanity triples, grace/pgod/praying, the six stat arrays and three luck
values, all timed states and composite parameter slots, the immunity/resistance/sustain
boolean families, derived bonuses and the skill group, the two antimagic values,
control/control_dir, the four spellbinder slots, the `corruptions@corruptions_aux`
alias, astral/leaving); `player_race` (title/desc/infra); `player_race_mod` with all
fields (six `r_adj` entries, luck/mana, eight skill modifiers, r_mhp/r_exp, base/mod age
height and weight for each sex, choice/pclass/mclass, `powers[4]`, body_parts, two flag
groups, `oflags1-5`/`oesp`/`opval` each `[51]` as per-level flags, `g` graphics, and
`skill_base`/`skill_mod` plus the `m` variants each `[MAX_SKILLS]`).

Variables: `energy_use`, the `p_ptr@player` alias, `max_rp_idx`/`race_info`/`rp_name`/
`rp_text`, `max_rmp_idx`/`race_mod_info`/`rmp_name`/`rmp_text`, `class_info`/`c_name`/
`c_text`, `flush_failure`, `dun_level`, `wizard`/`total_winner`/`has_won`/
`joke_monsters`, `max_dlv[999999]` (fake-size hack).

Functions:

- Fifty-eight `set_*` timed-state functions redeclared in full (including `set_grace`, `set_mimic`, `set_fast(v,p)`, the five-parameter `set_shield`, `set_tim_thunder`, `set_tim_breath`, `set_invis(v,p)`, `set_tim_regen(v,p)`).
- `apply_flags` (the last five parameters default to 0; the comments explain the flags one by one).
- Experience: `check_experience`/`check_experience_obj`/`gain_exp`/`lose_exp`.
- Deity: `inc_piety`/`abandon_god`/`wisdom_scale`/`follow_god`/`add_new_gods`/`desc_god`.
- Misc: `no_lite`/`do_cmd_throw`/`change_wild_mode`/`switch_class`/`switch_subclass`/`switch_subrace`/`get_subrace_title`/`set_subrace_title`/`do_rebirth`.
- `test_race_flags` through an embedded wrapper that picks `PRACE_FLAG` or `PRACE_FLAG2` by `slot` = 1 or 2.

- **Anchors**: `src/player.pkg:43-102` (level/stats/sex), `:104-225` (PR1/PR2 and PN), `:228-292` (PU), `:295-476` (PR and PW), `:478-515` (gods and body), `:518-1821` (player_type fields), `:1823-1836` (spellbinder thresholds), `:1839-2103` (race and race_mod structs), `:2105-2184` (variables), `:2186-2950` (set_* redeclarations and SHIELD/PY_FOOD), `:2981-3120` (experience and deity), `:3136-3390` (PWR table), `:3392-3525` (misc functions and winner variables)

### Requirement: player_c.pkg Binding Surface

`src/player_c.pkg` SHALL expose the following binding surface.

Structs:

- `player_class` with all fields (`titles[10]`, six `c_adj` entries, eight `c_` skill modifiers and eight `x_` bonuses, `c_mhp`/`c_exp`, `powers[4]`, the seven spell slots plus `max_spell_level`/`magic_max_spell`, `flags1`, mana, `blow_num`/`wgt`/`mul`/`extra_blows`, five pseudo-id sense parameters).
- `skill_type` with all fields (`action_desc`/`action_mkey`, `i_value`/`i_mod` and `value`/`mod`/`rate`, `uses`, `action[9999]` (fake size), `father`/`dev`/`order`/`hidden`).
- `ability_type` (`action_mkey`/`cost`/`acquired`).

Constants:

- `MAX_SKILLS` 100.
- The 58 `SKILL_*` numbers (values 1 through 59 with 38 skipped) plus `SKILL_MAX` 50000 and `SKILL_STEP` 1000.
- `AB_*`: eleven ability numbers (`SPREAD_BLOWS` 0 through `UNDEAD_FORM` 10).

Variables: `cp_ptr`, `old_max_s_idx`/`max_s_idx`, `s_info[MAX_SKILLS]`, `max_ab_idx`/
`ab_info`.

Functions:

- `get_skill_name` (embedded wrapper).
- `get_skill`/`get_skill_scale`/`do_get_new_skill`/`get_melee_skills`/`find_skill`/`find_skill_i`.
- The three embedded wrappers `get_class_name`/`get_race_name`/`get_subrace_name`.
- `find_ability`/`do_cmd_ability`/`has_ability`.

- **Anchors**: `src/player_c.pkg:43-267` (class struct and cp_ptr), `:271-389` (skill struct and s_info), `:391-817` (SKILL constant table), `:819-905` (skill functions and name getters), `:907-1059` (ability struct, functions, and AB table)

### Requirement: quest.pkg Binding Surface

`src/quest.pkg` SHALL expose the following binding surface.

Constants: `QUEST_STATUS_*` eight grades, `IGNORED` -1 through `FAILED_DONE` 6.

Structs: `quest_type` (silent/dynamic_desc/status/level/type).

Variables: `max_q_idx` and the `quest@quest_aux` alias.

Functions:

- `quest(q_idx)` goes through an embedded address-taking helper.
- `add_new_quest@new_quest` (`@dgonly`).
- `desc_quest@quest_desc` (`@dgonly`).
- `lua_get_new_bounty_monster@get_new_bounty_monster`.

- **Anchors**: `src/quest.pkg:50-77` (status flags), `:79-136` (struct, variables, and the quest getter), `:138-170` (three functions)

### Requirement: spells.pkg Binding Surface

`src/spells.pkg` SHALL expose the following binding surface.

Format increments:

- A `typedef mcptr` (a mutable string).
- `struct c_name@lua_name` type aliases (`spell_type@school_spell_type`).
- `extern` functions support aliases (`project_hack@project_los`, `grab_spell_type@spell`, `grab_school_type@school`, among others).
- Out-parameter defaults `=0` and `FILE*=NULL`.

Constants:

- `DEFAULT_RADIUS` 25.
- `GF_*`: the full table 1-110 plus `MAX_GF` 111 (base elements, terrain shaping, the `OLD_` state spells, `AWAY`/`TURN`, the six `DISP` families, holy and hell fire, mind control, and the new Z-series classes).
- `PROJECT_*`: seventeen bits (`JUMP`/`BEAM`/`THRU`/`STOP`/`GRID`/`ITEM`/`KILL`/`HIDE`/`VIEWABLE`/`METEOR_SHOWER`/`BLAST`/`PANEL`/`ALL`/`WALL`/`MANA_PATH`/`ABSORB_MANA`/`STAY`).
- `EFF_WAVE`/`EFF_LAST`/`EFF_STORM` plus `EFF_DIR1`-`EFF_DIR4` and `EFF_DIR6`-`EFF_DIR9` (eight direction bits; `EFF_DIR5` is absent).

Structs: `magic_power` (min_lev/mana_cost/fail/name/desc), `spell_type` (name/
skill_level/mana/mana_max/fail/level), `school_type` (name/skill).

Variables: `project_time`, `hack_no_detect_message`, `power_max`,
`last_teleportation_y`/`last_teleportation_x`.

Functions:

- Projection core: `project` (its comments carry the full bolt/beam/ball semantics and the `gm[]` ring convention) and `project_hook`/`project_hack@project_los`/`project_meteor`.
- Teleport family: `teleport_player_directed`/`teleport_away`/`teleport_player`/`teleport_player_to`/`teleport_monster_to`/`teleport_monster`/`teleport_player_level` and `fetch`/`recall_player`/`get_pos_player`/`swap_position`/`teleport_swap`/`passwall`/`create_between_gate`/`alter_reality`.
- Damage and states: `take_hit`/`take_sanity_hit`/`corrupt_player`/`hp_player`/`heal_insanity`/`do_dec_stat`/`do_res_stat`/`do_inc_stat`.
- Growth and terrain: `grow_things`/`grow_grass`/`grow_trees`/`warding_glyph`/`explosive_rune`/`wall_stone`/`stair_creation`/`map_area`/`wiz_lite`/`wiz_lite_extra`/`wiz_dark`/`lite_room`/`unlite_room`/`lite_area`/`unlite_area`.
- Identification and charging: `identify_pack`/`ident_spell`/`identify_fully`/`remove_curse`/`remove_all_curse`/`restore_level`/`lose_all_info`/`self_knowledge`/`recharge`/`alchemy`.
- Detection family: the fifteen `detect_*` functions.
- Mass spells: `aggravate_monsters`/`genocide_aux`/`genocide`/`mass_genocide`/`probing`/`banish_evil`/the six `dispel_*` functions/`turn_undead`.
- Adjacency functions: `door_creation`/`trap_creation`/`glyph_creation`/`destroy_doors_touch`/`destroy_traps_touch`/`wipe`/`destroy_area`/`earthquake`.
- Firing family: `fire_ball`/`fire_bolt`/`fire_beam`/`fire_bolt_or_beam`/`fire_ball_beam`/`fire_cloud`/`fire_wall`/`fire_wave`/`fire_druid_ball`/`fire_druid_bolt`/`fire_druid_beam` (the druid trio noted deprecated in the source).
- Creation: `create_artifact`/`make_wish`/`reset_recall`.
- Direction: `get_aim_dir`/`get_rep_dir`/`tgt_pt`.
- The `@dgonly` magic-power system, five functions: `new_magic_power`/`grab_magic_power@get_magic_power`/`get_magic_power_lua@select_magic_power`/`lua_spell_success@magic_power_sucess`/`add_new_power`.
- The `@dgonly` school system, seven functions: `new_school`/`new_spell`/`grab_spell_type@spell`/`grab_school_type@school`/`lua_get_level`/`lua_spell_chance`/`lua_spell_device_chance`, plus `get_school_spell` (not `@dgonly`-marked).

- **Anchors**: `src/spells.pkg:56-345` (GF table), `:347-438` (PROJECT and project_time), `:440-604` (teleports and take_hit), `:606-746` (project and corrupt), `:748-1140` (growth/stats/identification/detection families), `:1142-1253` (tgt_pt/wall_stone/create_artifact/wall_to_mud/ident/recharge/aggravate), `:1255-1420` (genocide family and adjacency), `:1421-1651` (destroy/earthquake and light and firing family and EFF), `:1653-2102` (fire_* family and misc), `:2104-2256` (magic_power system), `:2258-2465` (school system), `:2467-2498` (last_teleportation and get_pos_player)

### Requirement: z_pack.pkg Binding Surface

`src/z_pack.pkg` SHALL expose the following binding surface.

Constants: the fifteen `TERM_XTRA_*` terminal actions (`EVENT` through `SCANSUBDIR`);
`ZSOCK_TYPE_TCP` and `ZSOCK_TIMER_DELAY` 100.

Variables: `Term_xtra_long`, `scansubdir_dir[1024]`/`scansubdir_max`/
`scansubdir_result[scansubdir_max]`, `Rand_quick`/`Rand_value`, and the `zsock`
singleton.

Functions:

- Terminal functions: `Term_xtra`/`set_cursor`/`gotoxy`/`putch`/`putstr`/`clear`/`redraw`/`redraw_section`/`get_size`.
- Random functions through embedded wrappers: `rand_int`/`rand_range`/`rand_spread`/`randint`/`magik` (the formulas are written into the comments) and `damroll`/`maxroll`.
- Sockets: the `ip_connection` struct (setup/conn_ip/conn_port/conn_type/connected/socket/server) and the `zsock_hooks` hook table (`new`/`free_connection`, `setup`/`unsetup`, `open`/`close`, `write`, `write_simple`, `read_simple`, `accept`, `can_read`, `wait`, `add_timer`/`remove_timer`).

- **Quirk:** the `read` entry is currently commented out of service in the hook table; it needs a dedicated wrapper in `script.c` (source note says DG). `script.c` registers such a wrapper — see specs/lua-engine/spec.md.

- **Anchors**: `src/z_pack.pkg:43-146` (TERM_XTRA and Term_xtra), `:148-272` (terminal functions), `:274-398` (random functions), `:404-454` (ip_connection), `:456-688` (zsock_hooks), `:690-693` (zsock)

### Requirement: util.pkg Binding Surface

`src/util.pkg` SHALL expose the following binding surface.

Constants:

- `TRUE`/`FALSE` (empty macros) and `ESCAPE`.
- `TERM_*`: sixteen colors (with an RGB note).
- The full `HOOK_*` table 0-77 (`MONSTER_DEATH` 0 through `DEBUG_COMMAND` 76, plus `HOOK_CALC_BONUS_END` 77); each hook documents its parameters and return semantics, and `HOOK_GF_EXEC` 59 is one number with five documentation sections covering the grid/object/angry/monster/player forms.
- `FF1_*`: nineteen terrain flags.

Structs: `cave_type` with all fields (info/feat/o_idx/m_idx/t_idx/special/special2/
inscription/mana/mimic/effect); `timer_type` (next/enabled/delay/countdown/callback,
where the callback is a Lua function name); `list_type` (an empty shell).

Variables: `turn`/`old_turn`, `cur_wid`/`cur_hgt`, `inkey_scan`, `ANGBAND_SYS`/
`ANGBAND_KEYBOARD`/`ANGBAND_GRAF`, the eighteen `ANGBAND_DIR_*` directories plus the
base `ANGBAND_DIR` (including `CORE`/`DNGN`/`SCPT`/`MODULES`/`PATCH`), the `m_allow_special`/`k_allow_special`/
`a_allow_special` trio, `target_who`/`target_col`/`target_row`, `max_bact`, `ddd`/`ddx`/
`ddy` and the two optimization tables, `option_ingame_help`, `game_module`, and the
twenty-one `adj_*` stat tables.

Functions:

- Input and display: `disturb`/`bst`/`path_build_lua@path_build`/`move_cursor`/`flush`/`inkey` (the comments carry the full semantics of `inkey_xtra`/`inkey_scan`/`inkey_base`/`inkey_flag` and the special keys ascii 29-31)/`cmsg_print`/`msg_print`/`screen_save`/`screen_load`/`Term_save`/`Term_load`/`c_put_str`/`c_prt`/`prt`/`message_add`/`display_message`/`clear_from`/`askfor_aux`/`get_string`/`get_check`/`get_com_lua@get_com`/`get_quantity`/`get_count`/`repeat_push`/`repeat_pull`/`repeat_check`/`get_keymap_dir`.
- Query functions: `test_monster_name`/`test_item_name`/`luck`/`get_player_race_name`/`value_scale`/`rescale`/`change_option`.
- Text and files: `text_out`/`text_out_c`/`show_file`/`file_character`/`quark_str`/`quark_add`.
- Hooks and scripting: `dump_hooks`/`add_hook_script`/`del_hook_name`/`process_hooks_restart`/`tome_dofile`/`tome_dofile_anywhere` (with `test_exist=TRUE`)/`exec_lua`/`string_exec_lua`/`dump_lua_stack`/`lua_print_hook@print_hook`.
- Savefiles: `register_savefile`/`save_number_key`.
- Cave: `los`/`lua_cave_is@cave_is`/`lua_get_cave@cave`/`cave_set_feat`/`note_spot`/`lite_spot`/`set_target`/`get_target`/`show_file`.
- Generation: `get_map_size`/`load_map`/`alloc_room` (all with default arguments).
- UI: `lua_input_box@input_box`/`lua_msg_box@msg_box`.
- Temp files: the four `@lua_*` functions.
- Modules: `module_reset_dir`/`scansubdir`/`file_exist`.
- Timers: `new_timer`/`del_timer`.
- Lists: the four `@dgonly` functions `create_list`/`delete_list`/`add_to_list`/`display_list`.
- `calc_bonuses` exposed directly.

- **Anchors**: `src/util.pkg:44-121` (macros and colors), `:123-1030` (full HOOK table), `:1033-1099` (variables and disturb/bst/path_build), `:1101-1460` (input/display and query functions), `:1463-1654` (text/hooks/scripting and savefiles), `:1669-1794` (stat tables), `:1796-1898` (repeat and FF1), `:1901-1969` (cave_type), `:1971-2145` (ANGBAND directory family), `:2147-2415` (cave/target/generation), `:2417-2541` (UI/temp files/quark), `:2543-2593` (modules and key tables), `:2595-2735` (timers/lists/tail)
