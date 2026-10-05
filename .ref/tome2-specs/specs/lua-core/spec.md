# lua-core Specification

## Purpose

Lua core helpers: the miscellaneous engine bridges under `lib/core/` — building action
registration (`building.lua`), dungeon and direction helpers (`dungeon.lua`), the help
index generator (`gen_idx.lua`), the registries for gods, quests, mimic shapes, powers,
mkeys, savefile variables, the contextual help engine, and the XML module. Language
baseline: the Lua 4 dialect (`%var` upvalues, `getn`/`tinsert`/`tremove`, `not nil` as
truth value, `TRUE`/`FALSE`). The interpreter that loads these files and the tolua
binding surface they call are covered in specs/lua-engine/spec.md and
specs/lua-binding/spec.md; the game scripts that consume them in specs/scpt-misc/spec.md.

## Requirements

### Requirement: Building Action Registration

`lib/core/building.lua` SHALL provide the building action registry:

- The `__building_actions` table holds action closures keyed by index.
- `add_building_action` asserts the `index` and `action` keys.
- `__bact_activate` dispatches by bact number.
- `HOOK_BUILDING_ACTION` is attached with the handler name `"__bact_activate"`.

#### Scenario: Dispatch by bact number

- **WHEN** `HOOK_BUILDING_ACTION` fires with a bact number that has a registered action
- **THEN** `__bact_activate` calls the stored closure for that index

- **Anchors**: `lib/core/building.lua:1-15`

### Requirement: Dungeon And Direction Helpers

`lib/core/dungeon.lua` SHALL provide:

- `place_dungeon` writes the `wild_map` entrance (with a `d_idx` it stores `1000 + d_idx`, without one it clears the entrance to 0).
- `dungeon(d_idx)` returns `d_info[1 + d_idx]`.
- `wild_feat` returns `wf_info[1 + feat]`.
- `explode_dir` returns the `ddy`/`ddx` pair for a direction.
- `rotate_dir` rotates around the eight-way direction ring in both senses.
- `load_map` and `get_map_size` SHALL dispatch to the C side based on the `"#!map"` prefix (inline map text versus file name).
- `place_trap` SHALL stash `dun_level`, substitute the requested level, call the C `place_trap`, and restore `dun_level`.
- The generator registry SHALL work as follows: `level_generator` asserts `name`/`gen` and defaults `stairs`/`monsters`/`objects`/`miscs` to `TRUE`, stores the generator in `__level_generators` and calls `add_scripted_generator`; `level_generate` asserts the name and then calls the stored closure. The file tail carries an example in a comment.

#### Scenario: Inline map dispatch

- **WHEN** `load_map` is called with map text that starts with the `"#!map"` prefix
- **THEN** the call is dispatched to the C side as full map text rather than as a file name

- **Anchors**: `lib/core/dungeon.lua:3-63` (helpers and map dispatch), `:66-92` (trap and generators), `:94-106` (example comment)

### Requirement: Help Index Generator

`lib/core/gen_idx.lua` SHALL generate the help index:

- The `files` table lists 122 `lib/help` document names (the document contents themselves are out of scope here).
- `parse_file` extracts anchors with two regex shapes, the two-part `~~~~~(anchor)|([name])` and the three-part form (with a sub-name), into the `index[name]` table (entries are `__primary__` or carry the sub-name).
- `sort_fct` compares byte by byte; on a tie the shorter name sorts first.
- `generate_index` SHALL run `sort` five times in a row over every group and over the master table (the source comment mocks its own unreliable sorter), then convert to `index_list`.
- `print_index` SHALL write the `index.txt` front page and the letter-jump anchors (`~~~~~c` plus the `*****/..[c]` link line), listing `__primary__` entries first and then sub-entries (indented two and four spaces respectively).
- The file tail executes `generate_index` followed by `print_index` directly.

- **Anchors**: `lib/core/gen_idx.lua:2-128` (file table), `:132-181` (parsing and sorting), `:183-207` (generation), `:209-257` (output), `:259-261` (execution)

### Requirement: God Registration

