# object-gen Specification

## Purpose

Object generation and operations: `src/object2.c` carries object list management
(compaction/reclamation), allocation-table draws (theme/legal/good hooks), value
computation, stacking decisions, template preparation and enchantment (the full
`apply_magic` family), artifact and ego generation, floor placement and drops,
inventory carry/takeoff/drop/combine/reorder, and corpse decay. Player-side wield
action commands are recorded under interface-commands; randart forging
(`create_artifact`) is recorded with the randart capability.

## Requirements

### Requirement: Object List Management

The list helpers SHALL: `calc_total_weight` sum the carried weight of the whole body;
`excise_object_idx` remove an object from a monster stack or a grid stack;
`delete_object_idx` remove, then `lite_spot`, then `object_wipe`, then decrement
o_cnt; `delete_object` clear one grid's whole stack; `compact_objects_aux` move i1 to
i2 and repair all next pointers plus the grid/monster head pointers.

- `compact_objects` SHALL delete objects under escalating pressure per level — as
 cur_lev rises, the distance protection 12*(101-cur_lev)/100 shrinks; monster-held
 objects use chance 100 and floor objects 90, each reduced further by cur_lev/2;
 artifacts are immune while cur_lev < 300 + object level (400+ for `TR3_NORM_ART`);
 past that they get chance = -1 (certain deletion), with cur_lev rolled back to
 prevent consecutive deletes;
 afterwards holes are compacted from tail to head, shrinking o_max.
- `wipe_o_list` SHALL clear the whole list (in preserve mode, or when nothing was
 generated yet, unknown artifact markers are rolled back across the
 cur_num/generated/artifact triple); o_max=1, o_cnt=0.
- `o_pop` SHALL extend o_max first, then reclaim dead slots; when both are exhausted
 it announces "Too many objects!" and returns zero.

#### Scenario: Artifact survives compaction

- **WHEN** `compact_objects` evaluates an artifact while
 `cur_lev < 300 + object level` (400+ for `TR3_NORM_ART`)
- **THEN** the artifact is immune (the chance = -1 assignment applies only once
 immunity has expired)

#### Scenario: Object slots exhausted

- **WHEN** `o_pop` finds both the `o_max` extension and the dead-slot reclaim
 exhausted
- **THEN** "Too many objects!" is announced and zero is returned

- **Anchors**: `src/object2.c:70-81` (weight), `src/object2.c:86-285`
 (excise/delete), `src/object2.c:291-504` (compaction), `src/object2.c:520-591`
 (wipe), `src/object2.c:600-646` (o_pop)

### Requirement: Allocation Table Draw

`get_obj_num_prep` SHALL filter `alloc_kind_table`'s prob2 through
`get_obj_num_hook` (legal entries copy prob1, otherwise zero).

- `get_obj_num` SHALL: at positive depth, a GREAT_OBJ hit raises level to
 1+level*MAX_DEPTH/randint(MAX_DEPTH); it accumulates prob2 into prob3 in depth
 order for the draw (chests excluded while `opening_chest`); afterwards a redraw
 runs when the 0-99 roll is under 60 and a second when it is under 10, each keeping
 the deeper result.
- **Discrepancy:** the in-code comment claims the first redraw triggers at 50%,
 but the test is `p < 60` (60%).
- The theme machinery SHALL: the four `obj_theme` components (treasure/combat/magic/
 tools) are stored globally by `init_match_theme`; `theme_changed` invalidates the
 `alloc_kind_table_valid` cache on change; `kind_is_theme` rolls the per-tval
 probabilities of the four components (skeleton/bottle/junk/corpse/egg use 100
 minus the four-way sum; a zero theme lets everything through).
- The eligibility hooks SHALL: `kind_is_legal` — the theme passes; `TR4_SPECIAL_GENE`
 needs `k_allow_special`; `TR3_NORM_ART` rejects when already generated; for
 corpses the four skull/skeleton/head/corpse svals reject; `TV_HYPNOS` rejects; the
 Nazgul ring `SV_RING_SPECIAL` rejects; `kind_is_legal_special` can force a single
 tval.
- `kind_is_good` — armor needs to_a >= 0; weapons need nonnegative to-hit and
 to-dam; arrows and bolts always pass (shots do not); main rods need sval at least
 `SV_ROD_SILVER` and rod tips cost 4500 or above; good books (sval within
 `SV_BOOK_MAX_GOOD`); the speed ring and seven good amulets.
