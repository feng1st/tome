# player-derive Specification

## Purpose

Player-derived attributes: `src/xtra1.c` carries the stat conversions
(`cnv_stat`/`modify_stat_value`/`luck`), the body computation (`calc_body`/
`calc_body_bonus`/`calc_wield_monster` plus the two monk predicates), the god bonuses
(`calc_gods`), the total flag application (`apply_flags`), the full recalculation
(`calc_bonuses`), the four resource derivations (hit points/mana/sanity/light), the powers
computation (`calc_powers` and the empty `calc_spells`), the weapon-style predicates and
the fates system. Screen drawing, the sub-windows and the five-level update scheduling are
specified in specs/ui-frames/spec.md; the Lua-side `calc_mimic`/`get_mimic_info`/
`calc_mimic_power` belong to the `lib/scpt` scripts.

## Requirements

### Requirement: Stat Conversion

`cnv_stat` SHALL run in two modes - the regular `18/xxx` three-part format (bonuses >= 220
show `***`) and the `linear_stats` folded mode (bonuses >= 220 show `40`, everything else
shows `18+bonus/10`). `modify_stat_value` SHALL step point by point: below 18 one point per
step, at 18 and above ten points per step; stepping down, any value with 18<x<28 drops
straight to 18, and 3 is the floor. `luck(min,max)` SHALL clamp `luck_cur` to +-30 and map
it linearly into [min,max] as (luck+30) x range/60.

- **Anchors**: `src/xtra1.c:19-71` (cnv_stat), `:84-121` (modify_stat_value),
 `:4842-4855` (luck)

### Requirement: Body Computation

The body calculators SHALL derive equipment slots and body-granted flags:

- `calc_body` takes the character's own body (body_monster=0) as the sum of the rp and rmp
 `body_parts` (Bear mimicry clears ARMS and LEGS), and the host's `r_info` body parts when
 possessing;
- the HOOK_BODY_PARTS hook writes `extra_body_parts` first, then class body parts and the
 mimicry bonus stack on top (mimic_extra's CLASS_ARMS gives weapons and arms +1 each
 capped at 3, CLASS_LEGS gives legs +1 capped at 2);
- each part type is floored at zero and capped at `max_body_part`;
- the counts are then expanded into the `p_ptr->body_parts` slot map: the BODY_WEAPON count
 decides the number of consecutive WIELD slots; any BODY_WEAPON grants a BOW slot; each
 BODY_TORSO grants one BODY, OUTER, LITE, AMMO and CARRY slot each; each BODY_FINGER
 grants one ring slot; each BODY_HEAD grants HEAD and NECK; each BODY_ARMS grants ARM and
 HANDS plus one TOOL; each BODY_LEGS grants FEET;
- equipment whose slot disappears SHALL be taken off immediately with
 `inven_takeoff(255, TRUE)`;
- `calc_body_bonus` SHALL, when possessing, place host flags on the player: host ac adds to
 ac; pspeed is assigned the host speed directly (not accumulated); NEVER_MOVE sets
 immovable; STUPID/SMART subtract/add 1 INT; RF2_INVISIBLE adds invisibility +20;
 REFLECTING, REGENERATE, both auras, PASS_WALL
 (as wraith_form), SUSCEP_FIRE, the IM flags for acid/elec/fire/poison/cold (as the
 matching resistances),
 RES_NETH/RES_NEXU/RES_DISE, NO_FEAR (as resist_fear), NO_SLEEP (as free_act), NO_CONF,
 CAN_FLY (as feather fall) and AQUATIC (as water_breath) each land on the matching player
 field;
- a disembodied body SHALL only set wraith_form;
- `calc_wield_monster` SHALL grant the carried symbiote's flags: invisibility +20,
 reflection, feather fall and water breathing, per flag;
- `monk_empty_hands` SHALL be true when every WIELD slot is empty and the melee style is
 SKILL_HAND;
