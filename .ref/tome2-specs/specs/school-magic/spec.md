# school-magic Specification

## Purpose

School magic system core: `lib/core/s_aux.lua` carries school and spell
registration (`add_school`/`finish_school`/`add_spell`/`finish_spell` with the four
`__spell_*` tables and `school_book`), the casting-level conversion
(`get_level_school`/`get_level_device`/`get_level`, three paths), mana cost and
display (the `get_mana`/`get_power` family, the `print_book` family), the casting
pipeline (`cast_school_spell` and `spell_chance`), misc helpers
(`have_object`/`get_random_spell`/`exec_lasting_spell`/`is_obvious`), wands and
activation (the `get_random_stick` family, `activate_*`), and new GF registration
(`add_spell_type`). The per-school spell definitions (`s_*.lua`) are covered in
specs/school-spells/spec.md; the C-side school table is documented under
`src/lua_bind.c`.

## Requirements

### Requirement: School And Spell Registration

The registration pieces SHALL work as follows:

- `add_school` stashes the school into `__schools` and returns its number;
- `finish_school` asserts name/skill, attaches hooks via `add_hooks` when hooks
 exist, and lands the school in the C table via `new_school`;
- `add_spell` stashes the spell into `__tmp_spells` and returns its number;
- `finish_spell(must_i)` asserts the seven keys name/school/level/mana/fail/spell/
 desc; `mana_max` defaults to `mana`; `info` defaults to an empty-string function;
 `random` defaults to SKILL_MAGIC; `lasting` must be a function; each `stick` table
 entry asserts `base_level`/`max_level`;
- **Quirk:** the stick-entry assert is guarded by `type(k) == "table"`, but the
 stick keys are the string "charge" and the numeric TV_WAND/TV_STAFF constants,
 so the guard never holds and the assert never fires;
- the `new_spell` return number must equal `must_i` exactly (on mismatch an assert
 asks for the maintainer to be contacted);
- when `school` is a single number, `__spell_school[i]` wraps it in a one-element
 table, otherwise the table is used directly;
- `spell(i)` lands the four fields mana/mana_max/fail/skill_level;
- the `__spell_spell`/`__spell_info`/`__spell_desc` tables attach closures by
 number;
- `find_spell` scans `__tmp_spells` linearly by name.

#### Scenario: Spell number mismatch

- **WHEN** `finish_spell` receives a `new_spell` return number that differs
 from `must_i`
- **THEN** an assert fires asking for the maintainer to be contacted

#### Scenario: Single school wrapped

- **WHEN** a spell's `school` field is a single number
- **THEN** `__spell_school[i]` wraps it in a one-element table

- **Anchors**: `lib/core/s_aux.lua:9-36` (stashing), `lib/core/s_aux.lua:38-77`
 (`finish_spell`), `lib/core/s_aux.lua:80-96` (tables and name lookup)

### Requirement: Casting Level Conversion

The level conversion pieces SHALL work as follows:

- `get_god_level(sch)` looks up `__schools[sch].gods` by the current `pgod` and
 returns the skill value scaled `* mul / div`, or nil when absent.
- `get_level_school(s, max, min)` - `max`/`min` default 50/1; a spell-level `depend`
 hook refusal returns (min, "n/a"); per school:
 - the god worship gate (a mismatch returns min, or 1 together with "n/a");
 - the `school(sch).skill` base value is taken; the school `depend` gate applies;
 - the sorcery bit merges in the SKILL_SORCERY value;
 - the `spell_power` bit requires unanimous permission (any school lacking the
 bit drops the SPELL-power bonus for the whole spell);
 - the `gods` value merges in `get_god_level`;
 - the largest of the three values wins and `bonus_level() * SKILL_STEP / 10`
 accumulates on top;
 - when any school lands at zero the result is "n/a" (source comment: all
 schools must be non-zero to be able to use the spell);
 - when `spell_power` is permitted by all, the SKILL_SPELL scale-20 bonus applies,
 then the `player.to_s` bonus is added (both `* SKILL_STEP / 10`);
 - the final value is averaged, divided by 10, and passed to `lua_get_level`
 (source comment: the division by ten guards against s32b overflow).
- `get_level_device`: the SKILL_DEVICE value plus
 `get_level_use_stick * SKILL_STEP`, capped at
 `get_level_max_stick + spell skill level - 1`, divided by 10, then
 `lua_get_level`.
- `get_level(s, max, min)` splits three ways - a number with
 `get_level_use_stick > -1` goes to the device path, a number goes to the school
 path, a table goes to `get_level_power`.
- `is_ok_spell`: a spell is unavailable when its level is zero or `obj.pval` is
 below the spell's `pval`.
- `get_mana`: `spell.mana` plus the `get_level` mana-gap segment.

#### Scenario: Depend hook refusal

- **WHEN** a spell-level `depend` hook refuses inside `get_level_school`
- **THEN** the conversion returns `(min, "n/a")`

#### Scenario: Any school at zero

- **WHEN** any school of a spell lands at zero in `get_level_school`
- **THEN** the result is "n/a"