- `kind_is_artifactable` — `kind_is_good` and at least one tval/sval range match in
 `ra_info`.

#### Scenario: Great object depth boost

- **WHEN** `get_obj_num` draws at positive depth and a `GREAT_OBJ` hits
- **THEN** the level is raised to `1+level*MAX_DEPTH/randint(MAX_DEPTH)`

#### Scenario: Chests excluded while opening

- **WHEN** the draw runs with `opening_chest` set
- **THEN** chests are excluded from the accumulated table

- **Anchors**: `src/object2.c:653-680` (prep), `src/object2.c:699-818`
 (get_obj_num), `src/object2.c:4420-4639` (theme), `src/object2.c:4644-4784`
 (eligibility hooks), `src/object2.c:4789-4815` (artifactable)

### Requirement: Knowledge And Value

The knowledge helpers SHALL: `object_known` set `IDENT_KNOWN` and clear
SENSE/EMPTY; `object_aware`/`object_tried` record the k_info global bits.

- `object_value_base` SHALL estimate the unidentified value — aware uses the template
 cost (eggs excepted), otherwise the tval table (food 5, potion/scroll 20, staff 70,
 wand 50, rod tip 90, ring/amulet 45, eggs monster level*100+100).
- `flag_cost` SHALL accumulate per-flag prices (`TR5_TEMPORARY` or
 `TR4_CURSE_NO_DROP` returns zero for the whole object; WRAITH/PRECOGNITION 250000;
 ESP 12500 per bit; negative-value flags
 TY_CURSE/AGGRAVATE/DRAIN_EXP/Black Breath/DG curse/CLONE/NEVER_BLOW; the random
 artifact activation tier adds the ACT_* price list of some seventy entries).
- `object_value_real` SHALL — RANDART uses the registered price; a zero-price base is
 zero; `art_flags` with values add flag_cost(pval); artifacts swap in the a-value;
 egos add the e-value (name2b adds again); SPELL_CONTAIN adds 5000+500*spell level;
 the pval family prices per flag (SPEED 30000 per point, and so on); tval specifics —
 eggs add monster level*100; staffs multiply by spell level and by the two-level
 mean, folded by 6, plus the pval/20 charge price (staff not divided by number);
 spellbooks with sval 255 multiply by spell level; `TV_ROD_MAIN` adds the rod tip
 price; ring/amulet/armor/weapon with negative bonuses and no base price are zero
 and otherwise use (to_h+to_d+to_a)*100; ammo is x5; dd above base adds ds*100 per
 extra die (ammo x5); exploding ammo is x14.
- `object_value` SHALL — identified cursed is zero; unidentified but felt-cursed is
 zero; the discount scales the price down by the discount percentage.

#### Scenario: Identified cursed is worthless

- **WHEN** `object_value` evaluates an identified cursed item
- **THEN** the value is zero

#### Scenario: Felt curse also worthless

- **WHEN** `object_value` evaluates an unidentified item felt to be cursed
- **THEN** the value is zero

- **Anchors**: `src/object2.c:842-949` (knowledge and base value),
 `src/object2.c:952-1163` (flag_cost), `src/object2.c:1188-1492` (real),
 `src/object2.c:1506-1538` (value)

### Requirement: Stacking And Absorption

`object_similar` SHALL (same k_idx, and neither side carries SPELL_CONTAIN) apply the
per-tval rules:

- spellbooks need both identified and matching artifact/ego identity (random books
 need the same pval);
- chests, RANDART, instruments, symbiotes, eggs, corpses, `TV_ROD_MAIN` never stack;
- RUNE1 always stacks; RUNE2 stone tablets do not;
- totems need identical pval/pval2; potions need the same pval2; scrolls the same
 pval and pval2;
- staffs need both identified or both EMPTY, the same charges, the same RECHARGED
 state, the same spell, the same level group, the same name1/name2/name2b;
- wands need both identified or both EMPTY, the same name1, the same
 RECHARGED/spell/level/ego;
- weapons and armor need the `stack_allow_items` option, then fall through into the
 rings/amulets/lites checks and the ammo checks below; rings/amulets/lights need
 both identified plus the same timeout; ammo needs the same knowledge state and the
 same to_h/to_d/to_a/pval/pval2/name1/name2/name2b;
- xtra1 nonzero on either side prevents stacking; timeout nonzero on either side
 (lites excepted) prevents stacking; same ac/dd/ds;
- food/essences need the same pval2 (quest hack); the default rule needs both
 identified.

