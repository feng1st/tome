# spell-effects Specification

## Purpose

The projection system: `src/spells1.c` carries the teleport family, the player
damage pipeline (`take_hit`/`take_sanity_hit`), elemental damage against players
and object destruction, stat operations and disenchantment, the projection path
calculation (`project_path` and the mana path), the project tetrad (`project_f`
terrain / `project_o` objects / `project_m` monsters / `project_p` player), the
`project` main function, the potion smash effects, and the random spell generation
(the Thaumaturgy/Power-mage side). The spell launch pieces (`fire_ball`/`fire_bolt`/
`lite_area` and friends) are documented under `src/spells2.c`; see
specs/spell-casting/spec.md.

## Requirements

### Requirement: Teleport Family

- `poly_r_idx` SHALL pick a shapechange target (uniques are refused; within 1000
 tries `get_mon_num((depth + level)/2 + 5)` draws a non-unique).
- `teleport_player_directed` SHALL scatter biased along the `dir` main axis
 (`min = rad/4` to start, doubling to relax; the landing point must be
 `cave_empty_bold`; stepping into a store runs `command_new = '_'` automatically;
 `tim_roots` forbids the move; a zero vector degrades to a normal teleport).
- `teleport_away` SHALL throw monsters (`resist_continuum` refuses; `min = dis/2`;
 GLYPH/MINOR_GLYPH and pattern floor tiles are refused; ICKY refused except on
 quest/arena levels; `dis` doubles and `min` halves per round, capped at a hundred
 MAX_TRIES).
- `teleport_to_player` SHALL pull a monster next to the player (the level roll
 `randint(100) > level` refuses; two grids to start; ICKY is not refused).
- `teleport_player` SHALL pass the resist_continuum / wild_mode / tim_roots /
 anti_tele / DF2_NO_TELEPORT five gates (`teleport_player_bypass` exempts
 globally), require `cave_naked_bold` plus no ICKY, give up after MAX_TRIES, and
 after the move scan the 3x3 around the old spot to pull over awake monsters with
 RF6_TPORT and without RES_TELE.
- `get_pos_player` only computes the point without moving.
- `teleport_monster_to`/`teleport_player_to` SHALL find a naked grid with
 `rand_spread` ring expansion (the former refuses the player's own grid; the
 anti_tele gate applies).
- `teleport_player_level` SHALL refuse through the quest / arena / NO_TELEPORT /
 NO_EASY_MOVE / DUNGEON_DEATH / anti_tele / resist_continuum / tim_roots gates;
 on the surface it descends, on the top level or a quest level it ascends,
 otherwise fifty-fifty - all through `autosave_l` plus leaving.
- `recall_player` SHALL, when inside a dungeon and not on the top level and not on
 a quest level, ask "Reset recall depth?" with the option to change `max_dlv`;
 with an empty `word_recall` it sets `rand_int(d) + f` and plays the charging
 message, with one set it clears to zero and cancels.

#### Scenario: Continuum resist refuses teleport

- **WHEN** `teleport_player` runs against any of its five gates
 (`resist_continuum` / wild_mode / `tim_roots` / `anti_tele` /
 `DF2_NO_TELEPORT`)
- **THEN** the teleport is refused

#### Scenario: Shapechange never picks a unique

- **WHEN** `poly_r_idx` draws a shapechange target
- **THEN** uniques are refused and within 1000 tries
 `get_mon_num((depth + level)/2 + 5)` yields a non-unique

- **Anchors**: `src/spells1.c:30-75` (shapechange), `src/spells1.c:83-192`
 (directed), `src/spells1.c:202-309` (away), `src/spells1.c:315-423` (to player),
 `src/spells1.c:435-590` (player teleport), `src/spells1.c:598-658` (point
 picking), `src/spells1.c:665-820` (the two targeted pieces),
 `src/spells1.c:827-939` (level change), `src/spells1.c:946-982` (recall)

### Requirement: Player Damage Pipeline