`lib/core/gods.lua` SHALL provide the god registry:

- The `__gods_hook`/`__gods_callbacks` tables are the registries.
- `add_god` asserts the `name`/`desc`/`hooks` keys; `add_new_gods` creates the god number, then `desc_god` writes each description line in turn.
- Hooks are attached one by one with `add_hook_script` under the `"__lua__gods_callback<N>"` globals (the original functions are hooked directly, with no predicate wrapper — unlike the corruption registration).
- The `data` table entries are pushed with `setglobal` item by item and registered into the savefile with `add_loadsave`.
- `add_god` returns the god number.

- **Anchors**: `lib/core/gods.lua:5-40`

### Requirement: Contextual Help

`lib/core/help.lua` SHALL provide the in-game contextual help engine:

- It maintains its own `__ingame_hooks` table (source note: so as not to overload the C hook processor).
- `ingame_help(t, ...)` with a string `t` dispatches through `call` to the global `"__ingame_help_fct_<t>"`.
- The registration path asserts `desc|fct` and `hook|callback`; the hook form additionally requires `event`.
- Every entry first sets the `"__ingame_help_activated_<N>"` global to `FALSE` and registers it into the savefile with `add_loadsave`.
- Hook form: when the first entry for a hook arrives, one master closure is attached via `add_hooks` (gated on `option_ingame_help`, skipping the `"n"` key, calling each list entry in turn); the desc form plays its lines in yellow one by one when the event hits and then marks the entry activated; the fct form marks the entry activated only when its `call` returns true.
- Callback form: registers the `"__ingame_help_fct_<callback>"` global; the `no_test` flag bypasses the option gate; the desc and fct variants are symmetric.
- `HOOK_BIRTH_OBJECTS` carries a reset hook that clears every activated flag; `ingame_clean` resets them the same way.
- `ingame_help_doc` SHALL `screen_save`, then `show_file(name, 0, -anchor, 0)`, then `screen_load`.

#### Scenario: Option gate

- **WHEN** a hooked event fires while `option_ingame_help` is not `TRUE`
- **THEN** the master closure returns without calling any list entry

- **Anchors**: `lib/core/help.lua:6-110` (registration), `:112-128` (the two reset functions), `:130-141` (doc)

### Requirement: Lua Boot Chain And Savefile Registry

`lib/core/init.lua` SHALL load, in order: `load.lua`, `xml.lua`, `util`, `player`,
`objects`, `monsters`, `powers`, `building`, `dungeon`, `s_aux`, `crpt_aux`,
`mimc_aux`, `quests`, `gods`, `help`, `stores` (all from `ANGBAND_DIR_CORE` via
`tome_dofile_anywhere`); it then calls `tome_dofile("init.lua")` into `lib/scpt`.

The patch system SHALL work as follows:

- `load_patches` scans `ANGBAND_DIR_PATCH` (skipping `.` and `..`); for each subdirectory containing a `patch.lua`:
- `patch_init` is cleared, and the patch is loaded in safe mode; `unset_safe_globals` wraps the `patch_init` check and `set_safe_globals` restores the guard.
- A missing `patch_init`, or a `patch_init()` that does not return a `name` and a `version`, each quit with an error.
- On success the version is registered through `patch_version` and a `patch_dofile[<subdir>]` closure is built.
- The last line loads `load2.lua` (source note: "Do not thouch after this line").

`lib/core/load.lua` SHALL provide the savefile variable registry:

- The `__loadsave_name` registry; `add_loadsave` asserts a name and a default value; a table default recursively expands into multiple dot-path entries (the comment states keys must be strings and array tables do not work).

`lib/core/load2.lua` SHALL provide the save/load pair:

- `__savefile_load` matches the key against the registry and assigns via `dostring`; `__savefile_save` reads each item into `__loadsave_tmp` via `dostring` and writes it with `save_number_key` (numbers only).
- `register_savefile(__loadsave_max)` registers the slot count and attaches both `HOOK_LOAD_GAME` and `HOOK_SAVE_GAME`.
- `reconstruct_table` rebuilds empty tables segment by segment along a dot path.
- The anonymous block that runs at load time SHALL `unset_safe_globals`, fill in the default value for every registered name, then `set_safe_globals` to restore.