The tail common rules SHALL: art_flags1-3 all equal; `IDENT_CURSED` in the same
state; inscriptions (when both present they must match; with `stack_force_notes` off
they always match); discount (with `stack_force_costs` off, always matching); the
combined count within MAX_STACK_SIZE.

`object_absorb` SHALL merge counts capped at MAX_STACK_SIZE-1, merge the
known/MENTAL/note/best-discount/STOREB clearing logic, and add wand charges
(`TV_WAND` only).

#### Scenario: Chests never stack

- **WHEN** `object_similar` compares two chest objects
- **THEN** they never stack

#### Scenario: Absorb caps the count

- **WHEN** `object_absorb` merges two stacks of the same kind
- **THEN** the combined count is capped at `MAX_STACK_SIZE-1`

- **Anchors**: `src/object2.c:1562-1897` (similar), `src/object2.c:1903-1936`
 (absorb)

### Requirement: Template Preparation And Random Bonuses

The preparation helpers SHALL: `lookup_kind` search tval/sval linearly (wizard mode
reports a miss); `object_wipe`/`object_copy` perform the structural operations.

- `object_prep` SHALL take from k_info: tval/sval/pval/pval2/number=1/weight/to_h/
 to_d/to_a/ac/dd/ds; `TR3_CURSED` sets `IDENT_CURSED`; for `TR4_LEVELS` the initial
 values are elevel = level/10+1, exp = the matching player_exp tier, pval2=1,
 pval3=0.
- `m_bonus` SHALL sample randnor with mean max*level/MAX_DEPTH (remainder carried
 up) and standard deviation max/4 (remainder carried up), clamped to 0-max.
- `finalize_randart` SHALL draw the slot with `rand_int(MAX_RANDARTS)` until an
 ungenerated one comes up (after 2000 tries it accepts even a generated slot),
 set sval, the pval2 activation number and the xtra2 activation spell, and
 record level and generated.
- **Dead code:** `finalize_randart` computes `foo = lev + randnor(0, 5)` clamped
 to 1-100 but never uses it for the pick.
- `random_artifact_resistance` SHALL give `random_resistance` per the artifact
 name-list tier (fixed resist / resist or ability / ability / both, four tiers;
 `give_power` sets xtra1 = `EGO_XTRA_ABILITY` plus xtra2 = randint(256);
 artifact_bias is cleared to zero).

#### Scenario: Cursed kind records the sense

- **WHEN** `object_prep` prepares a kind with `TR3_CURSED`
- **THEN** `IDENT_CURSED` is set

#### Scenario: Bonus clamped to the cap

- **WHEN** `m_bonus` samples past its `max` argument
- **THEN** the result is clamped into the 0-max range

- **Anchors**: `src/object2.c:1943-2032` (prep), `src/object2.c:2074-2114`
 (m_bonus), `src/object2.c:2122-2154` (randart finish), `src/object2.c:2197-2277`
 (resistance)

### Requirement: Artifact And Ego Generation

`make_artifact_special` SHALL scan a_info for `TR3_INSTA_ART` entries not yet
generated (failing outright at depth 0, the town) — lenient depth (out-of-depth rolls at x2 against), the rarity roll
(rarity adjusted by luck across a +/-half span), the base object depth roll against
x5, and `TR4_SPECIAL_GENE` needing `a_allow_special` (`vanilla_town` exempt) — and on
a hit run `object_prep` plus name1 and the LEVELS initial values.

- `make_artifact` SHALL take non-insta artifacts by the same rules plus a matching
 tval/sval and the one-piece limit, and on a hit add `random_artifact_resistance`.
- `make_ego_item` SHALL: reject an object that is already an artifact or already has
 name2; scan e_info for tval/sval range matches, sided good/bad by cost, needing all
 six need_flags groups and no overlap with the six forbid groups — these form the
 candidate list; roll depth per candidate across candidate-count x10 attempts
 (out-of-depth rolled against by the difference) and rarity (discarded when the
 roll > rarity after mrarity is adjusted by luck); with a 7+luck percent chance try
 a second ego — it must be on the opposite side (complementary before) and a
 different entry, and never overwrite an existing name2b; on success set name2b.

#### Scenario: Already-enchanted object refused

- **WHEN** `make_ego_item` receives an object that is already an artifact or
 already has `name2`
- **THEN** the ego is rejected

#### Scenario: Second ego must complement

- **WHEN** the second-ego try draws an entry not on the opposite side or equal
 to the first
