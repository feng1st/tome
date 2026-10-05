# content-maps Specification

## Purpose

Content maps and level definitions: `lib/edit/*.map` prefab quest/special-level
layouts (28 files), town layouts and assembly files (t_info/t_pref/special plus
the twelve town layouts with numenor/volcano and the t_basic base, 16 files), the
Middle-earth
wilderness overview map (w_info.txt), and `lib/dngn/*` dungeon level definitions
(19 files). Line anchors record only structure and key points; the grids (D: glyph
lines) are not transcribed — glyph semantics are defined by the F: lines of each
word-list and the pref files accompanying this directory. Formats and parsing
mechanics: process_dungeon_file (`specs/map-format/spec.md`), dngn commands
(N/U/D/F/A/L/B/S, `specs/dungeon-level/spec.md`), %:/?: inclusion (`src/init1.c`,
see `specs/data-loading/spec.md`).

## Requirements

### Requirement: Quest and Plot Maps (lib/edit/*.map)

The map family SHALL load through process_dungeon_file. Quest-side call sites:
q_betwen loads between.map, q_dragons loads dragons.map, q_evil loads evil.map,
q_haunted loads haunted.map, q_invas loads maeglin.map, q_nirna loads
nirnaeth.map, q_rand loads qrandN.map (N = 1/5/6/7/10/11/12/14, selected by
random_quests[type]), q_spider loads spiders.map, q_thief loads thieves.map,
q_thrain loads thrain.map (twice: once measuring the size, once loading it for
real), q_troll loads trolls.map (twice), q_wight loads wights.map, q_wolves loads
wolves.map; the eight s_*.map files are pulled in through U: lines of lib/dngn.
Per-file key points SHALL be:

- between.map — 71 lines, N:0 keeps monsters awake; the four Thunderlord color
 glyphs G/L/B/z bind races 955-958 (Z shares 958)
- dragons.map — 43 lines; mountain chains ^ and up-staircases <, start at 6:6
- evil.map — 52 lines; Lesser/Greater Balrog 996/807 and Pit Fiend 812
- haunted.map — 49 lines; locked door D:38, secret door S:48, glyph g a great
 item (`F:g:1:0:0:*:*` — random object plus random ego)
