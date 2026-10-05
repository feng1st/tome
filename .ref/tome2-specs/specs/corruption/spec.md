# corruption Specification

## Purpose

The corruption system: `lib/core/crpt_aux.lua` carries the corruption registry
(`__corruptions`), the player-side accessor (`player.corruption`), the recursive
dependency and mutual-exclusion test (`test_depend_corrupt`), gain, loss and
lose-all (`gain`/`lose`/`lose_all_corruptions`), the registration piece
(`add_corruption`, with hook wrapping and mutual write-back), and spoiler
generation (`corruption_spoiler_generate`). The corruption definitions themselves
(the individual entries and their hooks) live in `lib/scpt/corrupt.lua` and are
recorded in the definitions requirement below; the C-side bridges
(`gain_random_corruption` and friends) are covered in specs/wish-corruption/spec.md.

## Requirements

### Requirement: Corruption Access And Testing

`player.corruption(c, set)` SHALL provide both read and write access to the
corruption state, and `test_depend_corrupt(corrupt, can_gain)` SHALL recursively
decide whether a corruption may be taken or dropped:

- the write path stores into `corruptions_aux[c+1]`, fires `PR_BASIC` plus the four
 groups `PU_BONUS`/`TORCH`/`BODY`/`POWERS`, calls the gain hook when set to TRUE
 and the lose hook when set to FALSE;
- the read path returns `corruptions_aux[c+1]`;
- `can_gain` defaults to FALSE;
- when the current state and the requested state disagree, the test fails;
- every `depends` entry must pass the test recursively (all true);
- every `oppose` entry must fail the test recursively (all false);
- when a `can_gain` hook exists and refuses, the test fails.

#### Scenario: Already in the requested state

- **WHEN** `test_depend_corrupt` evaluates a corruption whose current state
 matches the requested state
- **THEN** the test fails

#### Scenario: Opposed corruption present

- **WHEN** one of the candidate's `oppose` entries passes its recursive test
- **THEN** the whole test fails (all `oppose` entries must fail)

- **Anchors**: `lib/core/crpt_aux.lua:8-22` (accessor), `lib/core/crpt_aux.lua:30-63`
 (test)

### Requirement: Gain, Loss And Registration

`gain_corruption(group)`, `lose_corruption` and `add_corruption(c)` SHALL provide
the state changes and the registry:

- `gain_corruption(group)` collects candidates through four filters - by group,
 `test_depend_corrupt(i, TRUE)`, the random bit, and `allow()` - then `rand_int`
 picks one, sets it TRUE and plays its `get_text` (returns -1 with no candidates).
- `lose_corruption` collects candidates by `test_depend_corrupt` and the removable
 bit, picks one, sets it FALSE and plays its `lose_text`; a cascading scan then
 walks the remaining candidates and clears every one whose current state disagrees
 with the test.
- `lose_all_corruptions` loops `lose_corruption`.
- `add_corruption(c)` asserts the six required fields color/name/get_text/lose_text/
 desc/hooks; `random`/`removable` default to TRUE, `allow` defaults to always true;
 `depends`/`oppose` default to empty tables; every `oppose` entry writes the own
 number back into the other corruption's table; every hook is attached with
 `add_hook_script` as a `__lua__corrupt_callback<N>` global closure (the original
 hook is only called when the test passes) and the counter increments; a table
 `desc` is joined with newlines into a string; the entry goes into the table and
 its number is returned.

#### Scenario: No gain candidates

- **WHEN** `gain_corruption(group)` finds no candidate passing the four filters
- **THEN** it returns -1

#### Scenario: Loss cascades

- **WHEN** `lose_corruption` clears a picked corruption
- **THEN** the cascading scan clears every remaining candidate whose current
 state disagrees with the test

- **Anchors**: `lib/core/crpt_aux.lua:66-134` (gain and loss),
 `lib/core/crpt_aux.lua:137-181` (registration)

### Requirement: Spoiler Generation

`corruption_spoiler_generate` SHALL write a full spoiler listing into a temporary
file: `make_temp_file` first, then a header explanation (the source and permanence
text) plus an index anchor, then per corruption the `[[[[[B`-prefixed name, its
`desc`, its `get_text`, either the `lose_text` when the removable bit is set or
"It is not removable.", and the `depends` and `oppose` name lists; `close_temp_file`
follows and the temporary file's name is printed.

#### Scenario: Spoiling a permanent corruption

- **WHEN** `corruption_spoiler_generate` lists a corruption with the removable
 bit clear
- **THEN** "It is not removable." is written instead of the `lose_text`

- **Anchors**: `lib/core/crpt_aux.lua:186-243`