- `monk_heavy_armor` SHALL be true when the six armor slots together weigh more than
 100+SKILL_HANDx4.

- **Anchors**: `src/xtra1.c:2115-2136` (wield), `:2144-2265` (body), `:2268-2307`
 (body_bonus), `:4494-4529` (monk pair)

### Requirement: God Bonuses

`calc_gods` SHALL grant bonuses per current god and grace thresholds:

- Eru: +1 WIS at each of the 10000/20000/30000 grace tiers;
- Melkor: +1 in each of the STR/CON/CHR tiers and -1 in each of the INT/WIS tiers; while
 praying, 5000 grace grants invisibility +30 and 15000 grants immune_fire; resist_fire
 comes with the god immediately;
- Manwe while praying: +1 speed per 5000 grace (capped at 35000), free_act at 7000, fly at
 15000;
- Manwe while not praying: feather fall at 2000;
- Tulkas: +1 CON at each of the 5000/10000/15000 tiers and +1 STR at each of the
 10000/15000/20000 tiers.

#### Scenario: Praying to Manwe

- **WHEN** a Manwe worshipper is praying with grace between 7000 and 14999
- **THEN** speed rises by one per full 5000 grace (capped at 35000) and free_act is
 granted, but not fly

- **Anchors**: `src/xtra1.c:2418-2491`

### Requirement: Total Flag Application

`apply_flags` SHALL place every flag bit onto the player fields:

- the six stats and luck (TR5_LUCK), to_s (TR1_SPELL), to_m (TR1_MANA), to_l (TR2_LIFE),
 stl;
- srh and fos each pval x 5; infra; dig pval x 20; pspeed;
- extra_blows (accumulated in a file-level static), xtra_crit, sensible_fire, impact,
 invisibility pval x 10;
- TR3_XTRA_SHOTS increments extra_shots;
- aggravate and teleport set true, drain_mana and drain_life increment, xtra_might adds
 pval; exp_drain, bless_blade, slow_digest and regenerate set true; telepathy merges
 bit-wise;
- lite: when the object tval is not a light source, LITE1-3 grant permanent light;
- see_inv, free_act, hold_life, wraith_form, feather, fly, climb;
- the four-element immunities; the sixteen resistances plus immune_nether; reflect; both
 auras; anti_magic; anti_tele; the six sustains; precognition; auto_id; black_breath;
 immovable;
- water breathing and magic breathing (magic breathing implies water breathing).

TR4_ANTIMAGIC_50/30/20/10 SHALL add antimagic computed as "base + antimagic skill scale -
to_h - to_d - pval - to_a"; the display tiers compute separately (tier 50: scale 4 with a
further (sum)/15 reduction; tier 30: scale 2; tier 20: fixed 2; tier 10: fixed 1).

- **Anchors**: `src/xtra1.c:2493-2682`

### Requirement: Full Bonus Recalculation

`calc_bonuses(silent)` SHALL recalculate everything from scratch:

- zero every derived quantity: stat_add, to_m/to_l/to_s, ac/dis_ac, the four to_h/to_d
 directions, to_a, pspeed=110, xtra_crit, num_blow/num_fire=1, throw_mult=1,
 tval_ammo/tval_xtra, the roughly fifty boolean flags, the antimagic pair,
 see_infra=racial sum, the ten skill base values, xtra_f1-f5 and the SKF1_AUTO_HIDE hidden
 bit; luck_cur=luck_base;
- mimicry goes through the Lua `calc_mimic` for blows, otherwise `calc_body_bonus` runs;
- the HOOK_CALC_BONUS hook and `calc_wield_monster` run; class oflags are applied layer by
 layer with apply_flags for levels 1..player level;
- SKILL_HAND: light armor grants a speed bonus per level (scale 5); above level 24 light
 armor grants free_act;
- SKILL_ANTIMAGIC adds the antimagic pair (CLASS_ANTIMAGIC in antimagic_extra additionally
 sets anti_tele and resist_continuum);