- **THEN** it is refused and an existing `name2b` is never overwritten

- **Anchors**: `src/object2.c:2288-2362` (special), `src/object2.c:2372-2440`
 (normal), `src/object2.c:2447-2595` (ego and double ego)

### Requirement: Enchantment Main Flow

`apply_magic` SHALL:

- compute lev plus luck +/-7; for a SPELL_CONTAIN base, set pval2=-1 first; the six
 obvious groups come down from k_info into art_oflags*;
- run the `TR3_NORM_ART` special path — if already generated, or `SPECIAL_GENE` not
 allowed, re-prep into a normal btval/bsval object ("We've been tricked!", wizard
 mode only);
 otherwise staffs/wands get the spell level group plus charges, k_info is marked as
 artifact, the cheat announce plays, and the function returns;
- roll good chance f1 = lev+10+luck(+/-15) capped at 75 and great f2 = f1/2 capped at
 20, then the power roll spans -2 (broken) through +2 (great)
 (`hack_apply_magic_power` can force; -99 zeroes it);
- `rolls` is 1 when power is at least 2, forced to 4 when great, and zeroed when not
 okay or the object already has name1; `make_artifact` is tried `rolls` times;
- downgrade The One Ring to an invisible ring while `QUEST_ONE` is not taken;
- on the artifact hit path — cur_num=1, all fields from a_info, cursed, rating+10
 (a price over 50000 adds another +10), good_item_flag, cheat announce, LEVELS
 initial values, then return;
- otherwise dispatch by tval to a_m_aux_1/2/3/4 (DAEMON_BOOK routes to the weapon or
 armor path by sval; ring/amulet without power has 50% chance forced to -1);
- art_name adds rating+40.

The ego path SHALL process name2 and name2b, each through five tiers — a rar[j] hit
merges the six flag groups and the six obviously groups and calls
`add_random_ego_flag` (fego); with LIMIT_BLOWS a BLOWS pval above 2 is reduced to
randint(2); `TR3_CURSED` records ident; to_h/to_d/to_a/pval add or subtract by the
max_* sign; rating adds the ego's rating field; MSTAFF_SPELL removes SPELL_CONTAIN,
otherwise pval2=-1.

The base finishing path — cursed recorded, LEVELS initial values, SPELL_CONTAIN
cleared, MSTAFF negative pval zeroed, `TV_ROD_MAIN` sets pval2 = capacity (x2 with
`TR4_CAPACITY`) plus a full timeout, trapkit passes `trap_hack`.

- `add_random_ego_flag` SHALL implement, by fego bit: SUSTAIN/OLD_RESIST/ABILITY/
 R_ELEM/R_LOW/R_HIGH/R_ANY/R_DRAGON (`dragon_resist` loops with a half chance to
 continue)/SLAY_WEAP (1/3 dd*2, otherwise the dd/ds increment chain; 1/5 poison
 brand; swords 1/3 VORPAL)/DAM_DIE/DAM_SIZE/LIMIT_BLOWS toggle/PVAL/AC/TH/TD
 M1/M2/M3/M5 (no M4 exists)/R_P_ABILITY/R_STAT/R_STAT_SUST/R_IMMUNITY
 (with IGNORE).
- `a_m_aux_1` SHALL weapons — tohit1/todam1 = randint(5)+m_bonus(5), tohit2/todam2 =
 m_bonus(10); with power>1 a `RANDART_WEAPON` hit forges a randart (trapkit exempt),
 otherwise `make_ego` good; power<-1 goes to the reversed ego; +/- first/second tier
 adjustments; a negative sum records `IDENT_CURSED`; trapkit switches to to_a
 adjustment; MSTAFF — with `EGO_MSTAFF_SPELL` installs the double rune (pval two
 GFs, pval3 two MODs, pval2 two mana values each randint(70)), otherwise sets
 SPELL_CONTAIN+WIELD_CAST; power==1 without ego gives ammo a 30% chance of an
 exploding GF (a 27-entry table); the `HOOK_APPLY_MAGIC` hook takes priority.
- `a_m_aux_2` SHALL armor — the toac single and dual segments share one shape;
 ELVEN_CLOAK pval=randint(4); MIMIC_CLOAK goes through Lua `find_random_mimic_shape`;
 dragon armor rating+30; dragon shield/helm rating+5 plus `dragon_resist`.
