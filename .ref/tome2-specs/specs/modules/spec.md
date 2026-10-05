# modules Specification

## Purpose

The game module system, Lua side: `lib/mods/mods_aux.lua` carries the module registry
and runtime information (the `modules` table and `current_module`, layout redirection,
query helpers, savefile compatibility markers, extra-module scanning, and the
`add_module` registration with its defaults), `lib/mods/modules.lua` is the loading
entry point, and `lib/module.lua` is the module declaration for ToME itself. The
C-side module selection lives in `src/modules.c` — see specs/module/spec.md.

## Requirements

### Requirement: Module Registration And Query

`mods_aux.lua` SHALL provide:

- A sandboxing block for dangerous functions that is currently commented out in full (`execute`/`getenv`/`exit` and the rest stay usable).
- The registration entry `add_module(t)` SHALL assert the five keys `name`/`version` (as a table)/`desc`/`author`/`mod_savefiles` and raise `error` on a duplicate name; a string `author` is expanded to `{<author>, "unknown@unknown.net"}`; string entries of `mod_savefiles` are expanded to `{<name>, "all"}`; a table `desc` is joined with newlines into one string.
- The defaults SHALL be: `rand_quest` and `C_quest` `FALSE`, `base_dungeon` 4, `death_dungeon` 28, `astral_dungeon` 8, `astral_wild_x` 45, `astral_wild_y` 19, `random_artifact_weapon_chance`/`random_artifact_armor_chance`/`random_artifact_jewelry_chance` 30/20/20, `max_plev` 50, `max_skill_overage` 4, `skill_per_level` returning 6, `allow_birth` `TRUE`; then the module is appended with `tinsert` into `modules`.
- The query set SHALL be five same-shaped functions — `max_modules`/`get_module_name`/`get_module_desc`/`get_module`/`find_module` — each walking the numeric keys in order; `assign_current_module` attaches `current_module` by name; `get_module_info` reads `type` or `type[subtype]`; `exec_module_info` forwards `arg` through `call` to `current_module[type]`.

- **Anchors**: `lib/mods/mods_aux.lua:2-17` (sandbox comment and registries), `:19-84` (layout and the five query functions), `:86-96` (info and exec), `:98-111` (savefile markers), `:113-124` (extra scan), `:126-185` (add_module)

### Requirement: Savefile Compatibility And Loading Chain

`module_savefile_loadable(savefile_mod, savefile_death)` SHALL walk
`current_module.mod_savefiles` — an entry with the same name is always true when
marked `"all"`, true only for a living character when marked `"alive"`, true only for
a dead one when marked `"dead"`, otherwise false.

`setup_module` SHALL redirect each entry of a `layout` with `module_reset_dir` (the
standard game has no `layout` and returns immediately); `init_module` wraps it.

`scan_extra_modules` SHALL scan `ANGBAND_DIR_MODULES` (skipping `.` and `..`) and load
each subdirectory's `module.lua`.

The entry point `modules.lua` SHALL load `module.lua` from `ANGBAND_DIR` first and
then scan the extra modules.

#### Scenario: Alive-only savefile marker

- **WHEN** `module_savefile_loadable` checks a savefile whose module matches an entry marked `"alive"` while the character is dead
- **THEN** the savefile is judged not loadable

- **Anchors**: `lib/mods/mods_aux.lua:98-111` (compatibility), `:19-30` (layout), `:113-124` (scan), `lib/mods/modules.lua:1-5`

### Requirement: ToME Module Declaration

`lib/module.lua` SHALL declare ToME 2.3.5 through `add_module` — author DarkGod, a
three-line description, `rand_quest` and `C_quest` both `TRUE`, `base_dungeon` 4,
`death_dungeon` 28, the astral triple (8/45/19), the three random artifact chances
(30/20/20), `max_plev` 50, `max_skill_overage` 4, `skill_per_level` returning 6, and
`mod_savefiles` containing only `"ToME"`.

- **Anchors**: `lib/module.lua:1-36`
