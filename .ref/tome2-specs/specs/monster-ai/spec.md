# monster-ai Specification

## Purpose

Monster intelligence: `src/melee2.c` carries monster spell decision and execution (two
parallel sets, vs player and vs monster), smart spell learning and cheating, fear and
fleeing, movement direction decisions (surrounding, ambush, pack following,
possessor corpse-seeking, player-controlled), monster-vs-monster physical attacks,
per-monster state processing (bleeding, poisoned, sleep, stun, confused, fear),
door opening, wall bashing, item handling, and the per-turn monster main loop
`process_monsters`. The player-side physical attack (`make_attack_normal`) is
specified in specs/monster-melee/spec.md.

## Requirements

### Requirement: Monster Kills Monster And Fear

`mon_take_hit_mon` SHALL resolve one monster damaging another, including death, kill
experience, and fear rolls.

- Monsters with `RF7_NO_DEATH` cannot die.
- The target wakes (`csleep = 0`) and the damage is subtracted from its hp.
- When hp drops below 0, a unique whose status is not above `MSTATUS_NEUTRAL_P` and a
 `MFLAG_QUEST` monster are locked at hp 1 instead of dying.
- Otherwise the kill path runs: the death message is picked by corpse status — a
 `note` is appended to the name, an unseen kill is silent, a non-living monster gets
 "is destroyed.", a living monster gets "is killed." (non-living = demon, undead,
 stupid, nonliving flag, or a `d_char` in `Evg`).
- The killer gains experience `mexp x target level / killer level` (killer level 0
 counts as 1; the quotient has a floor of 1) via `monster_gain_exp`.
- With the `SKILL_LORE` skill and a pet killer, the player also gains a share:
 `mexp x level / max_plv` with the remainder carried in `exp_frac` at 0x10000 scale,
 then scaled by `get_skill_scale(SKILL_LORE, 120) / 100`, granted via `gain_exp`.
- A killed unique sets `max_num = 0` (stays dead); `monster_death` generates the drop
 and `delete_monster_idx` removes the corpse.
- Fear (under `ALLOW_FEAR`): pain cancels fear — a `randint(dam)` roll reduces
 `monfear`, clearing it entirely when the roll reaches it.
- A monster with no `monfear` and no `RF3_NO_FEAR` becomes afraid when (hp at 10% or
 less and `rand_int(10) < percentage`) or (`dam >= current hp` and
 `rand_int(100) < 80`); the new `monfear` is `randint(10) + ((dam >= current hp and
 percentage > 7) ? 20 : (11 - percentage) * 5)`.
- **Discrepancy:** the source comment claims monsters run "when hit for half its
 current hit points", but the code tests `dam >= m_ptr->hp` — the whole current hp.

#### Scenario: Wounded into fear

- **WHEN** a monster without `RF3_NO_FEAR` sits at 8% hp and takes a blow with
 `rand_int(10)` rolling under 8
- **THEN** it gains timed fear of `randint(10) + (11 - 8) * 5` and the caller's fear
 flag is set

- **Anchors**: `src/melee2.c:29-220`

### Requirement: Smart Learning And Spell Filtering

`int_outof` SHALL roll `rand_int(100) < prob`, with non-SMART monsters halving the
probability first. `remove_bad_spells` SHALL trim a monster's spell flags against what
it believes about the player.

- `RF2_STUPID` monsters are too stupid to learn anything and are returned unchanged.
- The filter runs only when the `smart_learn` or `smart_cheat` option is on.
- Under `smart_learn`, there is a 1-in-100 chance per call that all accumulated memory
 is wiped (`m_ptr->smart = 0`); the memorized `m_ptr->smart` bits are then the filter
 basis.
- Under `smart_cheat`, the monster reads the player's full current state and sets
 `SM_*` bits: the four elements in three tiers each (resist / oppose / immune),
 poison (resist / oppose), the eleven special resistances (nether, lite, dark, fear,
 conf, chaos, disenchant, blind, nexus, sound, shards), reflect (`SM_IMM_REFLECT`),
 free action (`SM_IMM_FREE`), and no-mana (`SM_IMM_MANA`).
- Filtering by memory: full immunity cuts the matching breath/ball/bolt with a 100%
 `int_outof` roll, resistance plus temporary opposition with 80%, a single resistance
 with 30%.