- maeglin.map — 85 lines, N:0; up staircase carrying Maeglin {:6:3:825, Ettin 621
 / War Troll 631 / Master mindcrafter 799 (the map comment says "Troll
 Chieftan", but r_info 799 is the Master mindcrafter) / Snaga sapper 251
- nirnaeth.map — 64 lines; the paired Olog 538 and Cave troll 496 (neither is a
 unique) plus the troll group 620/621/631/709
- qrand1.map — shares the common skeleton (reward p:969, glass wall G:188, locked
 door D:38, random trap t:1:8:*); theme adds lava F:85
- qrand5.map — the same skeleton in its most minimal, monster-free form
- qrand6.map — water pair W:84 / w:187
- qrand7.map — water pair W:84 / w:187
- qrand10.map — the trap glyph t carries cave_info 2056 (CAVE_ROOM|CAVE_FREE),
 plus lava F:85
- qrand11.map — lava pair L:85 / l:86
- qrand12.map — the water pair reversed: W:187 / w:84
- qrand14.map — water pair W:84 / w:187
- s_crypt.map — about 110 lines; %:special.txt pulls in the common glyphs, then
 a-j bind races 432/418/633/520/623/802/327/664/755/798 in order
- s_death.map — about 110 lines; a-j bind races 118/126/149/244/264/313/285/140/
 186/215
- s_name.map — about 110 lines; a-j bind races 728/589/549/601/624/675/756/790/
 741/793
- s_orc.map — about 110 lines; a-j bind races 40/118/126/149/264/238/285/330/313/
 244
- s_doom.map — 226 lines; lava wall 177 (X, #, and space), $ Great Fire 178, %
 blazing fire 205, L deep lava 85, random object * on shallow lava and random
 trap ^ on deep lava
- s_factory.map — 238 lines; machine terrain 189/213/214 and M:97
- s_gates.map — 117 lines, pulling in special.txt the same way
- s_ship.map — 239 lines; < > two-way staircases and wall 215
- spiders.map — 66 lines; three spiders 175/275/277
- thieves.map — 70 lines; M:38:22 and the door-opening d:4
- thrain.map — 35 lines; glyphs 1/2 bind NPCs 951/952, o:866, ten-column form with
 mimic=61
- trolls.map — 58 lines; H:96:1027 binding, f:297
- wights.map — 82 lines; f:381/g:470
- wolves.map — 55 lines; door D:4

- **Anchors**: `lib/edit/between.map:1-71`, `lib/edit/dragons.map`,
 `lib/edit/evil.map`, `lib/edit/haunted.map`, `lib/edit/maeglin.map`,
 `lib/edit/nirnaeth.map`, `lib/edit/qrand1.map` through
 `lib/edit/qrand14.map` (eight files), `lib/edit/s_crypt.map` through
 `lib/edit/s_ship.map` (eight files), `lib/edit/spiders.map`,
 `lib/edit/thieves.map`, `lib/edit/thrain.map`, `lib/edit/trolls.map`,
 `lib/edit/wights.map`, `lib/edit/wolves.map` (each file lines 1-end); consumers
 `src/q_betwen.c:78`, `src/q_dragons.c:34`, `src/q_evil.c:34`,
 `src/q_haunted.c:34`, `src/q_invas.c:30`, `src/q_nirna.c:30`,
 `src/q_rand.c:323,357`, `src/q_spider.c:30`, `src/q_thief.c:35`,
 `src/q_thrain.c:113,147`, `src/q_troll.c:30,142`, `src/q_wight.c:30`,
 `src/q_wolves.c:34`

### Requirement: Town Layouts and Assembly Files

The assembly chain SHALL — wild.c:227 loads t_info.txt through
process_dungeon_file; t_info.txt is a pure orchestration file: its first line
%:t_pref.txt pulls in the glyph table, then each of the five towns gets a pair of
?: [AND [EQU $TOWN n] [EQU $TOWN_DESTROYn]] / [NOT ...] conditions choosing
between the destroyed-town and intact layouts (Bree 1 / Gondor 2 / Minas Anor 3 /
Lothlorien 4 / Khazad-Dum 5, matching the ten files t_d_*.txt and t_*.txt).
t_pref.txt SHALL define the functional town glyphs — { the Barrow-Downs entrance
(special=4), ~ the Mirkwood entrance (special=1), | the Mordor entrance
(special=2), > the Angband entrance (special=3), mountain chain ^:97. Town layout
files SHALL bind quest states and buildings through F: lines — t_bree as the
representative: the z glyph switches between quest entrance and house per the
QUEST4 completion state, y/x switch on QUEST8 with DAYTIME and on QUEST9, and the
building glyphs B/b/a/c bind st_info numbers such as Castle plot / Mayor's House /
Prancing Pony (special=58) / Soothsayer (special=12); the other ten files follow
the same shape (grids not transcribed). numenor.txt SHALL be the lost Numenor
wilderness town — > entrance special=7, W water walls ringing the full map;
volcano.txt SHALL be the volcano town — $ special=6 on the mountainside, >
special=5 the central hole, L lava ringing the map. t_basic.txt currently has no
direct loading point in src (outside the t_info assembly chain; kept by the town
generation mechanism as a base / retired leftover).

- **Anchors**: `lib/edit/t_info.txt:1-41`, `lib/edit/t_pref.txt:1-20`,
 `lib/edit/t_bree.txt:1-45` (representative sample), `lib/edit/t_basic.txt`,
 `lib/edit/t_d_bree.txt`, `lib/edit/t_d_gond.txt`, `lib/edit/t_d_khaz.txt`,
 `lib/edit/t_d_lori.txt`, `lib/edit/t_d_mina.txt`, `lib/edit/t_gondol.txt`,
 `lib/edit/t_khazad.txt`, `lib/edit/t_lorien.txt`, `lib/edit/t_minas.txt`,
 `lib/edit/numenor.txt:1-14`, `lib/edit/volcano.txt:1-14`,
 `lib/edit/special.txt:1-25`; loading point `src/wild.c:227`, inclusion mechanism
 `src/init1.c:10991`

### Requirement: Middle-earth Wilderness Overview

w_info.txt SHALL be the Middle-earth overview map (W:/D: glyph lines; the source
note says it is designed to look like the Middle-earth map, drawn by DarkGod and
Gwidon S. Naskrent in the PernAngband era) — loaded twice, at wild.c:394 (large
map state) and wild.c:598 (small map state); the glyphs are symbols only, their
meaning resolved through the wf_info word-list terrain table and the t_info town
decision (mechanics in `specs/wilderness/spec.md`); X is the border column, g
glacier, t jungle, ^ hills, & mountains, _ shallow water, % the Angband entrance,
= deep water, . grass, M high mountains, digits town positions.

- **Anchors**: `lib/edit/w_info.txt:1-12` (head and representative lines); loading
 points `src/wild.c:394,598`

### Requirement: Dungeon Level Definitions (lib/dngn/*)

The command semantics (N name / U map / D description / F flags / A father branch
/ L depth / B branch staircase / S savefile snapshot) are specified in
`specs/dungeon-level/spec.md`; this requirement records the nineteen instance
files. Branch anchor files SHALL: dun1.14 (B:10, the Heart of the Earth
staircase), dun2.31 (B:5, the Mount Doom staircase), dun11.20 (B:6, the Nether
Realm staircase), dun22.10 (B:24, the Small Water Cave staircase). Father-branch
anchor files SHALL: dun5.0 (A:2 L:31, father of Mordor level 32), dun6.0 (A:11
L:20, father of Void level 20), dun10.0 (A:1 L:14, father of Mirkwood level 14),
dun24.0 (A:22 L:10, father of Moria level 10). The eight special levels SHALL
pair N+U+D with the flag rows DESC|NO_GENO|NO_NEW_MONSTER|SPECIAL|NO_STAIR, and
seven of them add ASK_LEAVE|NO_TELEPORT — dun5.14 Mt Doom is the exception, it
carries NO_TELEPORT without ASK_LEAVE: dun3.3 Crypt (U:s_crypt.map), dun3.18 Dim
Gates (s_gates.map), dun3.28 Nameless (s_name.map, source note: powerful
artifact), dun5.14 Mt Doom (s_doom.map, plus S:mdm as this level's savefile
snapshot), dun17.15 Machine (s_factory.map), dun19.11 Deathwatch (s_death.map),
dun22.5 Orc Town (s_orc.map), dun29.15 Galleon (s_ship.map). Miscellaneous SHALL:
dun11.22 carries a single F:NO_GENO line (the no-genocide exemption deep in
the Void); the S:mz0/mz1 lines of dun18.0/dun18.1 are currently commented out.

- **Anchors**: `lib/dngn/dun1.14:1-2`, `lib/dngn/dun2.31`, `lib/dngn/dun3.3:1-5`,
 `lib/dngn/dun3.18`, `lib/dngn/dun3.28`, `lib/dngn/dun5.0:1-3`,
 `lib/dngn/dun5.14:1-14`, `lib/dngn/dun6.0`, `lib/dngn/dun10.0`,
 `lib/dngn/dun11.20`, `lib/dngn/dun11.22`, `lib/dngn/dun17.15`,
 `lib/dngn/dun18.0`, `lib/dngn/dun18.1`, `lib/dngn/dun19.11`,
 `lib/dngn/dun22.5`, `lib/dngn/dun22.10`, `lib/dngn/dun24.0`,
 `lib/dngn/dun29.15`