- `take_hit` SHALL work as follows: early return when already dead; `invuln`
 blocks damage (`dam < 9000` with a 1/13 penetration chance, otherwise no
 damage); `disrupt_shield` pays mana at double damage and, when the mana runs
 out, converts to half damage on hp; the symbiote (INVEN_CARRY) absorbs damage by
 the `5 + symbiosis skill` percentage (on its death its hp is subtracted from the
 remaining damage and `pval2` cleared); `chp` is reduced. The death branch:
 without AB_UNDEAD_FORM, or already in the undead state, the normal death process
 runs (the `death.txt` dying words, `died_from` plus "(?)", leaving,
 `total_winner` cleared, `death` set true, the HTML dump prompt); with
 AB_UNDEAD_FORM and not yet undead the character turns undead instead
 (`necro_extra` set, kills needed `lev +/- lev/4`, `calc_hitpoints` refills hp to
 full plus `wiz_cure_all`). Below a `hitpoint_warn` tier a bell rings with the
 warning (the undead state changes it to "LOW DEATHPOINT WARNING"). By hp
 percentage the spellbinder triggers (the HP 75/50/25 three tiers). Below 25% hp
 and worshipping Melkor, demon or undead reinforcements arrive with probability
 `grace/500 - 10` (grace above ten thousand takes the high tier).
- `take_sanity_hit` SHALL be the sanity-shaped twin - `csane` is reduced, and
 reaching zero means "You turn into an unthinking vegetable." death.

#### Scenario: Invulnerability blocks the blow

- **WHEN** `take_hit` runs while `invuln` is up with damage under 9000
- **THEN** the damage is blocked unless the 1/13 penetration roll hits

#### Scenario: Undead form cheats death

- **WHEN** a character with `AB_UNDEAD_FORM` takes lethal damage and is not yet
 undead
- **THEN** the character turns undead instead, with hp refilled to full

- **Anchors**: `src/spells1.c:1263-1275` (spellbinder trigger),
 `src/spells1.c:1287-1521` (`take_hit`), `src/spells1.c:1525-1601` (sanity)

### Requirement: Elemental Damage And Object Destruction

- The object hatred predicates SHALL be: `hates_acid` (missiles, bows, melee
 weapons, every armor piece including shields and dragon armor, staffs, scrolls,
 chests, skeletons, bottles, eggs), `hates_elec` (rings, wands, eggs),
 `hates_fire` (arrows, lights, bows, hafted and polearm weapons, soft armor,
 boots, gloves, cloaks, books, chests, staffs, scrolls, eggs), `hates_cold`
 (both potion families, flasks, bottles, eggs).
- `inven_damage` SHALL scan the pack's non-artifact items and roll destruction per
 item by `perc` (the All of/Some of/One of wording; potion destruction goes
 through `potion_smash_effect`; partially destroyed wand stacks lose charges by
 ratio).
- `minus_ac` SHALL randomly pick one of the six armor slots (with an item and
 `ac + to_a > 0`); IGNORE_ACID resists with the announcement, otherwise `to_a`
 drops by one and the call returns true.
- The four element damage pieces SHALL work as follows: immunity always blocks;
 resist and oppose each fold `(dam + 2)/3`; without resistance the HURT_CHANCE
 one-in-thirty-two stat drop applies (acid CHR, elec DEX, fire STR, cold STR;
 fire additionally doubled by `sensible_fire`); a `minus_ac` hit folds again by
 half; with both resists no object destruction runs, otherwise `inven_damage` by
 the damage tier (<30 one, <60 two, rest three).

#### Scenario: Immunity blocks the element

- **WHEN** one of the four element damage pieces hits an immune character
- **THEN** the damage is always blocked

#### Scenario: Double resist spares the pack

- **WHEN** an element with both resist and temporary opposition hits
- **THEN** no object destruction runs

- **Anchors**: `src/spells1.c:1614-1826` (hatred and destruction sets),
 `src/spells1.c:1842-1917` (`inven_damage`), `src/spells1.c:1929-2111`
 (`minus_ac` and the four elements)

### Requirement: Stat Operations

- `inc_stat` SHALL: below 18 add one with 75% else two; from 18/00 to 18/98 add
 `randint(gain) + gain/2` with `gain = (((18+100) - value)/2 + 3)/2` (the code
 comment calls this "1/6 to 1/3 of distance to 18/100"), capped at 18/99 there;
 beyond that per point; capped at 18/100 overall; `stat_max` is raised in sync.
