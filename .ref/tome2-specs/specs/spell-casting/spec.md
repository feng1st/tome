# spell-casting Specification

## Purpose

Spell launcher pieces and helper spells: `src/spells2.c` carries every spell launch
wrapper (`fire_ball`/`fire_bolt`/`fire_beam`/`fire_cloud`/`fire_wave`/`fire_wall`
and friends), the group spell wrappers (the `project_hack` family), the detection
spells (the `detect_*` family), identification, recharging and enchantment,
destruction and earthquake, room light and darkness, large-range movement
(`passwall`/swap), the curse triggers (TY/DG), the genocide family, the full
`self_knowledge` text, the god-given `bless_weapon`, the recall reset, and the
between gates. The projection body itself (the project tetrad) is covered in
specs/spell-effects/spec.md.

## Requirements

### Requirement: Growth, Life And Light

- `grow_things`/`grow_trees`/`grow_grass` SHALL lay their terrain on clean grids
 within `rad^2 + 11` samples (the mean of two Rand_mod rolls); trees and grass
 need an FF1_SUPPORT_GROWTH base.
- `hp_player` SHALL add hit points capped at `mhp` (guarding against negative
 wrap-around), with the four wording tiers (<5/<15/<35/rest).
- `warding_glyph`/`explosive_rune` SHALL place FEAT_GLYPH/FEAT_MINOR_GLYPH on bare
 floor (refused when an object is present).
- `stair_creation` SHALL pass the `cave_valid_bold` plus DF1_FLAT plus DF2_SPECIAL
 double gate, delete the grid's object, and set up or down stairs per the
 situation (arena quest invalid, surface only down, top quest level only up,
 otherwise fifty-fifty).

#### Scenario: Healing capped

- **WHEN** `hp_player` would push hit points past `mhp`
- **THEN** the total is capped at `mhp`

#### Scenario: Glyph refused on an object

- **WHEN** `warding_glyph`/`explosive_rune` targets a floor grid holding an
 object
- **THEN** the placement is refused

- **Anchors**: `src/spells2.c:28-89` (growth), `src/spells2.c:94-149`
 (`hp_player`), `src/spells2.c:156-180` (glyphs), `src/spells2.c:2817-2865`
 (stair creation)

### Requirement: Identification, Curses And Alchemy

The identification pieces SHALL work as follows:

- `identify_pack` makes the whole body aware+known and fires HOOK_IDENTIFY;
- `make_item_fully_identified` adds IDENT_MENTAL plus `alchemist_learn_object`;
- `identify_pack_fully` applies that to the whole body;
- `ident_spell` uses `item_tester_hook_unknown` to pick one item and identify it
 (three-tier wording; artifacts with `take_notes`/`auto_notes` on record the
 notes plus `irc_emote`);
- `ident_all` does the same for all floor objects;
- `identify_fully` uses `item_tester_hook_no_mental` to pick an item, runs it
 through `make_item_fully_identified` and displays it with `object_out_desc`;
- `lose_all_info` clears the sense/EMPTY/KNOWN/SENSE four states of every
 non-MENTAL worn item and forgets the map with `wiz_dark`.

The curse pieces SHALL work as follows:

- `remove_curse_object` - only removable when not PERMA_CURSE (the non-all
 variants also refuse HEAVY_CURSE); clears the ident and `art_flags3` curse bits,
 records SENSE_UNCURSED, and on a `1/(55 - level)` roll, when not an artifact,
 reverses negative to_a/to_h/to_d/pval;
- `remove_curse`/`remove_all_curse` scan the whole body.