### Requirement: Corruption Definitions In corrupt.lua

`lib/scpt/corrupt.lua` SHALL define fourteen corruptions in four groups.

- Balrog group:
 - `CORRUPT_BALROG_AURA`: `xtra_f3` gains `TR3_SH_FIRE` and `TR3_LITE1`; a
 `HOOK_READ` burns the scroll five percent of the time and returns
 (TRUE, TRUE, FALSE).
 - `CORRUPT_BALROG_WINGS`: `TR4_FLY`; CHR -4 and DEX -2.
 - `CORRUPT_BALROG_STRENGTH`: STR+3 CON+1 DEX-3 CHR-1.
 - `CORRUPT_BALROG_FORM`: depends on the previous three; the five resistances
 IM_ACID/IM_FIRE/IM_ELEC/RES_DARK/RES_CHAOS; grants `PWR_BALROG`.
- Demon group:
 - `CORRUPT_DEMON_SPIRIT`: INT+1 CHR-2.
 - `CORRUPT_DEMON_HIDE`: `to_a` and `dis_to_a` gain the level, `pspeed` loses
 `level/7`, at level >= 40 adds IM_FIRE.
 - `CORRUPT_DEMON_BREATH`: grants `PWR_BR_FIRE`; a `HOOK_QUAFF` destroys the
 potion nine percent of the time and returns (TRUE, FALSE).
 - `CORRUPT_DEMON_REALM`: depends on the previous three; when the `SKILL_DAEMON`
 mod is zero it sets 1500 and unhides the skill.
- Teleport group:
 - `CORRUPT_RANDOM_TELEPORT`: `TR3_TELEPORT`; on `HOOK_PROCESS_WORLD` with
 `rand_int(300) == 1`, seventy percent ask via `get_check`, otherwise a forced
 `teleport_player(50)`; opposes ANTI_TELEPORT, which writes back mutually.
 - `CORRUPT_ANTI_TELEPORT`: opposes RANDOM_TELEPORT; a birth hook initializes
 `corrupt_anti_teleport_stopped`; a `PWR` entry `POWER_COR_SPACE_TIME`; while
 not stopped it grants `resist_continuum`; in the stopped state each world tick
 drains `(msp+csp)/100` with a floor of 1 mana to keep it running, and when
 `csp` reaches zero it recovers automatically and fires `PU_BONUS`.
- `CORRUPT_TROLL_BLOOD`: `can_gain` refuses the Troll race; grants `TR3_REGEN` plus
 AGGRAVATE plus ESP_TROLL.
- Vampire group (group "Vampire", all removable FALSE):
 - `CORRUPT_VAMPIRE_TEETH`: `allow` permits only when
 `test_race_flags(1, PR1_NO_SUBRACE_CHANGE)` has not forbidden it; `gain` runs
 `switch_subrace(SUBRACE_SAVE, TRUE)` to store the original subrace, then
 `subrace_add_power` attaches `PWR_VAMPIRISM` and sets the three flags
 PR1_VAMPIRE/UNDEAD/NO_SUBRACE_CHANGE.
 - `CORRUPT_VAMPIRE_STRENGTH`: depends on TEETH; `gain` applies `r_mhp+1`,
 `r_exp+100`, the six `r_adj` adjustments, then `do_rebirth`.
 - `CORRUPT_VAMPIRE_VAMPIRE`: depends on STRENGTH; `gain` gives the subrace title the
 "Vampire" prefix (or the title outright) with `place = FALSE`, sets
 `PR1_HURT_LITE`, sets `oflags2[2]` poison/nether/cold/dark resistances plus
 HOLD_LIFE, and sets `oflags3[2]` LITE1.

The end-of-file comment contains a blank corruption template.

#### Scenario: Balrog aura burns a scroll

- **WHEN** a `CORRUPT_BALROG_AURA` character reads a scroll and the five-percent
 draw hits
- **THEN** the scroll burns and the hook returns (TRUE, TRUE, FALSE)

#### Scenario: Troll blood refused

- **WHEN** `CORRUPT_TROLL_BLOOD`'s `can_gain` runs for a Troll
- **THEN** the corruption is refused

- **Anchors**: `lib/scpt/corrupt.lua:4-105` (Balrog group),
 `lib/scpt/corrupt.lua:109-205` (Demon group), `lib/scpt/corrupt.lua:212-286`
 (teleport group), `lib/scpt/corrupt.lua:290-317` (Troll Blood),
 `lib/scpt/corrupt.lua:320-420` (Vampire group), `lib/scpt/corrupt.lua:423-440`
 (template comment)