- Special resistances cut their matching spells at 50%; the fear/blind/conf resistance
 dispels cut at 100%.
- `SM_IMM_REFLECT` cuts all direct bolts and arrows; `SM_IMM_FREE` cuts HOLD and SLOW;
 `SM_IMM_MANA` cuts DRAIN_MANA.
- The category predicates `spell_attack` / `spell_escape` / `spell_annoy` /
 `spell_summon` / `spell_tactic` / `spell_haste` / `spell_heal` SHALL classify spells
 by their number ranges across the `96+N` / `128+N` / `160+N` segments.

#### Scenario: Stupid monster skips filtering

- **WHEN** `remove_bad_spells` runs on a `RF2_STUPID` monster
- **THEN** its spell flags are returned unchanged

#### Scenario: Immunity cuts a breath

- **WHEN** `smart_cheat` has revealed full immunity to an element the monster
 can breathe or ball
- **THEN** the matching spell is cut with a 100% `int_outof` roll

- **Anchors**: `src/melee2.c:261-268`, `src/melee2.c:275-548`, `src/melee2.c:645-759`

### Requirement: Spell Selection

`choose_attack_spell` SHALL pick a spell from the assembled list: STUPID monsters pick
uniformly at random; otherwise the spells are sorted into seven buckets (escape, heal,
tactic, annoy, haste, summon, attack) and the buckets are tried in the priority order
below.

- hp below one third or currently afraid: cast an escape spell if any.
- Still below one third (escape failed or unavailable): heal.
- Player within 4 grids and attack spells available: 75% chance to cast a tactic
 spell (blink away).
- hp below three quarters: 75% chance to heal.
- Summon: 50% chance.
- Attack: 85% chance.
- Tactic (second try): 50% chance.
- Haste: chance `20 + speed - mspeed` (rare when already fast).
- Annoy: 85% chance.
- Otherwise no spell is cast.

#### Scenario: Wounded monster escapes

- **WHEN** a monster's hp is below one third (or it is afraid) and it knows an
 escape spell
- **THEN** the escape bucket is tried first and the escape spell is cast

#### Scenario: Stupid monster picks at random

- **WHEN** a `RF2_STUPID` monster assembles a spell list
- **THEN** it picks one spell uniformly at random instead of the bucket
 priority order

- **Anchors**: `src/melee2.c:776-890`

### Requirement: Monster Casting On Monsters

`monst_spell_monst` SHALL execute the full `RF4`/`RF5`/`RF6` spell set against another
monster.

- The target is the `m_ptr->target` positive value; the spell is abandoned when the
 target is beyond `MAX_RANGE` or not projectable.
- The cast chance is `(freq_inate + freq_spell) / 2`; the global
 `monst_spell_monst_spell` can force a specific spell and bypass the roll.
- A SMART monster below 10% hp switches to the `*_INT_MASK` smart spell subset
 with 50% probability.
- Every case announces by visibility (`see_m` / `see_t` / both seen; unheard cases get
 sound-only messages).
- Breaths are scaled by current hp with hard caps: acid, lightning, fire, frost
 `hp/3` capped at 1600; poison gas `hp/3` capped at 800; nether `hp/6` capped at 550;
 light, darkness, confusion, sound, shards `hp/6` capped at 400; chaos `hp/6` capped
 at 600; disenchantment `hp/6` capped at 500; nexus `hp/3` capped at 250; time
 `hp/3` capped at 150; inertia `hp/6` and force `hp/6` capped at 200; gravity `hp/3`
 capped at 200; plasma `hp/6` capped at 150; mana `hp/3` capped at 250;
 disintegration `hp/3` capped at 300; toxic waste breath `hp/3` capped at 800.
- The radiation ball (`RF4_BA_NUKE`) is not hp-based: it deals `rlev + 10d6` with
 radius 2; the raw chaos invocation (`RF4_BA_CHAO`) deals `rlev x 2 + 10d10` with
 radius 4; the rocket deals `hp/4` capped at 800 with radius 2.
- The four arrow spells deal `1d6` / `3d6` / `5d6` / `7d6`; balls and bolts roll
 level-based dice.
- DRAIN_MANA only drains monsters that know spells (an "unaffected" message
 otherwise) and heals the caster by `6 x (randint(rlev) / 2 + 1)`.