- `a_m_aux_3` SHALL rings/amulets by sval specifics (attacks 1-3, crit 1-10, stat
 1+5, speed d5+5 then 50% chained increments with no cap, lordly repeated
 dragon-style resist rolls plus to_a = 10+randint(5)+m_bonus(10) and rating+5,
 the weak/foolish/doom ones always cursed, the damage/accuracy/
 protection/slaying segments each reversible under power<0, and the trickery/
 devotion/weaponmastery/brilliance/charisma/wisdom/infra/serpent/no_magic/no_tele/
 resistance/searching/magi/doom segments).
- `a_m_aux_4` SHALL misc — random books 75% MAGIC 25% SPIRITUALITY through Lua
 `get_random_spell`; lites add a random fuel time; corpses take the current level's
 non-unique monster (unique folds to #2, the scrawny cat); eggs take a HAS_EGG
 monster (after 1000 misses fold to #940, the Blue Firebird — **Discrepancy:**
 the in-code comment says "Blue fire-lizard", but r_info #940 is the Blue
 Firebird) and set weight and
 hatch pval (**Quirk:** a HAS_EGG hit on exactly the 1000th attempt is still
 overridden by #940, since the fold tests the count, not the success flag); symbiotes take a non NEVER_MOVE monster and record maxroll blood;
 wands/staffs pick a spell through Lua `get_random_stick` (wands default to
 Manathrust, staffs to Globe of Light) with `get_stick_base_level`/`get_stick_max_level` filling the
 levels plus `charge_stick`; chests get `place_trap_object` plus pval2 =
 (sval % SV_CHEST_MIN_LARGE)*2 as the object count, skipping ruined chests (kind
 level <= 0) and zero pval;
 blood vials rating+25; mimic potions take a mimic shape; dragon horns set the
 four-element GF.

#### Scenario: Tricked norm-art

- **WHEN** `apply_magic` runs the `TR3_NORM_ART` path on a base that is already
 generated, or while `SPECIAL_GENE` is not allowed
- **THEN** the object re-preps into a normal btval/bsval object with
 "We've been tricked!"

#### Scenario: One Ring downgraded

- **WHEN** The One Ring comes up while `QUEST_ONE` is not taken
- **THEN** it is downgraded to an invisible ring

- **Anchors**: `src/object2.c:4009-4416` (apply_magic main), `src/object2.c:2613-2742`
 (aux1), `src/object2.c:2745-2873` (aux2 and dragon resist),
 `src/object2.c:2885-3292` (aux3), `src/object2.c:3300-3571` (aux4 and trap_hack),
 `src/object2.c:3574-3976` (ego flags)

### Requirement: Placement And Drops

The placement helpers SHALL: `make_object` — with good, invprob = 10-luck(+/-9) rolls
`make_artifact_special`; a theme change invalidates the cache; good attaches
`kind_is_good`, preps, draws, then restores `kind_is_legal` and marks the cache
valid; failure returns false; then `apply_magic(object_level, TRUE, good, great)`;
spikes, shot and ammo get number=6d7; artifacts are one piece; un-cursed out-of-depth
objects add the depth difference to rating plus the cheat announce.

- `place_object` SHALL require `in_bounds` and `cave_clean_bold`; after
 `make_object` it records the found source by `where` (VAULT/FLOOR/SPECIAL/RUBBLE);
 on `o_pop` success it enters the grid stack plus `note_spot`/`lite_spot`; on
 overflow it rolls back the generation markers across the artifact/normal/randart
 triple.
- `make_gold` SHALL take the kind i=(randint(level+2)+2)/2-1, GREAT_OBJ adds
 randint(level+1), `coin_type` forces, `object_prep(OBJ_GOLD_LIST+i)`, pval =
 base+8*randint(base)+randint(8); `place_gold` shares the `place_object` skeleton.
- `drop_near` SHALL: exempt artifacts from the chance annihilation roll; scan the
 three surrounding rings scoring s = 1000 - (dy^2+dx^2 + k*5) (k the object count
 at the landing point; mergeable objects not counted); the `testing_stack` option
 allows
 multiple-object grids and takes a random best-scoring spot; with nowhere to land
 and not an artifact, annihilate; artifacts instead bounce within 1000 tries via
 `rand_spread`, after which the whole map is searched for one of five floor types;
 the landing point first tries `object_similar` merging, else `o_pop` (overflow
 annihilates and rolls back the artifact markers); with a nonzero chance and a
 landing under the player, announce "You feel something roll beneath your feet.".
- `acquirement` SHALL produce num objects via `make_object(TRUE, great)` then
 `drop_near` (fully identifying when known).
- `pick_trap` SHALL set CAVE_TRDT plus refresh the display.
- `floor_carry` SHALL be the floor version of merging or placing (stack capacity
 capped at 23).

#### Scenario: Drop rolls beneath the feet

- **WHEN** `drop_near` lands an object under the player with a nonzero
 chance
- **THEN** "You feel something roll beneath your feet." is announced

#### Scenario: Nowhere to land

- **WHEN** a dropped non-artifact finds no landing spot in the three rings
- **THEN** it is annihilated

- **Anchors**: `src/object2.c:4832-4936` (make_object), `src/object2.c:4949-5047`
 (place), `src/object2.c:5053-5162` (gold), `src/object2.c:5181-5482` (drop_near),
 `src/object2.c:5490-5541` (acquirement and pick_trap), `src/object2.c:6392-6465`
 (floor_carry)

### Requirement: Inventory And Floor Operation Helpers

The list operation helpers SHALL: `inven_item_charges`/`floor_item_charges` report
charges for known staffs/wands; `inven_item_describe`/`floor_item_describe` report
the description; `inven_item_increase`/`floor_item_increase` adjust the count (capped
at 255; the inventory side triggers PU_BONUS/PU_MANA/PN_COMBINE).

- `inven_item_optimize` SHALL clean up zero-count slots — the inventory side shifts
 everything forward and decrements inven_cnt; the equipment side decrements
 equip_cnt plus `takeoff_set` set detach plus PU_BONUS/PU_TORCH/PU_MANA;
 `floor_item_optimize` deletes directly.

The carry helpers SHALL: `inven_carry_okay` return true with a free slot or a
mergeable match (gold always false); `inven_carry` SHALL first try merging across
the whole pack (`object_similar` plus `object_absorb`), otherwise take a free slot
with presorted insertion — the sort key is tval descending, aware first, sval
ascending, known first, `TV_ROD_MAIN` by descending timeout (larger remaining
recharge first), `object_value` descending — everything after the insertion point shifts back, floor
fields are
cleared, inven_cnt increments, PN_COMBINE/PN_REORDER.

- **Discrepancy:** the rod sort comment in `inven_carry`/`reorder_pack` claims
 "increasing recharge time", but the comparisons place larger timeouts first
 (descending).
- `inven_takeoff` SHALL truncate the count plus the wording table
 (wield/bow/lite/ammo/tool and the seven wear slots), give the `INVEN_CARRY`
 symbiote its landing announcement, then `force_drop` or `inven_carry`.
- `inven_drop` SHALL takeoff equipment first, split wand charges proportionally
 (`TV_WAND` only, pval*amt/number),
 then `drop_near` plus the three-helper count/describe/optimize updates.
- `combine_pack` SHALL merge pairwise from tail to head (the PW_INVEN window flag
 plus the announcement; no PN flags are set); `reorder_pack` SHALL shift forward by
 the same sort key (j>=i
 stays put; the overflow guard for a full INVEN_PACK slot is skipped).
- **Dead code:** `reorder_pack`'s overflow guard tests `i == INVEN_PACK` inside a
 loop bounded by `i < INVEN_PACK`, so it can never fire; `display_koff` shows
 the known base in the window.
- The misc helpers SHALL: `get_object` fetch by signed index; `pack_decay`/
 `floor_decay` run the corpse decay chain — announce then delete; a HEAD becomes a
 skull (weight wt/60 + rand_int(wt)/600); a CORPSE with `RF9_DROP_SKELETON` becomes
 a skeleton
 (weight wt/4 + rand_int(wt)/40); the converted object is reinserted (known ones become aware
 with name1=201 as the "The skull of ..." artifact-ization).

#### Scenario: Gold never merges into the pack

- **WHEN** `inven_carry_okay` evaluates gold
- **THEN** it always returns false

#### Scenario: Head decays to skull

- **WHEN** `pack_decay` processes a HEAD corpse
- **THEN** it becomes a skull with weight `wt/60 + rand_int(wt)/600`

- **Anchors**: `src/object2.c:5546-5776` (list helpers), `src/object2.c:5785-5975`
 (carry), `src/object2.c:5989-6168` (takeoff and drop), `src/object2.c:6177-6343`
 (combine/reorder), `src/object2.c:6350-6386` (koff), `src/object2.c:6649-6662`
 (get_object), `src/object2.c:6470-6646` (decay)