- SKILL_DAEMON above level 20/30 grants resist_conf/resist_fear;
- SKILL_MINDCRAFT at level >= 40 grants ESP_ALL;
- the astral state sets wraith_form;
- race oflags are applied layer by layer only for the body itself, plus
 PR1_HURT_LITE -> sensible_lite and the xtra_f* fields;
- with maximize on, r_adj and c_adj stack in three layers.

The equipment scan SHALL:

- honor the object_flags_no_set bypass; ART_ANCHOR sets resist_continuum; silent mode
 strips BLACK_BREATH; apply_flags places the flags; artifacts hook into apply_set;
- take ac/dis_ac and to_a (only known items enter dis_to_a);
- add no to_h/to_d from the weapon/bow/ammo/tool four slot classes;
- SKILL_HAND fills in six-part AC for empty armor slots;
- sh_fire grants lite;
- PR1_AC_LEVEL adds +20+level/5;
- run calc_gods.

Stat finalization SHALL produce the top/use/ind three layers and trigger
PU_HP/PU_SANITY/PU_SPELLS/PU_MANA per CON/WIS/INT; Melkor sacrifice adds dis_to_d/to_d per
wisdom_scale(4) x count; the tactic table tactic_info and the movement table move_info
place to_hit/to_dam/to_ac/to_stealth/to_disarm/to_saving/to_speed/to_search/to_percep;
stun above 50 subtracts 20, any stun value below that subtracts 5; invuln adds +100 AC; the
tim_* and oppose_* states place item by item (tim_wraith adds +10 AC when disembodied,
otherwise +50 AC plus reflect; holy hold_life adds luck+5; blessed +5/+10; shield
+shield_power; hero +12; roots
clears stun and adds to_d_melee/to_a; shero +24/-10; strike +15; meditation -25; fast
+speed_factor; lightspeed +50; slow -10; tim_esp ESP_ALL; tim_invisible adds tim_inv_pow;
tim_invis grants see_inv; tim_infra +1).

The derived cross-implications SHALL hold: magical_breath -> water_breath;
fly -> feather fall; resist_chaos -> resist_conf; hero/shero -> resist_fear; resist_lite
and sensible_lite cancel each other out; any fire resistance/opposition/immunity clears
sensible_fire; sh_fire -> lite; a change in telepathy or see_inv triggers PU_MONSTERS.

Weight and speed SHALL: past the half-way point over the limit, speed drops 1 per
(limit/10); the bloated (food at maximum) and searching states each -10; CLASS_MANA_PATH
caps at 110; the
four stat modifier tables (adj_dex_ta/adj_str_td/adj_dex_th/adj_str_th) place
to_a/to_d/to_h; hold=adj_str_hold.

Heavy shooter and ammo SHALL: when hold < bow weight/10, to_h gains 2 x the difference and
heavy_shoot is recorded; tval_ammo maps the bow sval into three classes (shot/arrow/bolt);
the archery skill grants to_h_ranged (scale 25), num_fire (/16) and might (/25), plus
another +1/30 for the
same ammo; after extra_shots merges in, the result is floored at one; PR1_XTRA_MIGHT_*
adds +1 per ammo class; TV_DIGGING tools add dig per weight/10.

The blow count SHALL take three branches:

- with weapons: analyze_blow plus the blows_table[str][dex] double table clamped, plus
 extra_blows, plus lev x cp extra_blows/50, plus weaponmastery scale 2 and SKILL_MASTERY
 scale 3;
- bare-handed SKILL_HAND: 1-7 blows by the seven thresholds (>9/19/29/34/39/44/49), halved
 in heavy armor plus +1+extra, light armor additionally grants to_h/to_d lev/3;
- when the host has no BODY_WEAPON: the same table with mul/3 capped at 4 blows, then
 truncated to the host's blow slots;
- Bear mimicry bare-handed: 2+level/5 blows, to_h -level/5 and to_d +level/2.

