# map-format Specification

## Purpose

The map command files: the process_dungeon_file family of `src/init1.c`
interprets `.txt` map/level description files (the t_*.txt, numenor.txt, and
special.txt files under lib/edit, the dungeon master files, and the boot-time
driving of p_basic and friends by init2.c). Content-family files (.map, t_*.txt,
and so on) are therefore skipped as "format analyzed, content not enumerated" —
the format is governed by this spec, and the content is governed by the source
data files at the anchors. The call side (special levels in generate.c, quest
levels in q_*.c, wilderness in wild.c) is specified in each capability:
`specs/dungeon-generation/spec.md`, `specs/content-maps/spec.md`,
`specs/wilderness/spec.md`.

## Requirements

### Requirement: Parsing Framework

`process_dungeon_file` SHALL: when the init parameter is true reset the letter
table (255 slots with defined false, the space slot ok, bx/by zeroed) and
meta_sleep; when full_text is non-empty read lines from the in-memory string
(my_str_fgets resets the cursor), otherwise open the file under ANGBAND_DIR_EDIT
(failure reports "Cannot find file" and returns -1); skip blank lines and #
comment lines line by line; evaluate `?:` condition lines into a bypass flag, with
all following lines skipped wholesale until the next condition line; recurse on
`%:filename` includes (the init parameter passed down false — the letter table is
not reset); and hand every other line to process_dungeon_file_aux, interrupting on
an error with the error code plus line number plus source line.

- **Anchors**: `src/init1.c:12038-12170`

### Requirement: Condition Expressions

`?:` lines SHALL be evaluated recursively by process_dungeon_file_expr: bracket
groups [FUNC args...] support IOR/AND/NOT/EQU/LEQ/GEQ (string-comparison
semantics), an unknown function name swallows its arguments as an always-true
placeholder (?o?o?/?x?x?); bare tokens protect spaces with quotes; `$` variables
expand — $SYS, $GRAF, $RACE, $RACEMOD, $CLASS, $PLAYER, $TOWN (town number),
$TOWN_DESTROY_n (destroyed state of town n), $QUEST_NUMBER, $LEAVING_QUEST,
$DAYTIME (bst folds 6-18 o'clock into day), $QUEST"name" (quest status looked up
by name), $VARIANT (always "ToME"), $WILDERNESS (NONE under vanilla_town,
otherwise NORMAL); a result of "0" means bypass.

- **Anchors**: `src/init1.c:11735-12035`

### Requirement: Letter Definition Line F:

`F:letter:feature:info:monster:object:ego:artifact:trap:special:mimic` SHALL
record the ten fields into letter[character] (fields not given are zeroed, ok and
defined set true); the feature/monster/object/ego/artifact/trap six fields accept
a `*` prefix setting the matching RANDOM_* bit (the remainder after the asterisk
is stored in the same field as a depth-offset parameter); the special field
supports the `"quest name"` quoted form looking the quest number up in the quest
table; the call side interprets the RANDOM_* bits.

- **Anchors**: `src/init1.c:10934-10962` (RANDOM_* bits and the dungeon_grid
 structure), `:11011-11175` (F lines)

### Requirement: Level Flag Line f:

`f:flag|flag` SHALL write into the global dungeon_flags1/2 through
grab_one_dungeon_flag (run-time level flags, sharing the d_info static flag name
table).

- **Anchors**: `src/init1.c:11178-11203`

### Requirement: Map Line D:

`D:character sequence` SHALL lay one row character by character: y taken from
*yval, x starting from xvalstart; during INIT_GET_SIZE only the cursors advance
and no grid is laid; a space glyph is skipped on the surface (dun_level zero) or
when the letter is undefined, while a space defined by an F: line is laid out
inside a dungeon (the plasma wilderness keeps its blanks); each grid lays the
feature (cave_set_feat), mimic, and cave_info bits; the monster field —
RANDOM_MONSTER places with place_monster at the quest level plus offset
(meta_sleep controls sleep), a named monster is let through m_allow_special and
placed hostile with place_monster_aux; the object field — with RANDOM_OBJECT and
RANDOM_TRAP both set it is 75% object and 25% trap, with RANDOM_OBJECT alone the
goodness is drawn in three ordered tiers (a named offset adjusts object_level), a
named object is let through k_allow_special, then object_prep plus
apply_magic(dun_level, good on) plus OBJ_FOUND_SPECIAL dropped in place; the
artifact field — let through a_allow_special, forged from a_info (object_prep
plus manual field copies, random_artifact_resistance, cur_num=1, and
TR5_SPELL_CONTAIN forcing pval2=-1 — working around the known Shadow Cloak of
Luthien problem, comment verbatim at the anchor), dropped with OBJ_FOUND_SPECIAL;
the special field — a value of -1 pairs a between gate (the first grid records
bx/by, the second and the first write each other's special coordinates packed as
(y<<8)+x), anything else writes c_ptr->special directly; at row end yval
increments, and xval takes the full row width when process_dungeon_file_full is
set.

- **Anchors**: `src/init1.c:11206-11427`

### Requirement: Wilderness Line W:, Player Line P:, and Limit Line M:

`W:` subcommands SHALL: `W:D:row` a wilderness map row — each character looked up
in reverse through wildc2i and written into wild_map[y][x].feat, the first placed
terrain with an entrance recording the wf_info entry's wild_x/wild_y;
`W:M:+-n` advancing or rewinding yval; `W:P:x:y` the wilderness start (written
into wilderness_x/y only while the player has none); `W:E:dungeon:y:x` setting
wild_map entrance = 1000 + dungeon number. `P:x:y` SHALL act only during
INIT_CREATE_DUNGEON — a quest level or INIT_POSITION sets py/px directly,
otherwise the first town landing sets oldpy/oldpx. `M:type:value` SHALL set the
various max_* limits — T/t towns (real towns), R/r monsters (monster egos), k
skills (rejected when beyond MAX_SKILLS), b abilities, K objects, V vaults, F terrain, A
artifacts, a alchemy recipes, E egos, Z randarts, O object entities, P:R|S|C|M|H
player races / subraces / classes / metaclasses / histories, M monster entities,
U traps, W wilderness feats, B store actions, S stores, s item sets, N store
owners, X/Y wilderness extents, D dungeons.

- **Anchors**: `src/init1.c:11430-11519` (W), `:11522-11544` (P),
 `:11547-11723` (M)

### Requirement: N: Sleep Switch

`N:0|1` SHALL set meta_sleep, controlling whether the following D rows' random
monsters generate asleep (place_monster's sleeping parameter).

- **Anchors**: `src/init1.c:10962` (default true), `:10998-11008` (N lines),
 `:11250-11266` (consumption side)