- MIND_BLAST / BRAIN_SMASH: the target's level opposes `randint(rlev - 10, floored at
 1) + 10`; uniques and `RF3_NO_CONF` monsters always save, and an observed `NO_CONF`
 is memorized into `r_flags3`.
- The four CAUSE spells save with the same style of opposition.
- SCARE / BLIND / CONF / SLOW / HOLD oppose with the same style.
- HASTE raises speed, capped at `base + 10` in one jump, then in `+2` steps up to
 `base + 20`.
- HAND_DOOM, against a non-unique target, wins a level opposition and then cuts
 `65 + d25` percent of current hp (floored at 1).
- HEAL restores `rlev x 6` hp and cancels fear.
- TPORT and TELE_AWAY are gated by `DF2_NO_TELEPORT`; BLINK is not gated and blinks
 the caster 10 grids regardless.
- TELE_AWAY respects `RF3_RES_TELE`: a unique always resists and gets it memorized in
 `r_flags3`; other monsters resist when `level > randint(100)`.
- DARKNESS projects `GF_DARK_WEAK` and additionally calls `unlite_room`.
- The summon family routes friendly casters through `summon_specific_friendly` and
 hostile casters through `summon_specific`; the `S_HI_DEMON` case summons through
 `summon_cyber`, and `S_WRAITH` is always hostile, eight wraiths.
- TELE_TO, TELE_LEVEL, TRAPS, FORGET, ANIM_DEAD, and MULTIPLY are present as empty
 cases and are not implemented.
- `wake_up` clears the target's `csleep`.
- If the case was visible, the race memory `r_flags4/5/6` and the `r_cast_inate` /
 `r_cast_spell` counters are updated; a player death is always counted in `r_deaths`.

#### Scenario: Target out of range

- **WHEN** the chosen target monster is beyond `MAX_RANGE` or not projectable
- **THEN** the spell is abandoned

#### Scenario: Unique resists teleport away

- **WHEN** `TELE_AWAY` is cast at a unique monster
- **THEN** the unique always resists and the resistance is memorized into
 `r_flags3`

- **Anchors**: `src/melee2.c:1001-2778` (main function and all cases),
 `src/melee2.c:898-942` (breath/bolt projector helpers)

### Requirement: Equipment Cursing

`curse_equipment` and `curse_equipment_dg` SHALL curse one random piece of the
player's equipment.

- One random equipment slot is picked; if `randint(100)` exceeds `chance`, nothing
 happens.
- A `TR3_BLESSED` item gets an extra saving throw against 888; on success it
 announces that it resists cursing and is left alone.
- The heavy branch (roll within `heavy_chance` and the item is an artifact, ego item,
 or random artifact) sets `TR3_HEAVY_CURSE` plus `TR3_CURSED`; the normal branch sets
 `TR3_CURSED`. The `_dg` version adds `TR4_DG_CURSE` in both branches.
- A newly cursed item announces a malignant black aura and clears an existing
 "uncursed" inscription.

#### Scenario: Chance roll spares the equipment

- **WHEN** `randint(100)` exceeds `chance` in `curse_equipment`
- **THEN** nothing happens

#### Scenario: Blessed item resists

- **WHEN** the picked `TR3_BLESSED` item wins its saving throw against 888
- **THEN** the item announces that it resists cursing and is left alone

- **Anchors**: `src/melee2.c:2781-2893`

### Requirement: Monster Casting On The Player

`make_attack_spell` SHALL execute the spell set against the player.

- The target is the player (index 0).
- Confused and `MFLAG_NICE` monsters refuse to cast, as do friendly ones
 (`is_friend >= 0`); an `RF7_MORTAL` monster refuses while the player's
 `no_mortal` flag is set.
- The cast chance rolls like the vs-monster version; with the `stupid_monsters`
 option off, a second roll (innate spells survive when it comes in under
 `chance x 2`) can ban innate (breath) spells by clearing the whole `RF4` set.
- The player must be within `MAX_RANGE` and projectable.
- The dying-smart subset applies as in the vs-monster version; `remove_bad_spells`
 filters the list.
- Bolt spells require `clean_shot` (dropped when another monster blocks the line);
 summon spells require `summon_possible` (a free grid within two of the player).
- After assembly, `stupid_monsters` picks purely at random, otherwise
 `choose_attack_spell` picks.
- The failure rate is `25 - (rlev + 3) / 4` (always 0 for STUPID monsters; judged only
 for `RF5`/`RF6` spells, i.e. spell numbers >= 128); a failure plays
 "tries to cast a spell, but fails." and no spell executes.
- The player's antimagic field can interrupt a spell of number >= 128 when the
 monster is within `antimagic_dis` and `magik(antimagic)` succeeds.
- The execution cases mirror the vs-monster set; the differences are: the player side
 uses the `set_*` family with resistance and saving throws (`skill_sav`); SANITY
 effects go through `take_sanity_hit`; the CAUSE family additionally calls
 `curse_equipment(33/50/80, 0/5/15)`; HAND_DOOM cuts `65 + d25` percent of current
 hp directly (floored at 1) with `curse_equipment(100, 20)`; DRAIN_MANA drains the
 player's `csp` and heals the caster; TELE_TO pulls the player with
 `teleport_player_to`; DARKNESS uses `unlite_area(0, 3)`; FORGET goes through
 `lose_all_info`.
- Nearly every case feeds learning via `update_smart_learn(DRS_*)` by effect category.
- Race memory and `r_deaths` counting work as in the vs-monster version.

#### Scenario: Confused monster refuses to cast

- **WHEN** a confused monster's spell turn arrives
- **THEN** it refuses to cast and no spell goes off

#### Scenario: Cast failure keeps the message

- **WHEN** an `RF5`/`RF6` spell (number >= 128) fails its
 `25 - (rlev + 3) / 4` failure roll
- **THEN** the message "tries to cast a spell, but fails." plays and no spell
 is executed

- **Anchors**: `src/melee2.c:2950-4679` (main function and all cases),
 `src/melee2.c:633-639`, `src/melee2.c:898-910` (bolt/breath helpers)

### Requirement: Fleeing And Hiding Decisions

`mon_will_run` SHALL decide terror flight (under `ALLOW_TERROR`).

- A monster farther than `MAX_SIGHT + 5` from the player does not run.
- Monsters with `is_friend >= 0` (friendly and neutral alike) do not run.
- A monster with `monfear` always runs.
- Monsters at `cdis <= 5` never become terrified.
- The monster's power level is `level + (m_idx & 8) + 25`; extreme cases short-circuit
 (monster more than 4 above the player: no terror; player at least 4 above: terror).
- Otherwise terror is decided by crossing `(p_lev x p_mhp + p_chp x 4) / p_mhp`
 against `(m_lev x m_mhp + m_chp x 4) / m_mhp` (compared as cross-multiplied
 products).
- `find_safety` SHALL ring-scan distances 1 through 9 for the farthest floor grid the
 player cannot project to.
- `find_hiding` SHALL find a pack ambush spot: a floor grid invisible to the player
 that still gives the monster a `clean_shot`, at least `distance x 3 / 4 + 2` grids
 from the player.
- `get_fear_moves_aux` SHALL score fleeing detour candidates as
 `5000 / (distance-to-goal + 3) - 500 / (path cost + 1)` (clamped at 0), within the
 `MONSTER_FLOW` compile domain.

#### Scenario: Friend never runs

- **WHEN** `mon_will_run` evaluates a monster with `is_friend >= 0`
- **THEN** the monster does not run

#### Scenario: Fear overrides everything

- **WHEN** a monster within `MAX_SIGHT + 5` already has `monfear`
- **THEN** it always runs

- **Anchors**: `src/melee2.c:4695-4751`, `src/melee2.c:4956-5096`,
 `src/melee2.c:4862-4938`

### Requirement: Target And Movement Direction

`find_corpse`, `get_target_monster`, and `get_moves` SHALL resolve what a monster
moves toward.

- `find_corpse` finds a possessor the highest-level corpse within sight whose level
 is not above the possessor's own.
- `get_target_monster` picks the nearest `is_enemy` monster with a line of sight
 (`RF7_NO_TARGET` excluded); a hostile monster switches its target to the player when
 the player is nearer.
- `get_moves` decides in order:
 - a pet out of range follows the player;
 - a monster without a target uses the player's position;
 - while a doppleganger exists, 70% of all monsters (pets included) are lured toward
 the doppleganger;
 - a POSSESSOR steers toward corpses;
 - `RF7_AI_SPECIAL` hands the target to the `HOOK_MONSTER_AI` hook for rewriting;
 - a controlled monster advances by `control_dir` (the `AI_PLAYER` flag skips the
 roll, otherwise 85%);
 - an `AI_ANNOY` monster within 4 grids inverts its direction to tease;
 - a `DEATH_ORB` monster without line of sight does not move;
 - a pack monster (`FRIENDS` + `ANIMAL`, not a wall mover) uses `find_hiding` to
 lure the player out while the player stands outside a full room enclosure
 (fewer than 8 room grids around the player) and is above three quarters health;
 - `FRIENDS` monsters surround the target's adjacent grids, rotating assignments by
 `(m_idx + i) & 7`;
 - afraid monsters move directly away if stupid or pets, otherwise via `find_safety`
 (and through `get_fear_moves_aux` under `MONSTER_FLOW`);
 - the final step feeds `move_val` (diamond anti-drift: weights adjusted when
 `ay > 2ax` or `ax > 2ay`) into the direction table and produces the five-entry
 `mm` direction set.

#### Scenario: Doppleganger lure

- **WHEN** a doppleganger exists while `get_moves` resolves a direction
- **THEN** 70% of all monsters (pets included) are lured toward the
 doppleganger

#### Scenario: Death orb without sight

- **WHEN** a `DEATH_ORB` monster has no line of sight to its target
- **THEN** it does not move

- **Anchors**: `src/melee2.c:5100-5178` (corpses and target selection),
 `src/melee2.c:5183-5573` (get_moves)

### Requirement: Monster Melee On Monsters

`check_hit2` and `monst_attack_monst` SHALL resolve physical combat between monsters.

- `check_hit2`: a percentile roll under 10 forces the outcome (`k < 5` is a hit, 5-9
 is a miss); otherwise `randint(power + level x 3)` competes against `ac x 3 / 4`.
- `monst_attack_monst` refuses when the attacker has `RF1_NEVER_BLOW` or the target
 has `RF7_IM_MELEE`.
- Up to four blows are executed in turn; if the target changes position mid-sequence,
 the remaining blows are cut off.
- Each effect has a power tier for the hit roll (HURT/SHATTER/SANITY 60, UN_BONUS 20,
 and so on).
- On a hit, the message is picked by method (the `touched` flag marks the touch
 family).
- Damage is tripled (source comment: "need more punch against monsters").
- Effects map onto projection types for auras and side channels: poison/disease to
 `GF_POIS`, disenchanting to `GF_DISENCHANT`, the four elements map directly,
 confusion/hallucination to `GF_CONFUSION`, fear to `GF_TURN_ALL`, paralysis borrows
 `GF_OLD_SLEEP` (with damage replaced by the level), experience drain to
 `GF_NETHER`, time to `GF_TIME`.
- SHATTER with damage over 23, on a non-quest dungeon level (town excluded), triggers
 an earthquake of radius 8 centered on the attacker.
- HURT and SANITY damage are both reduced by the target's armor: the armor term is
 capped at 150 over a denominator of 250.
- After a touch, the target's fire or electric aura retaliates against the attacker
 for `(1 + level/26)d(1 + level/17)`.
- An `RBM_EXPLODE` blow destroys the attacker after it lands (via
 `mon_take_hit_mon` with damage of current hp + 1).
- A thief's successful steal triggers the flash-away half the time
 ("The thief flees laughing!").
- Visible blows are counted into `r_blows`.

#### Scenario: Target moves mid-sequence

- **WHEN** the target monster changes position between blows of
 `monst_attack_monst`
- **THEN** the remaining blows are cut off

#### Scenario: Shatter quake

- **WHEN** a SHATTER blow deals damage over 23 on a non-quest dungeon level
- **THEN** an earthquake of radius 8 triggers, centered on the attacker

- **Anchors**: `src/melee2.c:5576-6249`

### Requirement: Invisibility Perception

`player_invis` SHALL decide whether an invisible player is noticed: the player's
`invis` score opposes `randint(monster level x 2)`, succeeding when
`invis >= randint(mlv * 2)`.

- The monster's level is adjusted by race first: `RF3_NO_SLEEP` +10, `RF3_DRAGON` +20,
 `RF3_UNDEAD` +15, `RF3_DEMON` +15, `RF3_ANIMAL` +15, `RF3_ORC` -15,
 `RF3_TROLL` -10, `RF2_STUPID` halves it, `RF2_SMART` multiplies it by 5/4.
- Quest monsters, `RF2_INVISIBLE` races, and controlled monsters treat the player's
 invis score as 0.
- The adjusted level has a floor of 1.

#### Scenario: Invisibility spotted

- **WHEN** the player's `invis` score is at least `randint(mlv * 2)` for the
 checking monster
- **THEN** the invisible player is noticed

#### Scenario: Invisibility always works on some monsters

- **WHEN** the checking monster is a quest monster, an `RF2_INVISIBLE` race,
 or a controlled monster
- **THEN** the player's invis score is treated as 0

- **Anchors**: `src/melee2.c:6255-6294`

### Requirement: Per-Monster Behavior Processing

`process_monster` SHALL advance one monster's turn state and action, in this order.

- An `RF9_DOPPLEGANGER` monster is registered as the doppleganger.
- Bleeding costs `1 + maxhp/50` hp per turn (lethal, with " bleeds to death.").
- Poison deals one tenth of the monster's `poisoned` counter per turn (at least 1)
 as damage, with " dies of poison." on a kill.
- Sleep wake-up check: `notice = rand_int(1024)` cubed competes against `noise`
 (noise = `1 << (30 - stealth)`); the wake step is `100 / cdis` within 50 grids
 and 1 beyond (with `aggravate` the sleep is cleared in one turn);
 `r_ignore` / `r_wake` memorize the outcome.
- Stun recovers (a `rand_int(5000) <= level x level` roll clears it fully).
- Confusion and fear count down and recover.
- Friendly monsters (status above neutral, below companion, and not `RF7_PET`) turn
 hostile when the player has `aggravate`, and friendly uniques turn hostile outside
 wizard mode, both via `change_side` with the "suddenly becomes hostile!" message.
- An `RF4_MULTIPLY` monster calls `ai_multiply` while `num_repro` is below the cap.
- With the `speak_unique` option, a visible `CAN_SPEAK` monster speaks with 1/8
 probability (`HOOK_MON_SPEAK` hook first, then a `monspeak.txt` line, then the
 pet/fear/bravado line families).
- A missing target, or a 10% roll, re-selects the target.
- The monster then acts: `make_attack_spell` and `monst_spell_monst` (wrapped by
 `hack_message_pain_may_silent`).
- The move direction is chosen (fully random when confused or when the player is
 invisible to it; the `RAND_50`/`RAND_25` flags add probability-based randomness and
 are memorized when observed).
- Each direction is attempted in turn: open floor and monster traps are walkable.
- A GLYPH of warding is broken by a non-`NEVER_BLOW` monster with a
 `BREAK_GLYPH` roll against its level.
- `KILL_TREES` turns trees into grass.
- Perma-walls block movement.
- `CAN_LEVITATE` / `CAN_FLY` combined with `RF7_CAN_FLY` cross barriers.
- `CAN_PASS` walks through walls with `PASS_WALL`, or eats them with `KILL_WALL`
 (a 1-in-20 wall-grinding sound; the wall becomes floor).
- **Dead code:** the `CAN_PASS` + `PASS_WALL` branch is duplicated verbatim a second
 time; the second copy is unreachable.
- WEB is crossed by `RF7_SPIDER` monsters.
- Door handling: OPEN_DOOR unlocks (`rand_int(hp/10)` against the lock level; pets
 need the `pet_open_doors` option), BASH_DOOR breaks it (50% to `FEAT_BROKEN`,
 otherwise `FEAT_OPEN`, mimics cleared).
- A MINOR_GLYPH (explosive rune) is broken with a `randint(BREAK_MINOR_GLYPH) <
 level` roll; when the broken rune sits on the player's own grid it explodes with
 `fire_ball(GF_MANA, 2 x (player level/2 + 7d7), radius 2)`, otherwise the disarm is
 announced.
- A BETWEEN gate teleports the monster (endpoint scattered with `get_pos_player(10)`;
 monsters without `IM_COLD` pay `distance x 2` hp).
- Grid inscriptions with `INSCRIP_EXEC_MONST_WALK` execute on the step (a failed
 inscription blocks the move, except when stepping at the player).
- `NEVER_BLOW` monsters do not attack the player.
- A player in the way receives `make_attack_normal`, ending the turn.
- Pattern tiles block movement without action.
- Another monster in the way: `KILL_BODY` swallows it (higher `mexp`, non-unique,
 non-quest, not friendly-on-friendly, on floor), `is_enemy` or confused monsters
 fight via `monst_attack_monst`, `MOVE_BODY` shoves weaker monsters aside.
- Terrain passage is re-checked with `monster_can_cross_terrain`.
- `NEVER_MOVE` monsters stay in place.
- The move is executed: swapping with a placeholder monster (the placeholder moves
 back to the origin grid, is woken, `PU_MON_LITE`); `update_mon` on both sides;
 `lite_spot` on both grids; `INSCRIP_CHASM` gets its dedicated execution; visible
 moves disturb via `disturb_move`/`disturb_near` (pets need `disturb_pets`); monster
 traps trigger `mon_hit_trap`; floor objects — gold is ignored, a corpse plus a
 POSSESSOR goes to `ai_possessor`, `TAKE_ITEM`/`KILL_ITEM` handling (friends are
 exempt; objects whose slay flags match the monster's race are refused with a failure
 message; under the `testing_carry` option they are hooked into `hold_o_idx`,
 otherwise deleted); `HAS_LITE` refreshes the monster's light.
 - A hostile, unafraid monster that neither turned nor moved retries
 `make_attack_spell` once before the turn ends (only with the `stupid_monsters`
 option off).
 - Visible behaviors are recorded into eight `r_flags2` bits (open/bash door,
 take/kill item, move/kill body, pass/kill wall).
 - An afraid monster with nothing to do has its fear cleared and turns to fight
 ("turns to fight!").

#### Scenario: Bleeding drains the monster

- **WHEN** a bleeding monster's turn is processed
- **THEN** it loses `1 + maxhp/50` hp, which can kill it (" bleeds to death.")

#### Scenario: Explosive rune under the player

- **WHEN** a monster breaks a `MINOR_GLYPH` rune sitting on the player's own
 grid
- **THEN** `fire_ball(GF_MANA, 2 x (player level/2 + 7d7), radius 2)` explodes

- **Anchors**: `src/melee2.c:6322-7543`

### Requirement: Summon Maintenance And Main Loop

`summon_maint` and `process_monsters` SHALL maintain summoned monsters and drive the
per-turn monster loop.

- `summon_maint`: when `maintain_sum / 10000` exceeds the player's `csp`, control is
 lost ("You lose control of ...") and the summoned monster is deleted; otherwise the
 upkeep cost `cost = (level x 10000 / summon skill scale - 10000) / 4` is charged,
 with a floor of `ml x 19 / 990 + 80000 / 199` (`ml` = level x 10000, skill scale =
 `get_skill_scale(SKILL_SUMMON, 100)`).
- `process_monsters`:
 - an expired doppleganger is cleared;
 - all tracked monsters get a full lore snapshot taken;
 - `noise` is derived from the player's stealth;
 - the list is scanned from highest to lowest index — a `leaving` flag stops the
 scan;
 - `MSTATUS_PET` monsters accumulate `total_friends` and their total levels;
 - `MFLAG_BORN` monsters are skipped;
 - energy accumulates by `extract_energy[mspeed]`, one action per full 100;
 - monsters at `cdis >= 100` are skipped;
 - processing eligibility: controlled monsters always, `MFLAG_PARTIAL` always,
 `cdis <= aaf`, `cdis <= MAX_SIGHT` with line of sight or `aggravate`, and scent
 tracking under `MONSTER_FLOW`;
 - poisoned and bleeding monsters are always processed;
 - `hack_m_idx` is set and `process_monster` runs; a death or departure stops the
 scan;
 - half-summoned monsters are charged via `summon_maint`;
 - at the end, tracked monsters' lore snapshots are compared and any change triggers
 a `PW_MONSTER` redraw.

#### Scenario: Upkeep exceeds mana

- **WHEN** `maintain_sum / 10000` exceeds the player's `csp`
- **THEN** control is lost ("You lose control of ...") and the summoned monster
 is deleted

#### Scenario: Far monster skipped

- **WHEN** `process_monsters` reaches a monster at `cdis >= 100`
- **THEN** that monster is skipped this turn

- **Anchors**: `src/melee2.c:7546-7584` (maintenance), `src/melee2.c:7619-7837`
 (main loop)