- **Anchors**: `lib/core/init.lua:6-39` (load order), `:41-76` (patch system), `:78-84` (load2), `lib/core/load.lua:4-21` (registry), `:23-37` (example comment), `lib/core/load2.lua:5-34` (load/save hooks), `:37-63` (table rebuild and default fill)

### Requirement: Mimic Shape Registration

`lib/core/mimc_aux.lua` SHALL provide the mimic shape registry:

- The `__mimics` table counts from 1.
- `add_mimic_shape` asserts the five keys `name`/`desc`/`calc`/`level`/`duration`; `limit` defaults to 0, `obj_name` defaults to the shape name, `show_name` becomes `"[name]"`; when `hooks` is present it is attached via `add_hooks`.
- `resolve_mimic_name` returns the number from the `__mimics_names` hash, or -1.
- `find_random_mimic_shape` SHALL draw with `rand_range` within 1000 tries — a realm and limit double filter, then three rolls: `rand_int(level x 3) < level`, `rarity < 100`, and `magik(100 - rarity)`; total failure folds to `"Abomination"`.
- `get_mimic_info` returns 0 for a missing shape, otherwise reads the field by key.
- `get_mimic_rand_dur` draws inside the shape's `duration` range.
- `calc_mimic` delegates to `.calc()`; `calc_mimic_power` calls the power hook when present.
- The single built-in vital shape SHALL be `"Abomination"` (`obj_name` `"Abominable Cloak"`, rarity 101 so it never comes up at random, `calc` applying seven `TR1` stat flags plus `TR3_AGGRAVATE` through `apply_flags` with pval -10).

- **Anchors**: `lib/core/mimc_aux.lua:7-40` (registration and name lookup), `:42-79` (random draw and getters), `:83-95` (Abomination)

### Requirement: Monster And Object Wrappers

`lib/core/monsters.lua` SHALL provide `summon_monster` — when `typ` is a number it
picks `summon_specific_friendly` (with `Group_ok` always `FALSE`) or
`summon_specific` based on `friend`, otherwise it forwards to `summon_monster_aux`
(`typ` a Lua callback name).

`lib/core/objects.lua` SHALL provide:

- `create_object` through `new_object` plus `object_prep(lookup_kind(...))`.
- `set_item_tester` branching three ways on `tolua.type`: numbers are passed straight through, strings go with `tval=0`, functions are stored in the `__get_item_hook_default` global and registered under that name.
- `create_artifact(a_idx)`: takes `tval`/`sval` from `a_info`, builds the object, sets `name1 = a_idx`, then `apply_magic(-1, TRUE, TRUE, TRUE)`.
- `get_kind` returns `k_info[k_idx + 1]`.
- `get_item` SHALL call `set_item_tester(mask)` and then `get_item_aux(0, ask, deny, flags)`.

- **Anchors**: `lib/core/monsters.lua:6-16`, `lib/core/objects.lua:6-23` (creation and tester), `:25-45` (artifact, get_kind, get_item)

### Requirement: Player-Side Wrappers And Power Registration

`lib/core/player.lua` SHALL provide:

- The `deity`/`skill`/`subrace` accessors (all with the `1 +` index offset).
- `player.start_lasting_spell` sets `music_extra = -spl`.
- `player.modify_stat` adds into `stat_add`.
- `player.add_power` sets the powers slot.
- `player.inventory` forwards to `inventory_real`.
- `increase_mana` SHALL adjust `csp`, raise the `PR_MANA` redraw bit, clamp negative values to zero and return `TRUE` as a warning, and clamp to `msp` at the top.
- `player.get_wild_coord` picks `py`/`px` or `wilderness_y`/`wilderness_x` based on `wild_mode`.