- `dec_stat` SHALL fold by the `amount` percentage: the low segment subtracts
 through the 90/50/20 three thresholds; the high segment randomly amplifies a
 one-quarter loss base (at least `amount/2`); the floor is 3 (high values that
 would drop below 18 land on 18 when `amount <= 20`, else 17); STAT_DEC_PERMANENT additionally cuts `max`;
 STAT_DEC_TEMPORARY records `stat_cnt` (duration `rand_int(max_dlv * 50) + 50`)
 and `stat_los` for natural recovery.
- `res_stat` SHALL pull the value back to `max` and clear the temporary account in
 the full state, otherwise only restore `stat_los`.
- `apply_disenchant` SHALL pick one of the eight equipment slots at random
 (`mode` non-zero designates one); no positive bonus is immune; artifacts resist
 with 71%; otherwise to_h/to_d/to_a each drop one (above 5 another one, at twenty
 percent).
- `corrupt_player` SHALL swap two groups of random stat values.

#### Scenario: Stat gain capped

- **WHEN** `inc_stat` pushes a stat past 18/100
- **THEN** the value is capped at 18/100 with `stat_max` raised in sync

#### Scenario: Artifact resists disenchant

- **WHEN** `apply_disenchant` picks an artifact's equipment slot
- **THEN** the artifact resists with 71%

- **Anchors**: `src/spells1.c:2123-2378` (increase, decrease, restore),
 `src/spells1.c:2393-2488` (disenchant), `src/spells1.c:2491-2510` (corrupt)

### Requirement: Projection Path

- `yx_to_dir`/`invert_dir` SHALL convert between grids and directions.
- `get_mana_path_dir` SHALL walk equal-mana grids choosing among the four
 directions - with two exits it takes the one other than the arrival direction,
 with more it prefers continuing `pdir`, otherwise it draws at random excluding
 the arrival direction.
- `project_path` SHALL: with PROJECT_MANA_PATH take the mana path (`range + 10`
 cap, walls stop it, PROJECT_STOP stops it on a monster); otherwise take the
 Bresenham variant - three modes by main axis (vertical/horizontal/diagonal),
 the slope advanced in half/full fixed-point steps, `range` folded by `n + k/2`
 for the diagonal distance, stopping at the endpoint (PROJECT_THRU excepted), at
 non-initial walls (PROJECT_WALL excepted), and on a monster with PROJECT_STOP.

#### Scenario: Wall stops the mana path

- **WHEN** `project_path` walks the mana path and meets a wall
- **THEN** the path stops there (capped at `range + 10`)

#### Scenario: Stop flag halts on a monster

- **WHEN** a Bresenham projection path reaches a monster with `PROJECT_STOP`
- **THEN** the path stops at that monster

- **Anchors**: `src/spells1.c:2571-2666` (directions and mana path),
 `src/spells1.c:2709-2974` (`project_path`)

### Requirement: Terrain Effects project_f

`project_f` SHALL fold damage by distance as `(dam + r)/(r + 1)` and judge `seen`
by `player_can_see_bold`. The cases:

- COLD/ICE freezes normal floors twenty or fifty percent (FEAT_ICE; the pick
 tests `c_ptr->feat == GF_COLD` — a feat-versus-GF mix-up — which the
 plain-floor gate leaves dead, so fifty applies in practice);
- BETWEEN_GATE pairs the normal floor with a random distant normal floor into
 mutually pointing FEAT_BETWEEN (the `special` field packs the coordinates);
- the five fire cases: burning trees (FEAT_DEAD_TREE, deducting fifty or sixty
 Yavanna piety), melting ice (thirty percent, into dirt or shallow water),
 burning floors (twenty-five percent, into shallow magma or ash), glassifying
 sand walls (thirty percent);
- WATER/WAVE converts by the table when damage exceeds thirty (floor 35 shallow
 water plus 5 deep water, magma 15 to floor, deep magma drops to shallow plus
 floor, shallow water deepens at ten percent);