- **Anchors**: `lib/core/s_aux.lua:99-105` (god level),
 `lib/core/s_aux.lua:108-201` (school conversion), `lib/core/s_aux.lua:204-247`
 (device and dispatch), `lib/core/s_aux.lua:250-259` (availability and mana)

### Requirement: Mana Cost And Display

The cost and display pieces SHALL work as follows:

- `get_power`/`get_power_name`: `check_affect(s, "piety")` picks
 `player.grace`/"piety", otherwise `csp`/"mana";
- `adjust_power`: with piety it goes through `inc_piety(GOD_ALL)`, otherwise
 `increase_mana`;
- `print_book`: a book of 255 is treated as a random book with `{spl}` temporarily
 attached; each spell is colored by `is_ok_spell` and the cost comparison
 (L_DARK unusable / ORANGE not enough mana / L_GREEN); school names are joined
 with "/"; the "n/a" case prints `%3s`; the line format carries letter number /
 name / school / level / cost / fail rate / `info()`; the header is
 "Name/School/Level/Cost/Fail/Info"; the last line number is returned;
- `print_spell_desc`: accepts a string or a table, prints the `desc` lines in
 L_BLUE, annotated with the piety/blind/confusion three-state notes (castable
 when not blind, castable when not confused);
- `print_device_desc` outputs through `text_out` with line breaks;
- `book_spells_num` is always 1 for a random book;
- `spell_x`: a random book returns `spl` directly, otherwise `next()` iterates to
 the s-th entry;
- `spell_in_book` searches linearly.

#### Scenario: Random book rendering

- **WHEN** `print_book` renders a book of 255
- **THEN** it is treated as a random book with `{spl}` temporarily attached

#### Scenario: Spell colors by usability

- **WHEN** `print_book` renders a spell the player cannot afford mana for
- **THEN** it is colored ORANGE (L_DARK for unusable, L_GREEN when castable)

- **Anchors**: `lib/core/s_aux.lua:262-286` (cost), `lib/core/s_aux.lua:289-333`
 (`print_book`), `lib/core/s_aux.lua:336-389` (desc and book counts),
 `lib/core/s_aux.lua:391-414` (`spell_x` and `spell_in_book`)

### Requirement: Casting Pipeline

The casting pieces SHALL work as follows:

- `spell_chance(s)`: the stick path (`get_level_use_stick > -1`) goes to
 `lua_spell_device_chance`, otherwise `lua_spell_chance` (arguments
 fail/`get_level(50)`/`skill_level`/`get_mana`/`get_power`/`get_spell_stat`).
- `check_affect(s, name, default)`: uses the field when it is a number, otherwise
 the default; FALSE returns nil, anything else returns TRUE.
- `get_spell_stat` defaults to A_INT.
- `cast_school_spell(s, s_ptr, no_cost)`:
 - the antimagic field and the anti_magic shell are a double gate refusing the
 cast (each with its own message);
 - the non-`no_cost` path passes the blind gate (including `no_lite`) and the
 confusion gate; when mana runs short a `get_check` prompt lets the player
 attempt the spell anyway;
 - on `magik(spell_chance)` success `__spell_spell[s]()` runs and a non-nil
 return records the use; on failure `flush()` runs when `flush_failure`,
 "You failed to get the spell off!" plays, the per-school fail hooks fire, and
 the use is still recorded;
 - the `no_cost` path casts directly;
 - after the use, `adjust_power(-get_mana)` applies and `energy_use` is set to 80
 or 100 per `is_magestaff`;
 - the wrap-up fires PR_MANA and PW_PLAYER.

#### Scenario: Antimagic double gate

- **WHEN** `cast_school_spell` runs inside an antimagic field or an anti_magic
 shell
- **THEN** the cast is refused (each with its own message)

#### Scenario: Failed cast still records

- **WHEN** `magik(spell_chance)` fails on a cast
- **THEN** "You failed to get the spell off!" plays, the per-school fail hooks
 fire, and the use is still recorded

- **Anchors**: `lib/core/s_aux.lua:417-431` (fail rate),
 `lib/core/s_aux.lua:433-453` (affect and stat), `lib/core/s_aux.lua:455-523`
 (casting)

### Requirement: Misc Helpers

The helper pieces SHALL work as follows: the HAVE_ARTIFACT/OBJECT/EGO three
constants; `have_object(mode, type, find, find2)` picks the scan segment by
USE_EQUIP (starting at INVEN_WIELD) and USE_INVEN (starting at zero) and matches in
three ways - artifact compares `name1`, object compares `k_idx` or the tval+sval
pair, ego compares `name2` or `name2b` - returning the slot number or -1;
`can_spell_random` returns the `random` field; `get_random_spell(typ, level)` draws
with `rand_int` within 1000 tries and hits only when `random == typ` and
`rand_int(skill level * 3) < level`, returning -1 when every try fails;
`exec_lasting_spell` asserts `lasting` and then calls it; `is_obvious(effect, old)`
combines the two truth values.

#### Scenario: Random spell draw exhausts

- **WHEN** `get_random_spell(typ, level)` fails all of its 1000 tries
- **THEN** it returns -1

