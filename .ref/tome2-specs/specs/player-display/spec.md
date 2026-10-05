# player-display Specification

## Purpose

Character display: `src/files.c` carries the seven character-screen modes - the standard
page (identity/age/six stats/middle-column numbers/miscellaneous ratings or background),
the five flag-matrix pages (equipment columns plus the symbiote column plus the player
column in a pierced comparison), the summary of the player's intrinsic flags
(`player_flags`) and of the symbiote flags (`wield_monster_flags`), and the character dump
file (`file_character`). The computation of the attribute values themselves is specified in
specs/player-derive/spec.md (calc_bonuses); this capability records only the presentation
and flag-summary rules.

## Requirements

### Requirement: Player Flag Summary

`player_flags` SHALL summarize flags from non-equipment sources:

- the Astral subrace grants wraith form;
- skill thresholds: SKILL_DAEMON above 20 grants resist_conf and above 30 grants
 resist_fear; SKILL_MINDCRAFT at 40 grants full ESP (ESP_ALL); SKILL_HAND above 24 with
 the SKILL_HAND melee style and no heavy armor grants free_act; SKILL_MANA at 35 grants
 TR1_MANA; SKILL_AIR at 50 grants both breathings (TR5_MAGIC_BREATH and
 TR5_WATER_BREATH); SKILL_WATER at 30 grants water breathing;
- piety thresholds per god: Eru (grace >= 100 or <= -100 grants TR1_MANA; grace > 10000
 grants WIS); Melkor (permanent resist_fire; melkor_sacrifice > 0 grants TR2_LIFE;
 grace > 10000 grants all five stats; while praying, grace > 5000 grants invisibility and
 grace > 15000 grants immune_fire); Manwe (grace >= 2000 grants feather fall; while
 praying, grace >= 7000 grants free_act, grace >= 15000 grants fly, and grace >= 5000 or
 <= -5000 grants speed); Tulkas (grace > 5000 grants CON, grace > 10000 grants STR);
- class/subrace/race per-level oflags bit tables are OR-ed in row by row up to the player
 level (the race layer is skipped while mimicking or possessing);
- a possessed form converts the body_monster flags (reflect/regenerate/both auras/wraith
 form/susceptible fire/four-element immunities plus poison/nether/nexus/disenchant
 resistances/no fear, no sleep, no confusion/feather fall);
- the xtra_f five groups and xtra_esp merge in; black_breath sets TR4_BLACK_BREATH; a
 non-zero hp_mod sets TR2_LIFE.

`wield_monster_flags` SHALL convert only four carried-symbiote flags: invisibility,
reflection, feather fall, water breathing.

- **Anchors**: `src/files.c:1868-1892` (symbiote), `:1898-2273` (player summary; the mimic
 branch as a whole is disabled in `:1989-2232`)

### Requirement: Character Page Seven Modes

The standard page SHALL display:

- name/sex (when possessing, sex follows the body_monster's male/female/neuter), full race
 name, speciality class title, host monster, god;
- age aged by the in-game calendar years (birth age plus elapsed years), height, weight and
 social class;
- the six stats (a drained stat shows its current value in yellow/orange plus the peak in
 green; 18/100 carries an exclamation mark);
- the middle column: melee and ranged hit/dam (a known weapon is merged in), base plus
 bonus armor, level, experience, experience cap, experience to next level, gold;
- the hit points/mana/sanity three bars (color changes below their warning lines; the
 undead form instead shows Death Points in blue/purple/red tiers), piety, speed (Fast/Slow
 relative to 110, with the searching state visually adding ten back).

The miscellaneous page SHALL rate: combat, bow (hit folded by twelve), saving throw (six),
stealth (one), perception/search (six), disarm (eight), magic device (six) through the
likert ten-step wording (Bad up to Legendary with an extended numeric suffix); the blows
and shots per round, the melee damage per round (bare-hand and Bear styles estimated from
the table's low/high entries, possession accumulated from the host's blow table, weapons
formatted as dice strings), the infra-red radius in feet, and the tactic and exploration
mode names.

The background page shows the four history lines.

- **Anchors**: `src/files.c:1423-1591` (middle column), `:1605-1679` (likert),
 `:1687-1860` (miscellaneous), `:2701-2830` (mode dispatch)

### Requirement: Flag Matrix

The flag pages SHALL present 192 flag names (from the object_flag_names table, empty slots
skipped) as equipment-slot columns (filtered by body part, letter headers) plus the
symbiote column plus the player column (@), split into five pages of 32 flags each;
parameter-type flags (the first fourteen of a group) show each slot's pval digits
(negative values in red, the player column with a sign); when the immune-nether flag is
present the resist-nether row is renamed to Imm Neth; colorless mode shows white.

- **Anchors**: `src/files.c:2278-2446` (name table), `:2451-2685` (matrix drawing)

### Requirement: Character Dump

`file_character` SHALL produce the character sheet: after confirming the overwrite, it
writes in order -

- the standard-page screen dump;
- a partial background page;
- the module patch list;
- the miscellaneous option states (joke/maximize/preserve/autoscum/small
 levels/arena levels/ironman rooms/persistent levels);
- the return-depth table;
- the rule-violation warnings (noscore/experimental/stupid);
- the host and mimic forms;
- the location description (describe_player_location);
- the total kill count;
- HOOK_CHAR_DUMP;
- the start and current (or final) Middle-earth date and day count;
- self_knowledge in full mode;
- the flag matrix's four pages as a screen dump (the first page carries the legend);
- the corruption list;
- the skill and ability tables;
- the companion death count;
- the known fates;
- the equipment/inventory/town houses/Mathom-house lists (artifacts, ego items, rings and
 amulets, and anything fully identified with IDENT_MENTAL in full mode get a full
 description; same-named stores are de-duplicated - the double Gondolin case is avoided).

The location description SHALL rank by dungeon level/town/landmark proximity; in the
wilderness it describes the position against the nearest seen landmark by the eight-way
compass (31/81 ratios decide the axis).

- **Anchors**: `src/files.c:2838-2924` (location), `:3084-3438` (dump)