- NETHER/NEXUS/ACID/SHARDS/TIME/FORCE/NUKE destroy trees;
- DISINTEGRATE turns thirty percent into ash;
- KILL_TRAP removes traps and unlocks (secret doors become normal doors);
- KILL_DOOR destroys doors and removes traps;
- JAM_DOOR jams doors with spikes (adding 0x08 turns the door jammed, then
 further);
- KILL_WALL turns walls to mud (veins digging treasure with `place_gold`, rubble
 burying an object at ten percent with `place_object`, doors to floor);
- MAKE_DOOR/MAKE_TRAP/MAKE_GLYPH/STONE_WALL create their feature (bare floor
 only);
- WINDS_MANA adds or subtracts grid mana (`dam >= 256` absorbs, clears the
 coating and restores mana);
- LAVA_FLOW sets shallow or deep magma;
- LITE/DARK toggles CAVE_GLOW and refreshes the monsters in the grid;
- DESTRUCTION clears Room/Icky/Mark/Glow, deletes monsters and objects, rerolls
 walls/veins/floors by `t`, and turns the player's grid to flag with an
 after-burn/blind;
- the default goes to the `HOOK_GF_EXEC("grid")` hook.

#### Scenario: Cold freezes the floor

- **WHEN** a COLD/ICE case folds onto a normal floor and the 30-50% draw hits
- **THEN** the floor becomes `FEAT_ICE`

#### Scenario: Digging a treasure vein

- **WHEN** a KILL_WALL case turns a vein wall to mud
- **THEN** treasure is dug with `place_gold`

- **Anchors**: `src/spells1.c:3003-3869`

### Requirement: Object Effects project_o

`project_o` SHALL judge the objects of each grid's stack: CORPSE_EXPL makes corpses
explode (the maxroll/damroll hit points fold to a dam percentage, radius seven
folded the same, GF_SHARDS re-projected); acid/elec/fire/cold destroy by the
hatred predicates (IGNORE_* resists); PLASMA stacks fire plus elec, METEOR stacks
fire plus cold; ICE/SHARDS/FORCE/SOUND smash potions and flasks; MANA/DISINTEGRATE
always destroy, CHAOS always destroys (RES_CHAOS immune); HOLY/HELL_FIRE destroy
cursed non-artifact objects; KILL_TRAP/KILL_DOOR unlock chests and run
`object_known`; STAR_IDENTIFY/IDENTIFY identify fully or identify and run
HOOK_IDENTIFY plus the squelch; RAISE raises corpses (the `raise_ego` table draws
a random ego among skeleton/zombie/spectral/lich, a player casting yields a PET,
otherwise hostile); RAISE_DEMON summons demons by tier from the corpse monster's
level (Manes up to Nycadaemon, passing only when `randint(100)` is within the
level difference); the default goes to the `HOOK_GF_EXEC("object")` hook.

The destruction execution SHALL: announce only when marked; exempt artifacts and
ignore-flagged objects with the unaffected announcement; and run
`potion_smash_effect` when a potion is destroyed.

#### Scenario: Corpse explodes

- **WHEN** a CORPSE_EXPL projection reaches a grid stack holding a corpse
- **THEN** the corpse explodes, re-projecting `GF_SHARDS` with its radius folded
 by the dam percentage

#### Scenario: Artifact unaffected

- **WHEN** the destruction pass meets an artifact or an `IGNORE_*` object
- **THEN** it is exempt and the unaffected announcement plays

- **Anchors**: `src/spells1.c:3872-4288`

### Requirement: Monster Effects project_m

`project_m` SHALL: early-return on an empty grid, the projector itself, and dead
monsters; judge `seen` by `ml` with `who` not -100/-101; RF2_DEATH_ORB blocks
magic; monsters with the DEMON/UNDEAD/STUPID/NONLIVING/Evg characters use
"is destroyed." as their death wording.

Player casting at allies SHALL decide enmity per case - the friendly cases
(teleport/charm/heal/haste/darkness/door-jam/raising/identification and friends)
do not enrage; TRAP_DEMONSOUL against demons, KILL_WALL against HURT_ROCK,
HOLY_FIRE against the non-good, each dispel against its faction, PSI against
non-EMPTY_MIND, and POLY/CLONE at one-in-eight do enrage; an enraged ally goes
through `change_side` or turns NEUTRAL_M.

The case body SHALL cover about seventy tiers:

- the four element families (SUSCEP takes three times, IM one ninth, recording
 lore);
- POIS attaches `do_pois` (a one-in-four chance of `10 + d11 + r` folded by
 distance; double for the poison-vulnerable, immune clears);
- UNBREATH is immune against the non-living and the undead;
- NUKE mutates one third;
- HELL_FIRE doubles against EVIL;
- HOLY_FIRE: the good are immune, the evil doubled, the rest folded;
- PLASMA takes RES_PLAS; NETHER: the undead immune, RES folds, EVIL half;
 WATER/WAVE: the "W"-prefixed water spirits and the Unmaker immune, RES_WATE
 folds;
- WAVE/FORCE pushes monsters when the player casts (`do_move = 2`, direction by
 the vector, blocked adds half again the damage - `dam * 15 / 10` when no grid
 is gained, `13 / 10` when only one);
- CHAOS always polymorphs plus confuses, breathers resist;
- SHARDS bleeds at thirty-three percent; ROCKET half damage;
- SOUND stuns by a `(100 - lev)` roll when the player casts, breathers resist;
- CONFUSION/DISENCHANT/NEXUS each with their resist form;
- INERTIA/TIME/GRAVITY: breathers resist, others are slowed or resist (GRAVITY
 adds `do_dist = 10` and a `d(lev/10 + 3, dam)` stun);
- MANA/MISSILE/METEOR/ARROW plain damage;
- DISINTEGRATE doubles against HURT_ROCK, uniques resist at one-in-eight;
- FEAR is player self-damage oriented;
- PSI/PSI_DRAIN: the empty-minded immune, the stupid/odd-minded/beasts/
 high-resist folded to three; strong demons and strong undead bite back (a failed
 save hurts, PSI attaches a random state, PSI_DRAIN drains mana), and a PSI_DRAIN
 hit converts damage to mana;
- TELEKINESIS `do_dist = 7` plus stun;
- DOMINATION tames (`dam > 29` and a successful roll turns PET, Yavanna adds
 piety for good animals) or picks among stun/confusion/fear;
- ICE cold plus stun plus cutting;
- OLD_DRAIN is immune against undead, demons, and the non-living;
- DEATH_RAY: only an 888=666 roll or a failed resist blocks it, a hit sets
 `dam = level * 200`;
- OLD_POLY/OLD_CLONE/OLD_HEAL/OLD_SPEED/OLD_SLOW/OLD_SLEEP/STASIS shapechange,
 clone, heal, haste, slow, sleep, and suspension;
- CHARM/STAR_CHARM add `adj_con_fix[CHR]` and turn FRIEND/COMPANION (Yavanna adds
 piety);
- CONTROL_UNDEAD/CHARM_UNMOVING/CONTROL_ANIMAL/CONTROL_DEMON each tame their
 faction;
- OLD_CONF/STUN/CONF_DAM/STUN_DAM/IMPLOSION/STUN_CONF combine stun and confusion
 (IMPLOSION immune for the non-living);
- LITE_WEAK only hurts HURT_LITE; LITE/DARK each with their resist form (DARK
 resisted by ORC and HURT_LITE);
- KILL_WALL flays HURT_ROCK;
- AWAY_UNDEAD/EVIL/ALL teleport (RES_TELE and unique/the one-in-hundred roll
 resist, the DF2_NO_TELEPORT gate);
- TURN_UNDEAD/EVIL/ALL three fear families (`3d(dam/2) + 1`);
- DISP_UNDEAD/EVIL/GOOD/LIVING/DEMON/ALL six dispel families (the hit wording
 "shudders.", the death wording "dissolves!");
- RAISE heals monsters;
- TRAP_DEMONSOUL imprisons the demon (killing it drops a corpse with
 `pval3 = maxroll` hit points);
- the default goes to the eleven-value `HOOK_GF_EXEC("monster")` hook.