`alchemy` SHALL pick an item, ask for a quantity (`command_arg` forces it through),
refuse artifacts while marking them SENSE_SPECIAL or the TERRIBLE feeling, convert
to gold at one third of `object_value_real` (a zero price becomes fool's gold), and
destroy the item. `restore_level` SHALL restore exp up to `max_exp` and run
`check_experience`.

#### Scenario: Perma curse stays

- **WHEN** `remove_curse_object` runs on a `PERMA_CURSE` item
- **THEN** the curse is not removable

#### Scenario: Zero-price alchemy

- **WHEN** `alchemy` converts an item whose `object_value_real` is zero
- **THEN** the yield is fool's gold

- **Anchors**: `src/spells2.c:351-416` (identification), `src/spells2.c:1912-1954`
 (`lose_all_info`), `src/spells2.c:430-533` (curse removal),
 `src/spells2.c:563-690` (alchemy), `src/spells2.c:540-560` (level restore)

### Requirement: Stat Wrappers And Self-Knowledge

- The stat wrappers SHALL work as follows: `do_dec_stat` (a sustain blocks it and
 plays a momentary feeling, otherwise `dec_stat(10, mode)` plus negative wording),
 `do_res_stat` (`res_stat` plus bidirectional wording), `do_inc_stat`
 (`res_stat(TRUE)` first, then `inc_stat` plus positive wording).
- `self_knowledge` SHALL produce the summary (full-screen paged display when `fff`
 is empty, otherwise written to the file): dead characters list cause of death and
 position on the first line; the possession state lists about eighty perception
 lines by the `body_monster` monster flags (including several "Not implemented"
 status comments); power descriptions follow `p_ptr->powers`; then
 `allow_one_death`, all timed states, the telepathy breakdown, the luck(-100,100)
 five-tier wording (note: each call rolls independently - a live quirk), the
 auto_id/hold_life/reflect/aura/anti-magic/spirit-body/anti_tele/permanent-light
 entries; resistances in three tiers (immunity / double resist "exceptional" /
 single resist) and fifteen singles; the six sustains; black_breath; twelve
 equipment flag hints; and the current weapon's bless/chaos/impact/vorpal/
 vampiric/five brand/eight slay/three kill lines.
- `report_magics` SHALL list the twenty current timed states with the
 `report_magics_aux` seven-tier duration wording (`confusing` always tier seven).

#### Scenario: Sustain blocks the drain

- **WHEN** `do_dec_stat` targets a sustained stat
- **THEN** the drain is blocked and a momentary feeling plays

#### Scenario: Luck wording rolls live

- **WHEN** `self_knowledge` lists the luck five-tier wording
- **THEN** each tier rolls independently (a live quirk)

- **Anchors**: `src/spells2.c:228-343` (stat wrappers),
 `src/spells2.c:708-1712` (`self_knowledge`), `src/spells2.c:1715-1905`
 (`report_magics`)

### Requirement: Detection Spells

The detection pieces SHALL work as follows (all within the radius `rad` circle):

- `detect_traps` sets CAVE_DETECT on every grid, remembers trapped grids plus
 `pick_trap` plus PR_DTRAP, and always returns true (status comment: so that
 unidentified devices can prove themselves);
- `detect_doors` turns secret doors into normal doors and adds door memory;
- `detect_stairs` remembers the six stair kinds;
- `detect_treasure` exposes gold/quartz veins (MAGMA_H/QUARTZ_H plus 0x02,
 SANDWALL_H becomes K) and then remembers them;
- `detect_objects_gold`/`detect_objects_normal` scan the `o_list` (monster-held
 objects visible only with RF9_MIMIC) and remember by tval category; the gold
 version also links `detect_monsters_string("$")`;
- `detect_objects_magic` judges by artifact/ego/randart/thirteen tvals/positive
 bonuses;
- the monster detect family - normal (not invisible, or see-invisible), invis
 (RF2_INVISIBLE, records lore), evil (RF3_EVIL, records lore), string (a character
 set), xxx (any flags3 bit, with the demons/undead/good three wording sets per
 bit), good, nonliving (the NONLIVING/UNDEAD/DEMON three factions) - all set
 MFLAG_MARK|SHOW, adding `repair_monsters` when `ml` is true;
- `detect_all` combines the eight items.

#### Scenario: Trap detection always succeeds

- **WHEN** `detect_traps` runs within its radius circle
- **THEN** `CAVE_DETECT` is set on every grid and the call always returns true

#### Scenario: Doors become known

- **WHEN** `detect_doors` sweeps its radius
- **THEN** secret doors turn into normal doors and door memory is added

- **Anchors**: `src/spells2.c:1962-2103` (traps and doors),
 `src/spells2.c:2095-2245` (stairs and buried gold), `src/spells2.c:2217-2440`
 (objects), `src/spells2.c:2446-2810` (monster family and `detect_all`),
 `src/spells2.c:7587-7646` (nonliving)

### Requirement: Enchantment And Randart Word Generation

- `enchant` SHALL apply the stack penalty `prob = number * 100` (ammo divided by
 20); of the `n` attempts those drawing `rand_int(prob) >= 100` are dropped; the
 four channels (TOHIT/TODAM/PVAL/TOAC) each check the `enchant_table` sixteen
 tiers (negative values always succeed, above 15/6 always fail) and artifacts
 resist with fifty percent; on success with the value turning positive and no
 PERMA_CURSE, a one-in-four roll removes the curse; success refreshes PU_BONUS.
- `enchant_spell` SHALL pick armor by `num_ac` or a weapon through the hook, play
 the glowing announcement, then enchant each channel; all failures play
 "The enchantment failed.".
- `curse_artifact` SHALL reverse the four values plus d4, set HEAVY+CURSED, and
 roll 1/3 TY, 1/2 AGGRAVATE, 1/3 DRAIN_EXP, 1/3 BLACK_BREATH, and 1/2 TELEPORT
 else 1/3 NO_TELE.
- The randart word-generation pieces SHALL work as follows:
 - `random_plus` - first the free stat additions per `artifact_bias` (BIAS_WARRIOR
 three stats, BIAS_MAGE/PRIESTLY and friends one stat each), then the
 flag roll in 23/19 tiers by tval (SPEED has its own tier, BLOWS is rerolled for
 bows, each tier sets the bias);
 - `random_resistance` - first the free resists per bias (ACID/ELEC/FIRE/COLD/
 POIS/WARRIOR/NECROMANTIC/CHAOS, including the immunity 1/BIAS_LUCK case and
 auras), then `specific` or `randint(41)` rolls the forty-one tiers (1-4 the
 WEIRD_LUCK one-twelfth immunity tier, 5-38 regular resists, 39/40 the
 cloak-armor-electric-fire aura, 41 shield-cloak-helm-hard-armor REFLECT,
 out-of-range rerolls);
 - `random_misc` - a free sustain per bias, or TELEPORT/LITE1, then `randint(31)`
 rolls the thirty-one tiers (six sustains, three FREE_ACT, HOLD_LIFE, two LITE1,
 two FEATHER, three SEE_INVIS, a random ESP bit, two SLOW_DIGEST, two REGEN,
 TELEPORT, SHOW_MODS+to_a or to_h/to_d 4+d11 in three-three, NO_MAGIC,
 NO_TELE);
 - `random_slay` - a free brand per bias (CHAOS confusion, PRIESTLY the keen
 BLESSED blade, NECRO vampiric-poison, RANGER beast-slaying, ROGUE/POIS poison,
 the four element brands, LAW the three-holy set); non-bows roll `randint(34)`
 over the slay/kill/brand/vampiric/chaotic tiers (VORPAL swords only, 18-19
 rerolled for non-swords); bows roll MIGHT or SHOTS.

#### Scenario: Artifact resists enchantment

- **WHEN** `enchant` works an artifact through a channel
- **THEN** the artifact resists with fifty percent

#### Scenario: Curse stripped by enchanting

- **WHEN** an enchantment succeeds, the value turns positive, there is no
 `PERMA_CURSE`, and the one-in-four roll hits
- **THEN** the curse is removed

- **Anchors**: `src/spells2.c:2980-3216` (enchantment), `src/spells2.c:3218-3235`
 (artifact cursing), `src/spells2.c:3242-3461` (plus),
 `src/spells2.c:3464-3766` (resistance), `src/spells2.c:3768-3976` (misc),
 `src/spells2.c:3979-4257` (slay), `src/spells2.c:422-428` (`enchant_table`)

### Requirement: Recharging

`item_tester_hook_recharge` SHALL refuse TR4_NO_RECHARGE and accept
staff/wand/ROD_MAIN. `recharge` SHALL work as follows:

- the ROD path - `strength = ((power > lev) ? power - lev : 0) / 5`; when
 `rand_int(strength) == 0` and the item lacks TR4_RECHARGE it explodes, otherwise
 `timeout` gains `power * 3d2`, capped at `pval2`;
- the wand/staff path - `strength = (100 + power - level - 8 * pval / stack
 count) / 15` for a stacked wand, `(100 + power - level - 8 * pval) / 15` for
 staffs and unstacked wands; the same explosion test applies (TR4_NO_RECHARGE
 always explodes); success charges `randint(power / (level + 2) + 1)`, with the
 wand stack bonus (capped at 12) or the staff stack even split (floor one);
 `pval` gains the amount and loses KNOWN (kept with TR4_RECHARGE), EMPTY is
 cleared, and TR4_RECHARGED is always set for the alchemists;
- the failure path - artifacts are never destroyed, only completely drained; other
 items split by AB_PERFECT_CASTING: with the talent a rod explodes one-in-ten and
 otherwise drains, a wand explodes one item two times in three and otherwise
 drains, a staff explodes one item half the time and otherwise does nothing;
 without the talent a rod explodes one-in-three and otherwise drains, a wand
 explodes the whole stack one time in five and otherwise explodes one item, a
 staff always explodes one item;
- `fail_type` 2 destroys one item (a wand's `pval` is cleared), 3 destroys the
 whole stack.

#### Scenario: Non-rechargeable wand explodes

- **WHEN** a wand with `TR4_NO_RECHARGE` enters the recharge path
- **THEN** the explosion test always detonates it

#### Scenario: Artifact failure only drains

- **WHEN** a recharge failure strikes an artifact
- **THEN** it is never destroyed, only completely drained

- **Anchors**: `src/spells2.c:4502-4838`

### Requirement: Group Spell Wrappers

- `project_hack` SHALL run an independent `project(0, 0, JUMP|KILL|HIDE)` for every
 monster in view.
- The wrapper family SHALL map directly: speed/slow/conf/sleep/scare_monsters use
 the player level; `banish_evil(dist)`, `turn_undead`,
 `dispel_undead`/`evil`/`good`/`monsters`/`living`/`demons(dam)`,
 `confuse`/`charm`/`charm_animals`/`charm_demons`/`stun`/`stasis`/`mindblast`/
 `banish`/`turn_evil`/`turn_monsters(dam)`, `deathray_monsters` (level).
- The single-point wrapper family SHALL go through `project_hook` (PROJECT_THRU
 plus `dir = 5` targeting): `fire_bolt` (STOP|KILL), `fire_beam` (BEAM|KILL),
 `fire_bolt_or_beam` (a probability roll), `lite_line` (BEAM|GRID|KILL, 6d8
 damage), `drain_life`, `wall_to_mud` (20+d30), `wizard_lock` (JAM_DOOR),
 `destroy_door`/`disarm_trap`, `heal_monster` (4d6),
 speed/slow/sleep/stasis/confuse/stun/poly/clone/fear/death_ray_monster (each at
 its level, clone at zero), `teleport_monster` (resist_continuum gate, AWAY_ALL `MAX_SIGHT * 5`),
 `charm`/`star_charm`/`control_one_undead`/`charm_animal`.
- The neighborhood family SHALL go through `project(0, 1, player grid)`:
 door/trap/glyph_creation, `wall_stone` (the trick of turning the own grid to
 floor after the wall ring), `destroy_doors_touch`/`destroy_traps_touch`/
 `sleep_monsters_touch`.
- `project_meteor` SHALL enumerate `radius + randint(radius)` meteors, each finding
 a visible non-wall point within five grids of the player and running
 `project(0, 2, JUMP|flg)`.
- `fire_godly_wrath`/`fire_explosion` are fixed-point balls.
- `fire_ball`/`fire_cloud` (adding PROJECT_STAY and `project_time`) / `fire_wave`
 (also setting `project_time_effect`) / `fire_wall` (BEAM|STAY) / `fire_druid_ball`
 (MANA_PATH) / `fire_ball_beam` (BEAM+STOP) all use a virtual target 99 grids away
 plus `dir = 5` targeting, with the radius capped at 16.

#### Scenario: Per-monster independent hits

- **WHEN** `project_hack` runs
- **THEN** an independent `project(0, 0, JUMP|KILL|HIDE)` fires for every
 monster in view

#### Scenario: Radius ceiling

- **WHEN** the area launch family (`fire_ball`, `fire_cloud`, `fire_wave`,
 `fire_wall` and friends) is given a radius over 16
- **THEN** the radius is capped at 16

- **Anchors**: `src/spells2.c:4847-4909` (hack and meteor),
 `src/spells2.c:4915-5017` (group wrappers), `src/spells2.c:6374-6556` (area
 launch family), `src/spells2.c:6773-7055` (single-point and neighborhood
 family), `src/spells2.c:7652-7771` (extended group family),
 `src/spells2.c:7747-7771` (charm single-point)

### Requirement: Destruction, Earthquake And Room Light

- `wipe` SHALL pass the DF2_NO_GENO and quest-level double gate, then inside the
 radius circle clear Room/Icky/Mark/Glow, delete monsters (COMPANION exempt) and
 objects, `place_floor`, and refresh the four groups at the end.
- `destroy_area` SHALL pass the same gates (the bypass flag exempts quest levels),
 turn the player's grid to flag with an after-burn/blind (double resist exempt),
 skip the quake-center grid, delete non-companion monsters, and on
 `cave_valid_bold` grids delete objects then reroll by `t` (20 granite / 50 quartz
 / 30 magma / rest floor).
- `earthquake` SHALL be refused on quest levels, cap the radius at twelve, and mark
 fifteen percent of the in-range grids as damaged in a 32x32 map. The player injury path: wraith is
 immune; the neighborhood searches for a non-quake safe grid - none means 300
 damage with "severely crushed", one found means the three-way wording (dodge for
 zero damage / falling rock 10d4 plus stun / caught 10d4 plus stun) plus
 displacement, and SEMI_WRAITH takes one quarter damage. The monster path: monsters
 without KILL/PASS_WALL cry out and take 4d8 (can flee) or 200 (buried) damage,
 negative-HP monsters are deleted (non-kill style), and fleers are moved to a safe
 grid. The terrain path: floor grids reroll by `t = rand_int(100)`, wall grids
 always become floor; the wrap-up refreshes PU_UN_VIEW and four more groups.
- The room light pieces SHALL work as follows: `cave_temp_room_aux` floods along
 CAVE_ROOM (TEMP capped); `lite_room` sets CAVE_GLOW and wakes monsters (STUPID
 ten percent, SMART all, the rest one quarter); `unlite_room` clears GLOW and
 forgets normal floors.

#### Scenario: Quest level refuses the quake

- **WHEN** `earthquake` is attempted on a quest level
- **THEN** it is refused

#### Scenario: Crushed with no escape

- **WHEN** the player is caught in an earthquake and the neighborhood holds no
 non-quake safe grid
- **THEN** 300 damage lands with "severely crushed"

- **Anchors**: `src/spells2.c:5480-5557` (wipe), `src/spells2.c:5568-5702`
 (destroy), `src/spells2.c:5723-6103` (earthquake), `src/spells2.c:6122-6366`
 (room light)

### Requirement: Large-Range Movement Pieces

- `teleport_swap` SHALL gate on `resist_continuum`, target with `dir = 5` or take
 an adjacent grid, and with a monster present and no RES_TELE swap the player and
 the monster (the inscription executes in both directions - the monster goes by
 the MONST_WALK bit, the player shows the text plus the WALK bit), refreshing the
 four groups; `swap_position` is the same structure but also moves directly when
 the target grid has no monster (the FORCE pushing uses it).
- `passwall` SHALL refuse on wild_mode / quest level / NO_TELEPORT / `dir = 5`
 (four refusals), push through walls along `dir` (skipping monster grids and ICKY,
 recording the retreat point lx/ly), stop at a permanent wall or the map edge,
 take 10d8 damage with "becoming one with a wall" when no landing point exists and
 not `safe`, then `place_floor` and move with the four groups refreshed.
- `change_wild_mode` SHALL warn the immovable, be blocked by `word_recall`, and
 toggle `wild_mode` plus autosave plus leaving; `alter_reality` announces plus
 autosave plus leaving.

#### Scenario: Passwall refusals

- **WHEN** `passwall` is attempted in wild_mode, on a quest level, on a
 `NO_TELEPORT` level, or with `dir = 5`
- **THEN** it is refused

#### Scenario: Entombed in the wall

- **WHEN** `passwall` finds no landing point and was not cast with `safe`
- **THEN** it deals 10d8 damage with "becoming one with a wall"

- **Anchors**: `src/spells2.c:6559-6767` (the two swap pieces),
 `src/spells2.c:7860-7949` (passwall), `src/spells2.c:7773-7815` (wilderness and
 reality)

### Requirement: Curse Triggers And Summons

- `activate_ty_curse` SHALL run the `randint(27)` weighted chain - a 1/6 chance of
 chaining (the `break` fall-through implements the chain); the cases: five tiers
 of enragement, three tiers of `activate_hi_summon`, four tiers of summoning one
 monster, three tiers of losing `exp/16`, five tiers of paralysis
 (free_act+sav can avoid it, 3 or 13 rounds, `stop_ty`), three tiers of random
 stat drop, `lose_all_info` amnesia, a Cyberdemon below depth 65 (stop), and the
 default cyclic stat drops; the final roll continues the chain one time in three.
- `activate_dg_curse` SHALL run `randint(30)` - enragement, the re-curse
 `curse_equipment_dg(100, 50 * d2)`, the constant curse, losing `exp/12` plus a
 half chance of Black Breath, paralysis, hallucinations of ten small Morgoths
 (half stop), a permanent stat drop, amnesia, the weapon TR4_NEVER_BLOW three
 tiers, an evil Thunderlord below depth 25 (half stop), and the default cyclic
 stat drops; the final roll continues one time in four.
- `activate_hi_summon` SHALL draw `randint(9) + depth/40` monsters from the
 twenty-six-entry summon table by `randint(26) + depth/20`.
- `summon_cyber`/`summon_dragon_riders` SHALL summon `depth/50 + d6`
 HI_DEMON/THUNDERLORD (depth 100).
- The genocide family SHALL work as follows: `get_genocide_race` picks a monster
 and takes its `d_char`; `genocide_aux` deletes same-character monsters one by one
 (unique/quest monsters exempt; RF2_DEATH_ORB monsters scatter ten-grid copies of
 themselves and abort the run); with `player_cast` each monster costs `randint(4)`
 damage accumulated into `take_hit`; `mass_genocide` is limited to MAX_SIGHT with
 `randint(3)` damage per monster; `invoke` SHALL project directional damage at
 every monster on the level by `d_char` (gated by DF2_NO_GENO and DUNGEON_DEATH).
- `do_probe` SHALL print the hp (wizard or companion adds all stats/four strikes/
 target/exp), with `lore_do_probe`; `probing` probes every visible monster.

#### Scenario: Curse chains onward

- **WHEN** `activate_ty_curse` finishes its weighted case
- **THEN** the chain continues one time in three (with a 1/6 chance of chaining
 mid-chain)

#### Scenario: Death orb scatters

- **WHEN** `genocide_aux` reaches an `RF2_DEATH_ORB` monster
- **THEN** the monster scatters ten-grid copies of itself and the run aborts

- **Anchors**: `src/spells2.c:7136-7322` (the two curses),
 `src/spells2.c:7325-7411` (summons), `src/spells2.c:5084-5391` (genocide family
 and invoke), `src/spells2.c:5394-5474` (probing)

### Requirement: God-Given And Misc

- `bless_weapon` SHALL pick an item through the weapon hook - a cursed item is
 uncursed first (HEAVY fails one time in three, PERMA always fails), an already
 blessed item is refused, and a non-artifact item (or a one-third roll) sets
 TR3_BLESSED; otherwise the artifact resists and per 33% takes the second-stage
 demagic (to_h/to_d/to_a each lowered).
- `heal_insanity` SHALL add `csane` capped at `msane` with the four-tier wording.
- `reset_recall` SHALL go through the `reset_recall_aux` menu (lists visited
 dungeons without DF1_NO_RECALL; `@` matches a lowercase substring of the name
 plus the NO_RECALL and min_plev double check) to pick a dungeon, then
 `get_quantity` to pick a depth (capped at `max_dlv` when `no_trepas_max_depth`
 else at `maxdepth`, floored at `mindepth`, 99/100 forced to 98), and record
 `recall_dungeon` and `max_dlv`.
- `remove_dg_curse` SHALL clear TR4_DG_CURSE plus the dual-curse bits from the
 whole body.
- `create_between_gate` SHALL refuse on DF2_NO_TELEPORT; without coordinates it
 targets a destination (costing `60 - Conveyance skill` energy, validating an
 empty grid/ICKY/distance and a `rand_int(skill^2 / 2)` roll, failure scattering
 and costing 100 energy); both ends, when not permanent-wall grids, set mutually
 pointing FEAT_BETWEEN (the `special` field packs the coordinates).
- `call_chaos` SHALL draw the type from the thirty-entry element table, with a
 one-in-four `line_chaos`: one sixth omnidirectional (line means beam 75, else a
 ball of 75 radius two), one third a self-centered 300 ball of radius eight, and
 the rest a directional beam 150 or a ball of 150 with radius `3 + level/35`.

#### Scenario: Already blessed refused

- **WHEN** `bless_weapon` targets an already blessed item
- **THEN** the blessing is refused

#### Scenario: Between gate refused

- **WHEN** `create_between_gate` is attempted on a `DF2_NO_TELEPORT` level
- **THEN** it is refused

- **Anchors**: `src/spells2.c:7444-7581` (bless), `src/spells2.c:7818-7854`
 (sanity), `src/spells2.c:7954-8147` (recall reset),
 `src/spells2.c:8150-8167` (DG curse removal), `src/spells2.c:8172-8214`
 (between gate), `src/spells2.c:7077-7128` (`call_chaos`)