The weapon miscellany SHALL:

- two-handed COULD2H with a shield subtracts half value from to_h_melee/to_d_melee (plus
 dd x ds/2);
- forbid_non_blessed edged weapons: -15 and records icky_wield;
- SKILL_SORCERY with a non-staff subtracts scale(100) (MSTAFF subtracts a tenth) and
 records icky;
- weaponmastery adds the skill value to to_h_melee and half the skill to to_d_melee;
- SKILL_COMBAT scale 10 adds to_d;
- the Dodge skill sets dodge_chance=scale(150)+hand skill-armor weight x 2-weight/100,
 floored at zero (cleared without the skill).

The skill summary SHALL: stealth +1 and item-by-item placement (dis: the two-stat table
plus SKILL_DISARMING scale 75; dev: SKILL_DEVICE scale 20 and 150; sav: adj_wis_sav plus
SPIRITUALITY scale 75; dig: adj_str_dig; stl: SKILL_STEALTH scale 25; srh/fos:
SKILL_SNEAK scale 35/25; thn=50 x (7 x melee style + 3 x combat)/100; thb=50 x (7 x
SKILL_ARCHERY + 3 x combat)/100; tht=50 x level/10); stl clamped to 0-30; dig floored at
1; with anti_magic, sav is raised to 95.

The wrap-up SHALL: announce changes in the four states heavy shooter/heavy weapon/icky
weapon/monk armor (skipped when silent); raise sav to 10 when at or below 10, otherwise add
another +10; run the HOOK_CALC_BONUS_END hook.

- **Anchors**: `src/xtra1.c:2705-4032`

### Requirement: The Four Resource Derivations

`calc_hitpoints` SHALL:

- compute mhp=player_hp[level-1] + adj_con_mhp modifier x level/2 (floored at level+1);
- add hp_mod;
- Sorcery subtracts a percentage by scale(50);
- Melkor sacrifice subtracts x10 per count;
- hero/shero add +10/+30;
- to_l adds a percentage (munchkin_multipliers: divide by 5, otherwise 10);
- possession averages three ways between the host formula
 maxroll(hdice,hside) x (20+SKILL_POSSESSION scale 80)/100, the previous value and sroot;
- disembodied sets mhp to 1;
- the CLASS_UNDEAD state divides by level/4 (guarded against zero);
- HOOK_CALC_HP may override the result;
- on change, chp is clamped and PR_HP is queued.

`calc_mana` SHALL:

- compute msp=SKILL_MAGIC scale 200 + adj_mag_mana[max(INT,WIS)] x level/4 + 1;
- possession changes the formula to 21-100/freq_spell, floored at 1;
- the rmp and cp percentages multiply on top;
- Eru adds a per-mille bonus of grace/100;
- when forbid_gloves holds, gloves that are neither FREE_ACT, nor positive DEX, nor magic
 containers subtract a quarter and record cumber_glove;
- to_m adds a percentage (munchkin: divide by 5, otherwise 10);
- when the six armor slots weigh more than 200+SKILL_COMBAT scale 500, subtract per point
 and record cumber_armor;
- meditation adds +50 twice (to both msp and csp);
- HOOK_CALC_MANA may override the result;
- on change, csp is clamped and PR_MANA queued; glove/armor state changes are announced
 (not during character_xtra).

`calc_sanity` SHALL compute msane=5 x (level+1) + adj_con_mhp[WIS] modifier x level/2
(floored at level+1); msane increments merge into csane and are capped.

`calc_torch` SHALL accumulate per equipment: FUEL_LITE items only count when timeout>0;
LITE1/2/3 each add +1/+2/+3; tim_lite +2; holy +1; capped at 5; intrinsic lite without a
light source floors at 1; HOOK_CALC_LITE runs; wild_mode caps at WILDERNESS_SEE_RADIUS;
running with view_reduce_lite caps at 1; changes trigger PU_VIEW plus PU_MONSTERS.