The wrap-up SHALL: refuse shapechange for uniques and quest monsters; cap a unique
not killed by the player at its hp, and likewise a quest monster; stack the
`do_pois`/`do_cut` states (poisoned/bleeding) with their wording; swap in
`note_dies` on death; run `do_poly_monster` when the `randint(90) > level` roll
hits; add half again the damage when a `do_move` push is blocked (`15 / 10`
fully blocked, `13 / 10` one grid); run `teleport_away` for `do_dist`; stack stun/conf/fear (capped at 200); route to
`mon_take_hit_mon` when `who > 0` else `mon_take_hit` (fear plays the fleeing
message); and `update_mon` plus `lite_spot` plus the recall window and tracking.

#### Scenario: Death orb blocks magic

- **WHEN** `project_m` reaches a monster with `RF2_DEATH_ORB`
- **THEN** the magic is blocked

#### Scenario: Susceptible monster takes triple

- **WHEN** an element family case hits a SUSCEP monster (an immune one instead)
- **THEN** the damage is tripled (one ninth)

- **Anchors**: `src/spells1.c:4291-4295` (hurt_monster),
 `src/spells1.c:4350-7162` (the full project_m cases), `src/spells1.c:30-75`
 (poly_r_idx), `src/spells1.c:9360-9412` (`do_poly_monster`)

### Requirement: Player Effects project_p