#### Scenario: Artifact scan by name1

- **WHEN** `have_object` scans with the ARTIFACT type
- **THEN** it compares `name1` and returns the slot number, or -1 on no match

- **Anchors**: `lib/core/s_aux.lua:527-608`

### Requirement: Wands And Activation

The stick and activation pieces SHALL work as follows:

- `activate_stick` calls the spell closure and returns the obvious/charge pair;
- `get_random_stick(stick, level)` draws within 1000 tries among entries carrying a
 `stick` table; both rolls must succeed - `rand_int(skill level * 3) < level` and
 `magik(100 - rarity)`; all tries failing returns -1;
- `get_stick_base_level`/`get_stick_max_level`: out of range or without a table
 entry returns 0; a range exceeding half the dungeon level is clamped to half
 the dungeon level; the `m_bonus` scatter applies and `min` is added;
- `get_stick_charges` is `charge[1] + randint(charge[2])`;
- `get_activation_desc` formats the "every ... turns" text for an `activate` field
 given as a number or as `{base, dice}`; `get_activation_timeout` extracts its
 value the same way; `activate_activation` calls the spell closure with the item
 as argument.

#### Scenario: Stick draw exhausts

- **WHEN** `get_random_stick(stick, level)` fails both rolls on every one of its
 1000 tries
- **THEN** it returns -1

#### Scenario: Stick level out of range

- **WHEN** `get_stick_base_level`/`get_stick_max_level` is asked about a spell
 without a stick table entry or out of range
- **THEN** it returns 0

- **Anchors**: `lib/core/s_aux.lua:613-688` (stick family),
 `lib/core/s_aux.lua:691-713` (activation family)

### Requirement: New GF Registration

`add_spell_type(t)` SHALL allocate a new GF number from `max_gf` (starting at
MAX_GF) into `t.index`, assert `color`, default the monster/angry/object/player/
grid five handlers to empty functions, attach the double hook - HOOK_GF_COLOR
(color table via `new_gfx`) and HOOK_GF_EXEC (dispatching on the action string to
`t[action](who, dam, rad, y, x, extra)`) - and return the number.

#### Scenario: New GF allocated

- **WHEN** `add_spell_type(t)` registers a new GF
- **THEN** a number is allocated from `max_gf` (starting at `MAX_GF`) into
 `t.index` with the five handlers defaulted to empty functions

- **Anchors**: `lib/core/s_aux.lua:717-742`

### Requirement: School Registration And Book List (spells.lua)

`spells.lua` SHALL register twenty-one schools:

- the generic domain (all carrying the `spell_power` and `sorcery` bits): Mana
 (Eru half-rate praying; on HOOK_CALC_MANA, at skill >= 35 mana scales by
 `(skill - 34)%`), Fire, Air (Manwe two-thirds; on HOOK_CALC_BONUS, skill >= 50
 grants `magical_breath`), Water (Yavanna half; skill >= 30 grants
 `water_breath`), Earth (Tulkas four-fifths and Yavanna half), Conveyance (Manwe
 half), Divination (Eru two-thirds), Temporal (Yavanna one-sixth), Nature
 (Yavanna half), Meta (Manwe one-third), Mind (Eru and Melkor one-third each);
- the special cases: Geomancy (depend: the four skills Fire/Air/Earth/Water all
 non-zero and carrying a TV_MSTAFF), Udun (`bonus_level` returns `level*2/3`),
 Demon (no_random, random books never draw it);
- the five deity schools Eru/Manwe/Tulkas/Melkor/Yavanna (all SKILL_PRAY with the
 god bit, exclusive);
- plus Device (the placeholder stick-device path) and Music.

The load order SHALL run through `tome_dofile` in sequence: the thirteen
generic-domain files, the five deity files, s_demon, s_stick, s_music.

`school_book` SHALL list twenty-eight books: 0 crystal (Manathrust/Delcurses/
Resists/Manashield), 1 eternal flame (five spells), 2 wind (six), 3 earth (five),
4 tides (five), 5 conveyance (six), 6 the tree book (five), 7 knowledge (six),
8 time (four), 9 Meta (five), 10 mind (four), 11 hellflame (four), 20-24 the five
deity books (4/4/3/3/5 spells), 50 beginners (six), 51 conveyance (three),
52 summoning (two), 55-57 the three demon blades (three spells each), 58-60
drum/harp/horn (4/6/5 tunes each), 61 the player book (empty, filled by the
library quest), 62 Geomancy (eight spells, not a physical book).

#### Scenario: Demon never in random books

- **WHEN** a random book draws its spells
- **THEN** the Demon school is never drawn (`no_random`)

#### Scenario: Mana skill scales the pool

- **WHEN** HOOK_CALC_MANA runs for a Mana character at skill >= 35
- **THEN** mana scales by `(skill - 34)%`

- **Anchors**: `lib/scpt/spells.lua:7-181` (twenty-one schools),
 `lib/scpt/spells.lua:184-217` (load order), `lib/scpt/spells.lua:220-333`
 (book list)