`weight_limit` SHALL be adj_str_wgt x 100, overridable by HOOK_CALC_WEIGHT.

#### Scenario: Gloved spellcaster

- **WHEN** `forbid_gloves` holds and the HANDS slot holds gloves that grant neither
 FREE_ACT, nor a positive DEX pval, nor a spell container flag
- **THEN** maximum mana drops to three quarters and `cumber_glove` is recorded

- **Anchors**: `src/xtra1.c:1674-1702` (sanity), `:1711-1896` (mana), `:1904-2007` (hp),
 `:2017-2094` (torch), `:2101-2113` (weight limit)

### Requirement: Powers And Spell Count

`calc_powers` SHALL honor the calc_powers_silent switch:

- save the old table, then reset the POWER_MAX_INIT segment from powers_mod (the remaining
 segments are zeroed);
- run HOOK_CALC_POWERS;
- hook each equipment's object_power slot (-1 skips);
- the body itself (not mimicking, not possessing) adds the four race and subrace power
 slots; the four class slots always apply;
- mimicry goes through the Lua calc_mimic_power;
- disembodied adds PWR_INCARNATE;
- compare against the old table bit by bit and announce the powers_type gain_text/
 lose_text (skipped when silent);
- clear the silent switch at the end.

`calc_spells` SHALL currently be an empty shell: it only sets new_spells=0 (skill-based
spells have no precomputation).

- **Anchors**: `src/xtra1.c:1578-1581` (spells shell), `:1584-1667` (powers)

### Requirement: Weapon Style Predicates

- `analyze_blow` SHALL return the class blow_num/wgt/mul plus one strike for each of
 AB_MAX_BLOW1/2.
- `get_weaponmastery_skill` SHALL return the matching skill only when every wielded weapon
 is of the same class (TV_DAEMON_BOOK merges into SKILL_SWORD; AXE/HAFTED/POLEARM each
 form their own class); empty slots are skipped; mixed classes return -1.
- `get_archery_skill` SHALL map the bow slot by sval/10 to SKILL_SLING/BOW/XBOW; anything
 not TV_BOW falls to SKILL_BOOMERANG; mixed returns -1.

- **Anchors**: `src/xtra1.c:2319-2415`

### Requirement: Fate System

- `get_artifact_idx` SHALL roll randint(max_a_idx-1) up to a thousand times, accepting the
 first artifact that has a tval, has not appeared in the world (cur_num zero), meets the
 depth requirement, and is not TR4_SPECIAL_GENE; if all rolls fail it returns 0 (treated
 as a randart).
- `gain_fate` SHALL take the first free slot of MAX_FATES and print "More of your prophecy
 has been unearthed!" plus "You should see a soothsayer quickly."
- without an argument it draws by rand_int(luck_cur>0 ? 17 : 18) - the lucky cannot draw
 slot 17 (FATE_DIE);
- bucket mapping: FIND_O seven slots / FIND_R seven slots / FIND_A two slots / DIE one
 slot / slot 0 rolls FATE_NO_DIE_MORTAL with dun_level/4 (capped 50), otherwise FIND_O;
- FIND_O SHALL draw an item across all four theme components plus the kind_is_legal hook
 and loop-discard INSTA_ART/NORM_ART, with the level taken as max_dlv+-20 clamped to 1-98
 and serious halving the roll;
- FIND_R is structured the same but draws a monster; FIND_A: serious is always true; DIE
 is announced only under wizard or precognition; NO_DIE_MORTAL sets p_ptr->no_mortal.
- `fate_desc` SHALL prefix "You are fated to "/"You may " by serious; FIND_A describes the
 pre-made artifact (a_idx=0 shows "something special").
- `dump_fates` SHALL write only entries whose know bit is set.

- **Anchors**: `src/xtra1.c:4531-4571` (roller), `:4574-4717` (gain), `:4719-4837`
 (description and dump)