Power registration SHALL work as follows: `add_power` asserts nine keys
(`name`/`desc`/`desc_get`/`desc_lose`/`stat`/`level`/`cost`/`fail`/`power`), creates the
number via `add_new_power`, then stores the `__power_fct[number] = power` closure;
`__power_fct_activate` dispatches on `HOOK_ACTIVATE_POWER`.

mkey registration SHALL work as follows: `add_mkey` asserts `mkey`/`fct` into
`__mkey_fct`; `__mkey_fct_activate` dispatches on `HOOK_MKEY`.

Plus: `subrace_add_power` fills the first free `powers[1-4]` slot (those holding -1)
and returns `nil` when all are taken; `player.add_body_part` adds into
`extra_body_parts`.

- **Anchors**: `lib/core/player.lua:7-36` (accessors and misc), `:40-61` (mana and wilderness coordinates), `:63-93` (power registration), `:96-119` (mkey registration), `:122-140` (subrace and body parts)

### Requirement: Magic Powers Wrapper

`lib/core/powers.lua` SHALL wrap the C magic-power system:

- `add_magic(m)` asserts `spell_list` is a table and then counts it; `new_magic_power` creates the group and the wrapper returns `{spells, max, fail_fct, stat (default A_INT), get_current_level (default returns player.lev), info[], spell[]}`; each entry asserts the seven keys `name`/`desc`/`mana`/`level`/`fail`/`info`/`spell` and the five fields are written through `get_magic_power`.
- `__get_magic_info` calls `__current_magic_power_info[power]()` for the menu info line.
- `execute_magic(m)` SHALL install `__current_magic_power_info`, then select a number with `select_magic_power` (`info_fct` set to `"__get_magic_info"`); with insufficient mana it prints `"Not enough mana!"` and returns; when `m.fail` exists the failure text goes to the `"__current_magic_power_fail"` global passed to `magic_power_sucess`, otherwise no text is set; on pass `m.spell[sn]()` casts and `increase_mana` deducts the cost.
- `get_level_power(s, max, min)` defaults to 50/1 and then evaluates `value_scale(get_current_level(), 50, max, min)`.

- **Anchors**: `lib/core/powers.lua:7-67` (registration), `:69-97` (execution), `:99-105` (level scaling)

### Requirement: Quest Registration

`lib/core/quests.lua` SHALL provide the quest registry:

- `add_quest` asserts the five keys `global`/`name`/`desc`/`level`/`hooks`.
- `new_quest` creates the number and `setglobal(q.global)` publishes it.
- The status is registered with `add_loadsave` under the name `"quest(<global>).status"` with default `QUEST_STATUS_UNTAKEN`.
- A table `desc` is written line by line through `quest_desc`; otherwise the function goes into `__quest_dynamic_desc` and the `dynamic_desc` bit is set.
- `level` and `silent` are stored onto `quest(i)` (`silent` defaulting to `FALSE`).
- Hooks are attached one by one with `add_hook_script` under the `"__lua__quest_callback<N>"` globals (the original functions are hooked directly).
- The `data` table entries are pushed with `setglobal` and registered with `add_loadsave` item by item.
- `add_quest` returns the quest number.

- **Anchors**: `lib/core/quests.lua:5-57`

### Requirement: Store Buy Lists

`lib/core/stores.lua` SHALL provide `store_buy_list(t)` — it asserts a table argument
and then hooks `HOOK_STORE_BUY`:

- The list entry is looked up by store number first and then by store name.
- A `function` entry returns `(TRUE, elt(obj))`.
- A `table` entry tests item by item (a number entry compares the object's `tval`; a three-element table compares `tval` and the `sval` range) and a hit returns `(TRUE, TRUE)`.
- An entry of -1 returns `(TRUE, FALSE)`, always denying.

#### Scenario: Deny-all entry

- **WHEN** the store's list entry for the item's store is -1
- **THEN** the hook answers `(TRUE, FALSE)` and the item is not bought

- **Anchors**: `lib/core/stores.lua:1-32`

### Requirement: Script Common Helpers

`lib/core/util.lua` SHALL provide:

- Safe globals: `safe_getglobal` raises `error` on an undefined name, active through `settagmethod` on the nil tag (`"getglobal"`), toggled by `set_safe_globals`/`unset_safe_globals` and enabled at load time.
- The patch registry `__patch_modules` — `patch_version` asserts on duplicate names, `patchs_list` lists the versions via `print_hook`, `patchs_display` via `msg_print`.
- `add_hooks(h_table, name_prefix)` attaches each entry with `add_hook_script` under the `"__<prefix>__hooks_list_callback<N>"` globals.
- The `msg_print(c, m)` wrapper — a numeric first argument goes to `cmsg_print`, otherwise it calls the original `msg_print`.
- `compass` judges the north/south and east/west axes with the +/-3 dead zone, returns `"close"` when both axes are near, and otherwise composes `"y-x"`.
- `approximate_distance` takes the larger axis difference and words four tiers at the 41/25/8 cut points.
- The `new_timer` wrapper asserts `delay > 0`/`enabled`/`callback`; function callbacks are stored as `"__timers_callback_N"` globals before the C timer is created and the `enabled` flag is stored.
- `save_timer` registers `enabled`/`delay`/`countdown` via dot-path `add_loadsave` entries.
- `display_list` converts between the Lua table and a C list (the `begin` and `sel` arguments each lose one to align the index bases).
- The three special-generation switches SHALL: `set_monster_generation` and `set_object_generation` resolve string names and then write `m_allow_special[+1]`; `set_artifact_generation` also writes `m_allow_special`.

- **Quirk:** the object and artifact switches write `m_allow_special`, but they should write `k_allow_special` and `a_allow_special` respectively — a live defect in the shipped code.

- `strcap` capitalizes the first letter.
- `msg_format` goes through `call(format, arg)`.
- `stack_push`/`stack_pop` raise `error` on an empty stack pop.
- `game.started` is set by a `HOOK_GAME_START` handler.

#### Scenario: Colored message wrapper

- **WHEN** the `msg_print` wrapper is called with a numeric first argument
- **THEN** the call is forwarded to `cmsg_print`; any other first argument goes to the original `msg_print`

- **Anchors**: `lib/core/util.lua:3-21` (safe globals), `:23-46` (patch registry), `:49-72` (add_hooks and msg wrapper), `:74-158` (compass and distance), `:160-201` (timers and lists), `:203-231` (generation switches and strcap), `:233-257` (stack and game flag)

### Requirement: XML Module

`lib/core/xml.lua` SHALL provide the thin XML parser and renderer:

- `parseargs` extracts `key='value'` pairs with `gsub`.
- `collect` is a stack-based tag parser (empty elements, start/end pairing, three error sites: nothing to close / tag mismatch / unclosed; whitespace-only text is dropped).
- Viewport coordinates `write_out_x`/`write_out_y`/`write_out_h`/`write_out_w` (24 x 80) and the offsets `write_off_x`/`write_off_y`.
- `write_screen` emits per-character `Term_putch` with clipping; `write_file` forwards to `print_hook`; `write` defaults to the screen.
- `rule2string` is a thirteen-entry English wording table (including the `foo1-3` placeholders).
- The `display_english` bit switches between the two `display_xml` paths.
- `english_xml` SHALL render recursively in natural language — `rule` (with an `inscribe` specialization), `and`/`or` (wording inverted under `not_flag`), `not` (an empty child prints `"(a negating rule)"`, otherwise the indent stays and `children_not_flag` is passed down), `comment`, the `skill`/`ability`/`level`/`sval`/`discount` range sentences, `rule2string` matches (with `" not"` appended when negated), and unknown tags under `not` degrade to `print_xml`; the selected state (`xml.write_active` and `t == auto_aux.rule`) highlights in violet.
- `print_xml` SHALL print the attributes, indent children by four spaces, and put the closing tag on its own line at the parent indent when element children were written, otherwise inline.
- `output` renders each top-level item in turn.

- **Anchors**: `lib/core/xml.lua:4-52` (parsing), `:54-88` (output channels), `:90-113` (wording table and switch), `:115-306` (english_xml), `:308-347` (print_xml and output)