`project_p` SHALL: early-return on a non-player grid; exempt the player shooting
themselves when not unsafe; roll the Dodge skill against non-ball monster attacks
with `(dodge_chance - attacker level * 5/6) / 3`; reflect with REFLECT when not a
ball, not a 1/10 roll, and the source is not -100/-101 (the target drawn from the
attacker's neighborhood, re-projected with PROJECT_STOP|KILL); cap damage at 1600
then fold by distance; take the killer name by source (-99 a god, -100 the
dungeon, -101 the terrain, -2 a trap, otherwise the monster name with 0x88).

The cases SHALL cover: DEATH_RAY kills outright; the four elements go through the
`*_dam` pieces; POIS folds the resist, drops CON and poisons; NUKE folds the
resist plus poison plus mutation (1/5 shapechange or corruption, 1/6 destroys
acid-vulnerable objects); MISSILE/ARROW/MANA/DISINTEGRATE plain damage;
HOLY/HELL_FIRE currently always diffs to zero (the VALARIN/NETHER skill
difference is disabled by `#if 0`); PLASMA damages plus stuns plus destroys
acid-vulnerable objects; NETHER immunity/fold/exp drain (`hold_life` saves it 75%
of the time, otherwise loses `200 + exp/1000 * MON_DRAIN_LIFE`); WATER/WAVE stun
plus confusion plus destroys cold-vulnerable objects; CHAOS resist-fold plus
confusion plus hallucination plus exp loss plus destroys electric/fire-vulnerable
objects; SHARDS resist-fold plus cutting plus destroys cold-vulnerable objects;
SOUND resist-fold plus stun; CONFUSION resist-fold plus confusion; DISENCHANT
resist-fold plus disenchant; NEXUS resist-fold plus `apply_nexus` (three parts
teleport, two parts pull toward the monster, one part level change, one part
corruption, the last two savable); FORCE stun plus pushes the player (`swap_position`,
blocked adds damage); ROCKET stun plus cutting (RES_SHARD half damage);
INERTIA/Old_SLOW slows; LITE resist-fold plus blindness (`sensible_lite` triple
damage, `tim_wraith` force-breaks it); DARK resist-fold plus blindness
(`wraith_form` converts it to healing); TIME folds the resist with
`resist_continuum`, otherwise five parts exp loss, four parts one stat at 3/4,
one part all stats at 3/4; GRAVITY teleport plus slow plus stun (`ffall`
mitigates, `unsafe` makes dam the distance); OLD_HEAL/OLD_SPEED/OLD_SLEEP/STASIS/
RAISE/MAKE_GLYPH/PSI/IDENTIFY form the benefit group; the default goes to the
`HOOK_GF_EXEC("player")` hook. FORCE pushing and the disturb wrap-up close it out.

#### Scenario: Death ray lands

- **WHEN** a DEATH_RAY case reaches the player
- **THEN** the player is killed outright

#### Scenario: Bolt reflected

- **WHEN** the player has REFLECT, the attack is not a ball, the 1/10 roll
 misses, and the source is not -100/-101
- **THEN** the attack reflects at a target drawn from the attacker's
 neighborhood, re-projected with `PROJECT_STOP|KILL`

- **Anchors**: `src/spells1.c:7188-8192`

### Requirement: The project Main Function

`project` SHALL: pick the start point by the PROJECT_JUMP/player/monster three
states (-100/-101 skip the path); PROJECT_BEAM collects including the start;
`project_path` computes the path (capped at MAX_RANGE); advancing per grid - a
ball (`rad > 0`) explodes early on a wall, a beam collects, and when not blind and
not HIDE the `bolt_pict` draws (the `delay_factor` cubed delay, `fresh_before`
refresh); with the blast center at the endpoint the blast area is collected by
radius rings (DISINTEGRATE converts floors instead of the los check, the rest use
the los circle); the beam's last grid steps back; the blast animates per radius
(drawn inside-out, delayed, erased); then the four passes run - PROJECT_GRID
through `project_f` (with PROJECT_STAY a `new_effect` lingers and writes the grid
effect), PROJECT_ITEM through `project_o`, PROJECT_KILL through `project_m` (a
single grid with a REFLECTING monster re-projects at nine-in-ten, only when
`dist_hack > 1`; a player single-hitting a monster auto-recalls plus health
tracks), PROJECT_KILL again through `project_p`; the notice returns.

#### Scenario: Ball bursts on a wall

- **WHEN** a ball projection's path hits a wall before reaching its endpoint
- **THEN** it explodes early at that grid

#### Scenario: Reflecting monster bounces the bolt

- **WHEN** a single-grid projection meets a `REFLECTING` monster with
 `dist_hack > 1`
- **THEN** the projection re-projects at nine-in-ten

- **Anchors**: `src/spells1.c:8339-8855`

### Requirement: Potion Smash

`potion_smash_effect` SHALL: return true directly for the eleven harmless potions
and false for the thirty no-effect potions; map the rest to a projection by sval -
SLOWNESS slow, POIS poison 3, BLINDNESS darkness, CONFUSION confusion, SLEEP
sleep, RUINATION/DETONATIONS 25d25 shards, DEATH 10d10 magic radius one, SPEED
haste, the CURE family 2d3/4d3/6d3/10d10 healing, STAR_HEALING/LIFE 50d50 radius
one, RESTORE_MANA 10d10 magic radius one - all projected with
PROJECT_JUMP|ITEM|KILL, returning `angry` for identification.

#### Scenario: Harmless potion smash

- **WHEN** one of the eleven harmless potions smashes
- **THEN** the call returns true directly with no projection

#### Scenario: Detonations smash

- **WHEN** a RUINATION/DETONATIONS potion smashes
- **THEN** a 25d25 shards projection fires

- **Anchors**: `src/spells1.c:8877-9021`

### Requirement: Random Spell Generation

- `attack_types` SHALL be the twenty-five-entry element table and
 `destructive_attack_types` the ten-entry destruction table (KILL_WALL x3 /
 STONE_WALL x3 / DESTRUCTION x4).
- `generate_spell` SHALL set `dice = level/5 + power/5` (capped at 10),
 `sides = level*2 + power/2` (5-100), `mana = level * (1 + power/8)`; the form
 roll: level one or < 25 a bolt, < 50 a beam, < 76 a ball (`radius = dice`, dam
 becomes sides d1), < 83 a blast (`radius = sides/3`, into the destruction pool),
 < 90 a meteor shower (`radius = max(4, sides/3)`, into the destruction pool),
 the rest viewable; when destruction and simple overlap, one quarter goes to
 destruction; the GF comes from the matching pool.
- `name_spell` SHALL prefix Bolt/Beam/Ball/Blast/Area/View by the flags plus the
 `describe_attack_fully` element name; the desc records the damage parameters.
- `describe_attack_fully` SHALL be the thirty-four-tier GF wording mapping.

#### Scenario: Dice ceiling

- **WHEN** `generate_spell` computes dice above ten
- **THEN** the dice count is capped at 10

#### Scenario: Low-level spell is a bolt

- **WHEN** the form roll lands for a spell at level one or below 25
- **THEN** the generated form is a bolt

- **Anchors**: `src/spells1.c:9024-9355` (generation),
 `src/spells1.c:9072-9183` (wording)
