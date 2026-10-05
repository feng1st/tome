# god-quest Specification

## Purpose

God quests: `lib/scpt/god.lua` carries the whole Lost Temple quest chain - quest
registration and per-deity granting (HOOK_PLAYER_LEVEL), wilderness site selection
for the temple (`place_rand_dung`), relic generation (`generate_relic`, driven by
HOOK_LEVEL_END_GEN), the per-deity rewrite of the temple dungeon parameters
(`set_god_dungeon_attributes`), direction hints (`get_god_quest_axes`), and the
character dump appendix. The dungeon 30 (`DUNGEON_GOD`) body parameters themselves
come from the `d_info` word-list and its parser in `src/init1.c`; the lua here only
rewrites them at grant time.

## Requirements

### Requirement: Quest Registration And Granting

The constants SHALL be `CHANCE_OF_GOD_QUEST` 21, `MAX_NUM_GOD_QUESTS` 5,
`DUNGEON_GOD` 30. The `add_quest` registration of GOD_QUEST SHALL define:

- a dynamic desc (in the TAKEN status it prints directions from both references via
 `get_god_quest_axes`);
- `level` -1;
- twelve data fields persisted into the savefile (`relic_num` 1,
 `quests_given`/`relics_found` 0, `dun_mindepth` 1 / `maxdepth` 4 / `minplev` 0,
 `relic_gen_tries` 0, `relic_generated` FALSE, `dung_x`/`dung_y` 1,
 `player_x`/`player_y` 0);
- HOOK_BIRTH_OBJECTS resets the quest state and the eight variables.

The `HOOK_PLAYER_LEVEL` grant path SHALL pass eight gates - the astral state, no
deity, TAKEN/FAILED status, `MAX_NUM_GOD_QUESTS` already reached, a missed
`magik(21)`, currently being inside dungeon 30 (with `dun_level > 0`), and level not
exceeding `dun_minplev` (anti-trickery: a deeper level raises `dun_minplev`). After
the gates pass it SHALL:

- set `relic_num` per deity (Eru 7 / Manwe 8 / Tulkas 9 / Melkor 10 / Yavanna 11);
- set the status to TAKEN and increment the grant counter;
- place the dungeon with `place_rand_dung`;
- record the player's wilderness coordinates;
- deliver the deity-named oracle long text (including the dual-reference
 directions);
- set the depth to `player.lev*2/3` for `dun_mindepth` and `dun_mindepth + 4` for
 `dun_maxdepth`.

#### Scenario: Faithless character never granted

- **WHEN** the `HOOK_PLAYER_LEVEL` grant path runs for a character without a
 deity
- **THEN** the no-deity gate refuses the grant

#### Scenario: Grant for Manwe

- **WHEN** all eight gates pass for a Manwe worshipper
- **THEN** `relic_num` is set to 8, the status becomes TAKEN, the grant counter
 increments, and the temple is placed with `place_rand_dung`

- **Anchors**: `lib/scpt/god.lua:4-13` (constants), `lib/scpt/god.lua:15-71`
 (registration and birth reset), `lib/scpt/god.lua:72-144` (granting)

### Requirement: Relic Generation And Collection

The `HOOK_LEVEL_END_GEN` driver SHALL: skip for a non-30 dungeon or an UNTAKEN
quest; when `relic_generated` is already TRUE and another level is generated, fail
the quest (FAILED) with the god's wrath long text (played only once); force
`generate_relic` on the fourth try, and otherwise generate when `randint(5) == 5`
and increment `relic_gen_tries` otherwise.

`generate_relic` SHALL:

- within 1000 tries roll `randint` over (height-1 / width-1) looking for a grid that
 is `FF1_FLOOR`, not `FF1_PERMANENT`, and trapless;
- `create_object(TV_JUNK, relic_num)` and add the quark "quest" inscription
 (so auto-pickers do not destroy it);
- when no safe grid was found, play the "luckily stumble" line and `inven_carry` or
 `drop_near` at the player's position, otherwise `drop_near` at the rolled
 coordinates;
- set `relic_generated` and clear the tries counter.

The `HOOK_GET` collection path SHALL, when all five conditions hold - TAKEN status,
`TV_JUNK`, `sval == relic_num`, `pval` not TRUE, and `found < given`:

- for the final piece (`given == MAX`) play the congratulations and add
 `10 * mod` to `SKILL_PRAY` (mod being the skill's `mod` field), for the other
 pieces play the encouragement and add `5 * mod`;
- remove the piece with `floor_item_increase`/`floor_item_optimize`, set
 `pval = TRUE`, increment `relics_found`, move the quest to UNTAKEN, and return
 TRUE.

`HOOK_CHAR_DUMP` SHALL, when quests were ever given, print `none` / the count /
`all` by `relics_found`, with the success or failure appendix.

#### Scenario: Wandering off fails the quest

- **WHEN** `HOOK_LEVEL_END_GEN` fires with `relic_generated` TRUE and another
 level is generated
- **THEN** the quest fails (FAILED) with the god's wrath long text, played only
 once

#### Scenario: Final piece collected

- **WHEN** the last relic piece is picked up with all five conditions holding
- **THEN** the congratulations play, `10 * mod` is added to `SKILL_PRAY`, the
 piece is removed, and the quest moves to UNTAKEN

- **Anchors**: `lib/scpt/god.lua:145-180` (generation driver),
 `lib/scpt/god.lua:205-246` (collection), `lib/scpt/god.lua:247-268` (dump),
 `lib/scpt/god.lua:322-371` (`generate_relic`)

### Requirement: Temple Site Selection And Parameter Rewrite

`place_rand_dung` SHALL:

- when an old temple exists, first erase it with the two-argument `place_dungeon`
 (no dungeon number) and clear `max_dlv[31]`;
- within 1000 tries roll `rand_range(1, bound-2)` for a point, rejecting grids with
 an existing entrance (both the block layer and the terrain layer) and the six
 terrains TERRAIN_EDGE / TERRAIN_DEEP_WATER / TERRAIN_TREES / TERRAIN_SHALLOW_LAVA
 / TERRAIN_DEEP_LAVA / TERRAIN_MOUNTAIN;
- when every try fails, fall back to (32, 19) (above Bree);
- place dungeon 30 with `place_dungeon`.

`set_god_dungeon_attributes` SHALL rewrite the dungeon 30 parameters per `pgod`:

- Eru: alloc 14/200; floors 88/89 in two segments 70/30 and 10/90; fill 97, outer
 wall 57, method 2; objs 45/5/45/5; DF1_BIG|NO_DOORS|CIRCULAR_ROOMS|EMPTY|TOWER|
 FLAT|ADJUST_LEVEL_2 plus DF2_ADJUST_LEVEL_1_2|NO_SHAFT|ADJUST_LEVEL_PLAYER; rules
 50% RF3_EVIL plus 50% RF7_CAN_FLY.
- Manwe: 18/160; floors 208/209 85/15; fill 211, outer wall 210, method 4; objs
 15/25/55/5; DF1_NO_DOORS|TOWER|CAVERN|ADJUST_2 plus NO_SHAFT|ADJUST_PLAYER; five
 rules of 20% each drawing INVISIBLE / ORC+IM_POIS / BR_POIS+BR_GRAV / BA_POIS /
 CAN_FLY.
- Tulkas: 20/120; floor 1; fill 56, outer wall 58, inner wall 57, method 0; objs
 10/70/5/15; DF1_NO_DESTROY|ADJUST_2 plus ADJUST_PLAYER; rule 100% DEMON+EVIL.
- Melkor: 24/80; floors 88/94/102 in three segments 45/45/10 and 35/35/30; fill 188,
 outer wall 188, inner wall 57, method 1; objs split evenly 25 across four entries;
 DF1_SMALL|LAVA_RIVERS|ADJUST_1 plus ADJUST_1_2|ADJUST_PLAYER; rule 80%
 unrestricted plus 20% RF3_GOOD.
- Yavanna: 22/100; floors 89/199/88 40/15/45; fill 96, outer wall 202, inner wall
 96, method 1; objs 20/10/30/40; DF1_NO_DOORS|WATER_RIVERS|NO_DESTROY|ADJUST_1|
 NO_RECALL plus ADJUST_1_2|NO_SHAFT|NO_GENO|ADJUST_PLAYER; rule 100%
 DEMON+UNDEAD+NONLIVING.
- the shared tail sets `mindepth`/`maxdepth`/`minplev`.

Three triggers SHALL call `set_god_dungeon_attributes` - HOOK_ENTER_DUNGEON
(entering dungeon 30), HOOK_GEN_LEVEL_BEGIN, and HOOK_STAIR (while inside dungeon
30). `get_god_quest_axes` SHALL pick the dual references per deity - non-Melkor
uses Bree (21,34) and Minas Anor (56,60), Melkor uses the Pits of Angband (7,34)
and the Land of Mordor (58,65) - each run through `compass` and
`approximate_distance`, returning the six-tuple.

#### Scenario: Site selection fallback

- **WHEN** all 1000 placement tries fail in `place_rand_dung`
- **THEN** the temple falls back to (32, 19), above Bree

#### Scenario: Entering the temple rewrites it

- **WHEN** `HOOK_ENTER_DUNGEON` fires for dungeon 30
- **THEN** `set_god_dungeon_attributes` rewrites the dungeon parameters per
 `pgod`

- **Anchors**: `lib/scpt/god.lua:273-319` (site selection),
 `lib/scpt/god.lua:376-606` (parameter rewrite), `lib/scpt/god.lua:181-204`
 (three triggers), `lib/scpt/god.lua:608-640` (direction axes)
