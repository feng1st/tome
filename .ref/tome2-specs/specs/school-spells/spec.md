# school-spells Specification

## Purpose

School spell definitions: the twenty-one `lib/scpt/s_*.lua` files register every
spell through `add_spell`. The unified skeleton (consumed by school-magic's
`finish_spell`, see specs/school-magic/spec.md): name/school (possibly several
schools)/level/mana/mana_max/fail are required; `stick` is optional (the charge
base-dice pair and the TV_WAND/TV_STAFF rarity/base_level/max_level ranges);
`inertia` is optional (the `{interval, span}` inertia-duration marker); `spell` is
the casting closure (`get_level` picks the effect), `info` the menu line, `desc`
the description table. This spec records the spells file by file; the school flags
and the book list, and the stick consumers (the `get_random_stick` family), are
covered in specs/school-magic/spec.md.

Notation used below: `level(n)` abbreviates `get_level(<spell>, n)`, and
`level(n, m)` its two-argument form; `charge a+1db` abbreviates
`charge = { a, b }`.

## Requirements

### Requirement: Air School

s_air.lua SHALL register six spells:

- NOXIOUSCLOUD `Noxious Cloud` (Air alone; level 3, mana 3/30, fail 20; wand
 charge 5+1d7, rarity 15; the cloud is `fire_cloud` for `7 + level(150)` damage,
 radius three, duration `5 + level(40)`, switching to GF_UNBREATH at
 level >= 30);
- AIRWINGS `Wings of Winds` (Air+Conveyance; level 22, mana 30/40, fail 60; staff
 charge 7+1d5, rarity 27; inertia {1,10}; below level 16 grants `tim_ffall`
 and otherwise `tim_fly`, duration `5 + level(25) + 1d10`);
- INVISIBILITY (level 16, mana 10/20, fail 50; inertia {1,30}; `set_invis`
 duration `15 + level(50) + 1d20`, strength `20 + level(50)`);
- POISONBLOOD (level 12, mana 10/20, fail 30; wand charge 10+1d15, rarity 45;
 inertia {1,35}; poison resistance `25 + level(25) + 1d30`, at level >= 15 adding
 the `tim_poison` poison brand);
- THUNDERSTORM (Air+Nature; level 25, mana 40/60, fail 60; wand charge 5+1d5,
 rarity 85; inertia {2,15}; `tim_thunder` duration `10 + level(25) + 1d10`,
 damage `5 + level(10)` d`(10 + level(25))`);
- STERILIZE (level 20, mana 10/100, fail 50; staff charge 7+1d5, rarity 20;
 `set_no_breeders` for `30 + 20 + level(70)`).

- **Anchors**: `lib/scpt/s_air.lua:3-39` (Noxious Cloud), `lib/scpt/s_air.lua:41-75`
 (Wings), `lib/scpt/s_air.lua:77-95` (Invisibility), `lib/scpt/s_air.lua:97-129`
 (Poison Blood), `lib/scpt/s_air.lua:131-163` (Thunderstorm),
 `lib/scpt/s_air.lua:165-193` (Sterilize)

### Requirement: Conveyance School

s_convey.lua SHALL register six spells:

- BLINK `Phase Door` (level 1, mana 1/3, fail 10; inertia {1,5};
 `teleport_player` `10 + level(8)`, at level >= 30 adding `create_between_gate`,
 a between gate that returns to the origin);
- DISARM (level 3, mana 2/4, fail 15; staff charge 10+1d15, rarity 4;
 `destroy_doors_touch`, at level >= 10 also `destroy_traps_touch`);
- TELEPORT (level 10, mana 8/14, fail 30; staff charge 7+1d7, rarity 50; inertia
 {1,10}; `teleport_player` `100 + level(100)` costing `25 - level(50)` energy);
- TELEAWAY (level 23, mana 15/40, fail 60; wand charge 3+1d5, rarity 75; three
 segments - below level 10 single-target `teleport_monster`, >= 10 a GF_AWAY_ALL
 ball of radius `3 + level(4)`, >= 20 `project_los` over everything visible);
- RECALL (level 30, mana 25/25, fail 60; three stances - on self `recall_player`
 (`d = 21 - level(15)` clamped to zero, `f = 15 - level(10)` clamped to one), on
 a monster `swap_position`, on an object `set_target` plus `fetch` (below level 15
 requires LOS); no inertia);
- PROBABILITY_TRAVEL (level 35, mana 30/50, fail 90; staff charge 1+1d2, rarity
 97; inertia {6,40}; `set_prob_travel` for `level(60) + 1d20`).

- **Anchors**: `lib/scpt/s_convey.lua:3-31` (Blink), `lib/scpt/s_convey.lua:33-65`
 (Disarm), `lib/scpt/s_convey.lua:67-97` (Teleport),
 `lib/scpt/s_convey.lua:99-140` (Teleport Away), `lib/scpt/s_convey.lua:142-195`
 (Recall), `lib/scpt/s_convey.lua:197-227` (Probability Travel)

### Requirement: Demon School

s_demon.lua SHALL register nine spells (all `random = 0`, never in random books,
belonging to the three demon-blade books):

- Demon Blade (level 1, mana 4/44, fail 10; wand charge 3+1d7, rarity 75;
 `set_project` weapon brand, duration `level(80) + 1d20`, `4 + level(40)` damage
 per strike, switching to GF_HELL_FIRE at level >= 30 and radius one at >= 45);
- DEMON_MADNESS (level 10, mana 5/20, fail 25; dual balls on the target and its
 mirror point, the type drawn at random from CHAOS/CONFUSION/CHARM, damage
 `20 + level(200)`, radius `1 + level(4, 0)`);
- DEMON_FIELD (level 20, mana 20/60, fail 60; `fire_cloud` GF_NEXUS, damage
 `20 + level(70)`, radius seven, duration `30 + level(100)`);
- DOOM_SHIELD (level 1, mana 2/30, fail 10; `set_shield` counter-shield, duration
 `20 + level(100) + 1d10`, AC `-300 + level(100)`, counter
 `1 + level(14)` d`(10 + level(15))`);
- UNHOLY_WORD (level 25, mana 15/45, fail 55; `tgt_pt` restricted to pets, a
 `30 - level(25, 0)`% chance the pet turns hostile, otherwise a heal of
 `(30 + level(50, 0))%` of its hit points and `delete_monster_idx`);
- DEMON_CLOAK (level 20, mana 10/40, fail 70; `set_tim_reflect`
 `5 + level(15, 0) + 1d5`);
- DEMON_SUMMON (level 5, mana 10/50, fail 30; `summon_specific_level` at
 `5 + level(100)` with the depth clamped to start at 4, upgrading to
 SUMMON_HI_DEMON at level >= 35, failure playing the blocked message);
- DISCHARGE_MINION (level 10, mana 20/50, fail 30; pets only, after `delete`
 deals `hp * (20 + level(60, 0))%` capped at `100 + level(500, 0)` as a
 GF_GRAVITY ball);
- CONTROL_DEMON (level 25, mana 30/70, fail 55; `fire_ball` GF_CONTROL_DEMON,
 strength `50 + level(250)`).

The end of the file carries the `HOOK_WIELD_SLOT` slot assignment for the three
demon-blade books: SV_DEMONBLADE -> INVEN_WIELD, SV_DEMONSHIELD -> INVEN_ARM,
SV_DEMONHORN -> INVEN_HEAD (`ideal` takes the ideal slot, otherwise `get_slot`
returns the current position).

- **Anchors**: `lib/scpt/s_demon.lua:4-113` (blade trio), `lib/scpt/s_demon.lua:117-207`
 (shield trio), `lib/scpt/s_demon.lua:211-307` (horn trio),
 `lib/scpt/s_demon.lua:310-337` (slot hook)

### Requirement: Divination School

s_divin.lua SHALL register six spells:

- STARIDENTIFY `Greater Identify` (level 35, mana 30/30, fail 80; questioning
 oneself runs `self_knowledge`, otherwise `identify_fully`);
- IDENTIFY (level 8, mana 10/50, fail 40; staff charge 7+1d10, rarity 45; three
 segments - below 17 single-item `ident_spell`, >= 17 the whole pack plus a
 GF_IDENTIFY zero-radius ball, >= 27 adding a floor ball of radius `level(3)`;
 success fires PN_COMBINE|PN_REORDER);
- VISION (level 15, mana 7/55, fail 45; staff charge 4+1d6, rarity 60; inertia
 {2,200}; below 25 `map_area`, otherwise `wiz_lite_extra`);
- SENSEHIDDEN (level 5, mana 2/10, fail 25; staff charge 1+1d15, rarity 20;
 inertia {1,10}; `detect_traps` `15 + level(40, 0)`, at level >= 15 adding
 `tim_invis` `10 + level(40) + 1d20`);
- REVEALWAYS (level 9, mana 3/15, fail 20; staff charge 6+1d6, rarity 35; inertia
 {1,10}; `detect_doors` plus `detect_stairs`, each `10 + level(40, 0)`);
- SENSEMONSTERS (level 1, mana 1/20, fail 10; staff charge 5+1d10, rarity 37;
 inertia {1,10}; `detect_monsters_normal` `10 + level(40, 0)`, at level >= 30
 adding `tim_esp` `10 + level(20) + 1d10`).

- **Anchors**: `lib/scpt/s_divin.lua:4-27` (Star Identify),
 `lib/scpt/s_divin.lua:29-81` (Identify), `lib/scpt/s_divin.lua:83-117` (Vision),
 `lib/scpt/s_divin.lua:119-157` (Sense Hidden), `lib/scpt/s_divin.lua:159-190`
 (Reveal Ways), `lib/scpt/s_divin.lua:192-230` (Sense Monsters)

### Requirement: Earth School

s_earth.lua SHALL register five spells:

- STONESKIN `Stone Skin` (level 1, mana 1/50, fail 10; inertia {2,50};
 `set_shield` duration `10 + level(100) + 1d10`, AC `10 + level(50)`, counter
 parameters `2 + level(5)` d`(3 + level(5))`, at level >= 25 adding the
 SHIELD_COUNTER type);
- DIG (level 12, mana 14/14, fail 20; wand charge 15+1d5, rarity 25 (base/max both
 {1,1}); `wall_to_mud`);
- STONEPRISON (level 25, mana 30/50, fail 65; wand charge 5+1d3, rarity 57; below
 level 10 self-centered, otherwise `tgt_pt`, `wall_stone` imprisons the target);
- STRIKE (level 30, mana 30/50, fail 60; wand charge 2+1d6, rarity 635; a GF_FORCE
 minor ball of `50 + level(50)` damage, radius one at level >= 12);
- SHAKE (level 27, mana 25/30, fail 60; staff charge 5+1d10, rarity 75; inertia
 {2,50}; at level >= 10 `tgt_pt` is available, `earthquake` of radius
 `4 + level(10)`).

- **Anchors**: `lib/scpt/s_earth.lua:3-32` (Stone Skin), `lib/scpt/s_earth.lua:34-64`
 (Dig), `lib/scpt/s_earth.lua:66-102` (Stone Prison),
 `lib/scpt/s_earth.lua:104-144` (Strike), `lib/scpt/s_earth.lua:146-184` (Shake)

### Requirement: Eru School

s_eru.lua SHALL register four spells (all `piety = TRUE` - cast through piety,
stat A_WIS, `random = SKILL_SPIRITUALITY`):

- ERU_SEE `See the Music` (level 1, mana 1/50, fail 20; castable while blind
 (`blind = FALSE`); `set_tim_invis` `10 + level(100) + 1d20`, at levels
 10/20/30 progressing through `map_area` / curing blindness / `wiz_lite_extra`);
- ERU_LISTEN (level 7, mana 15/200, fail 25; three segments - single-item
 `ident_spell`, >= 14 the whole pack, >= 30 a full-map `ident_all` plus the whole
 pack);
- ERU_UNDERSTAND (level 30, mana 200/600, fail 50; below 10 single-item
 `identify_fully`, otherwise the whole pack `*identify*`);
- ERU_PROT `Lay of Protection` (level 35, mana 400/400, fail 80; `fire_ball`
 GF_MAKE_GLYPH, radius `1 + level(2, 0)`, the glyph circle).

- **Anchors**: `lib/scpt/s_eru.lua:3-42` (See the Music),
 `lib/scpt/s_eru.lua:44-77` (Listen), `lib/scpt/s_eru.lua:79-107` (Understand),
 `lib/scpt/s_eru.lua:109-130` (Lay of Protection)

### Requirement: Fire School

s_fire.lua SHALL register five spells:

- GLOBELIGHT (level 1, mana 2/15, fail 10; staff charge 10+1d5, rarity 7; inertia
 {1,40}; below level 3 `lite_room` otherwise `lite_area`, at level >= 15 adding a
 GF_LITE ball of `10 + level(100)` damage, radius `5 + level(6)`, and firing
 PU_VIEW);
- FIREFLASH (level 10, mana 5/70, fail 35; wand charge 5+1d5, rarity 35; a fire
 ball of `20 + level(500)` damage, radius `2 + level(5)`, switching to
 GF_HOLY_FIRE at level >= 20);
- FIERYAURA `Fiery Shield` (level 20, mana 20/60, fail 50; staff charge 3+1d5,
 rarity 50; inertia {2,15}; `set_shield` fire shield, duration
 `10 + level(70) + 1d20`, AC 10, counter `5 + level(10, 15)` (the info line shows
 15/7), upgrading to SHIELD_GREAT_FIRE at level >= 8);
- FIREWALL (level 15, mana 25/100, fail 40; wand charge 4+1d5, rarity 55;
 `fire_wall` of `40 + level(150)` damage, duration `10 + level(14)`, switching to
 GF_HELL_FIRE at level >= 6);
- FIREGOLEM (Fire+Mind; level 7, mana 16/70, fail 40; `do_control_reconnect` runs
 first; a torch or lantern (TV_LITE sval 0/1) is taken and destroyed as the
 material, `m_allow_special[1044]` temporarily permits `place_monster_one` of
 monster 1043 as MSTATUS_FRIEND, `monster_set_level` to `7 + level(70)`, and
 `player.control` plus MFLAG_CONTROL are set; the desc carries the control key
 table).

- **Anchors**: `lib/scpt/s_fire.lua:3-48` (Globe of Light),
 `lib/scpt/s_fire.lua:50-86` (Fireflash), `lib/scpt/s_fire.lua:88-123` (Fiery
 Shield), `lib/scpt/s_fire.lua:125-162` (Firewall),
 `lib/scpt/s_fire.lua:164-227` (Fire Golem)

### Requirement: Mana School

s_mana.lua SHALL register four spells and one helper: `get_manathrust_dam` returns
`3 + level(50)` and the `1 + level(20)` dice group.

- MANATHRUST (level 1, mana 1/25, fail 10; wand charge 7+1d10, rarity 5; a
 GF_MANA bolt with that damroll dice group, irresistible);
- DELCURSES (level 10, mana 20/40, fail 30; staff charge 3+1d8, rarity 70; inertia
 {1,10}; below 20 `remove_curse` otherwise `remove_all_curse`, success playing
 "The curse is broken!");
- RESISTS `Elemental Shield` (level 20, mana 17/20, fail 40; inertia {2,25}; the
 four-element oppose entries each `15 + level(50) + 1d10`, skipping those already
 present);
- MANASHIELD `Disruption Shield` (level 45, mana 50/50, fail 90; inertia {9,10};
 below level 5 grants `disrupt_shield`, otherwise (level >= 5) grants `invuln`,
 duration `3 + level(10) + 1d5`, not refreshed when the state is already present).

- **Anchors**: `lib/scpt/s_mana.lua:3-5` (dice group),
 `lib/scpt/s_mana.lua:7-42` (Manathrust), `lib/scpt/s_mana.lua:44-78` (Remove
 Curses), `lib/scpt/s_mana.lua:80-103` (Elemental Shield),
 `lib/scpt/s_mana.lua:105-132` (Disruption Shield)

### Requirement: Manwe School

s_manwe.lua SHALL register four spells (all `piety = TRUE`, A_WIS,
`random = SKILL_SPIRITUALITY`):

- MANWE_SHIELD `Wind Shield` (level 10, mana 100/500, fail 30; `set_protevil`
 duration `level(50) + 10 + 1d20`, at level >= 10 adding the wind shield AC
 `level(30)` (upgrading to SHIELD_COUNTER at level >= 20 with counter
 `1 + level(2)` d`(1 + level(6))`));
- MANWE_AVATAR (level 35, mana 1000/1000, fail 80; `set_mimic` into the Maia form,
 duration `level(20) + 1d10`, mimic level = `player.lev`);
- MANWE_BLESS (level 1, mana 10/100, fail 20; the blessing, duration
 `level(70) + 30 + 1d40` plus fear removal plus a `set_lite(0)` call, at levels
 10/20/30 adding hero/shero/holy);
- **Discrepancy:** the spell description promises it "surrounds you with holy
 light", but the cast calls `set_lite(0)`, which clears any active temporary
 light instead of granting one (`tim_lite` is the +2 light timer, see
 `src/xtra2.c:1306-1349`, `src/xtra1.c:2047`).
- MANWE_CALL (level 20, mana 200/500, fail 40; `find_position` places a Great
 eagle ally and `monster_set_level` to `20 + level(70, 0)`).

- **Anchors**: `lib/scpt/s_manwe.lua:3-47` (Wind Shield),
 `lib/scpt/s_manwe.lua:49-70` (Avatar), `lib/scpt/s_manwe.lua:72-112`
 (Blessing), `lib/scpt/s_manwe.lua:114-144` (Call)

### Requirement: Melkor School

s_melkor.lua SHALL register three spells (all `piety = TRUE`, A_WIS,
`random = SKILL_SPIRITUALITY`; Curse and Mind Steal require a monster target)
and one shared piece:

- `do_melkor_curse(who)` SHALL apply tiered weakening - at level >= 35 it cuts one
 die side from maxhp with a floor of one and clamps the hp, at level >= 25 it
 lowers `speed` and `mspeed` each by `level(7)` clamped to 70, at level >= 15 it
 lowers `ac` by `level(50)` clamped to -70; each of the four strike positions
 with `d_dice > 0` loses `pow` from `d_dice`, where `pow` starts at `level(2)`
 and shrinks to `min(pow, d_dice, d_side)` as each strike is processed (so later
 strikes can lose fewer dice); it plays "looks weaker." and wakes the target.
- MELKOR_CURSE (level 1, mana 50/300, fail 20; casts `do_melkor_curse`; the desc
 carries the free in-combat auto-cast from level 5);
- MELKOR_CORPSE_EXPLOSION (level 10, mana 100/500, fail 45; `fire_ball`
 GF_CORPSE_EXPL, damage `20 + level(70)`%, radius `2 + level(5)`);
- MELKOR_MIND_STEAL (level 20, mana 1000/3000, fail 90; control only when
 `randint(monster level) < level` and the target is not RF1_UNIQUE - sets
 `player.control` plus MFLAG_CONTROL and plays the controlled message, otherwise
 plays the resistance message).

- **Anchors**: `lib/scpt/s_melkor.lua:4-49` (curse shared piece),
 `lib/scpt/s_melkor.lua:51-84` (Curse), `lib/scpt/s_melkor.lua:86-108` (Corpse
 Explosion), `lib/scpt/s_melkor.lua:110-154` (Mind Steal)

### Requirement: Meta School

s_meta.lua SHALL register five spells and the inertia-control system:

- RECHARGE (level 5, mana 10/100, fail 20; `recharge(60 + level(140))`);
- SPELLBINDER (level 20, mana 100/300, fail 85; `get_spellbinder_max` caps at 4;
 when already active the current state plays; the trigger thresholds a/b/c map to
 HP 75/50/25; each slot picks spells with `get_school_spell("bind")` (a base level
 above `7 + level(35)` is refused); full success costs 3100 energy and plays ready;
 abandoning midway clears trigger/num);
- DISPERSEMAGIC (level 15, mana 30/60, fail 40; double immunity to blind and
 confusion (`blind`/`confusion = FALSE`); wand charge 5+1d5, rarity 25; inertia
 {1,5}; the level tiers 1/5/10/15/20 clear the state groups in turn - blindness
 and light, confusion and images, hasting/slowness/light speed, stun/meditation/
 cutting, courage/berserk/blessing/shield/fear/parasite/mimicry);
- TRACKER (Meta+Conveyance; level 30, mana 50/50, fail 95; a `last_teleportation`
 of -1 is refused, otherwise `teleport_player_to` the last teleport point).

The inertia-control system SHALL work as follows: the TIMER_INERTIA_CONTROL timer
(delay 10, `save_timer` persisted, `player.inertia_controlled_spell` persisted and
defaulting to -1; the callback passes the double anti-magic gate or refuses, and
outside the wilderness casts `__spell_spell[number]()`); `stop_inertia_controlled_spell`
clears the number, stops the timer and fires PU_MANA; HOOK_CALC_MANA deducts four
times the spell cost from `msp`; HOOK_BIRTH_OBJECTS resets the state; INERTIA_CONTROL
(level 37, mana 300/700, fail 95; when already controlled it cancels;
`get_school_spell("control")` picks the spell, refused when there is no `inertia`
field or `inertia[1] > level(10)`; on success the spell number is registered, the
timer is enabled with `delay = inertia[2]`, and an announcement plays).

- **Anchors**: `lib/scpt/s_meta.lua:3-20` (Recharge),
 `lib/scpt/s_meta.lua:22-103` (Spellbinder), `lib/scpt/s_meta.lua:105-168`
 (Disperse), `lib/scpt/s_meta.lua:170-193` (Tracker),
 `lib/scpt/s_meta.lua:195-239` (inertia-control system),
 `lib/scpt/s_meta.lua:241-287` (Inertia Control)

### Requirement: Mind School

s_mind.lua SHALL register four spells:

- CHARM (level 1, mana 1/20, fail 10; wand charge 7+1d5, rarity 35; three
 segments - a GF_CHARM bolt of `10 + level(150)` damage, at >= 15 a ball of
 radius three, at >= 35 `project_los` over everything visible);
- CONFUSE (level 5, mana 5/30, fail 20; wand charge 3+1d4, rarity 45; the
 same-structure three segments with GF_OLD_CONF);
- ARMOROFFEAR (level 10, mana 10/50, fail 35; inertia {2,20}; `set_shield`
 SHIELD_FEAR, duration `10 + level(100) + 1d10`, counter-fear
 `1 + level(7)` d`(5 + level(20))`);
- STUN (level 15, mana 10/90, fail 45; a GF_STUN bolt of `10 + level(150)` damage,
 at >= 20 a ball of radius three).

- **Anchors**: `lib/scpt/s_mind.lua:3-42` (Charm), `lib/scpt/s_mind.lua:44-83`
 (Confuse), `lib/scpt/s_mind.lua:85-104` (Armor of Fear),
 `lib/scpt/s_mind.lua:106-132` (Stun)

### Requirement: Music School

s_music.lua SHALL register thirteen tunes (all stat A_CHR, `random = SKILL_MUSIC`,
`pval` being the in-book level; lasting tunes drain mana per turn through their
`lasting` closures - source comment: every lasting tune must return a mana cost;
`start_lasting_spell` attaches `music_extra` with a negative sign).

- The stop piece MUSIC_STOP (fail -400, always castable; stops the song).
- Drum group, three lasting tunes: MUSIC_HOLD (level 1; `project_los` GF_OLD_SLOW
 `10 + level(100)`), MUSIC_CONF (level 5; GF_OLD_CONF), MUSIC_STUN (level 10;
 GF_STUN).
- Harp group, five lasting tunes: MUSIC_LITE (level 1; `set_lite(5)`, always
 draining one mana), MUSIC_HEAL (level 7; `hp_player` `7 + level(100)`),
 MUSIC_HERO (level 10; `hero` 5, at levels 10/20/25 adding shero/strike/oppose_cc),
 MUSIC_TIME (level 20; shield AC `10 + level(50)`, at >= 15 adding
 `fast(5, 7 + level(10))`), MUSIC_MIND (level 25; `tim_esp(5)`, at >= 10 adding a
 GF_IDENTIFY ball of radius `1 + level(3, 0)`).
- Horn group, four instant tunes: MUSIC_BLOW (level 4; a GF_SOUND ball centered on
 the player, damage `(2 + level(10, 0))` d`(4 + level(40, 0))`, radius
 `1 + level(12, 0)`), MUSIC_WIND (level 14; a GF_AWAY_ALL ball centered on the
 player, **its damage and radius currently reference MUSIC_BLOW's level**),
 MUSIC_YLMIR (level 20; `earthquake` of radius `2 + level(SHAKE, 10)` - currently
 referencing another spell's level), MUSIC_AMBARKANTA (level 25;
 `alter_reality`).

The end-of-file comment contains the two-form blank template.

- **Anchors**: `lib/scpt/s_music.lua:4-26` (Stop), `lib/scpt/s_music.lua:29-114`
 (the three drum tunes), `lib/scpt/s_music.lua:117-285` (the five harp tunes),
 `lib/scpt/s_music.lua:289-388` (the four horn tunes),
 `lib/scpt/s_music.lua:391-443` (template comment)

### Requirement: Nature School

s_nature.lua SHALL register five spells:

- GROWTREE (Nature+Temporal; level 6, mana 6/30, fail 35; inertia {5,50};
 `grow_trees` of radius `2 + level(7)`);
- HEALING (level 10, mana 15/50, fail 45; staff charge 2+1d3, rarity 90;
 `hp_player` by the max-hp percentage `15 + level(35)`);
- RECOVERY (level 15, mana 10/25, fail 60; staff charge 5+1d10, rarity 50; inertia
 {2,100}; with poison it starts halved, at levels 5/10/15 progressing through
 poison-and-cut double clear / six-stat restore / level restore);
- REGENERATION (level 20, mana 30/55, fail 70; inertia {4,40}; `tim_regen`
 duration `5 + level(50) + 1d10`, strength `300 + level(700)`);
- SUMMONANNIMAL (level 25, mana 25/50, fail 90; wand charge 1+1d3, rarity 85;
 `summon_specific_level` at `25 + level(50)` then `summon_monster` friendly
 SUMMON_ANIMAL).

- **Anchors**: `lib/scpt/s_nature.lua:3-22` (Grow Trees),
 `lib/scpt/s_nature.lua:24-51` (Healing), `lib/scpt/s_nature.lua:53-101`
 (Recovery), `lib/scpt/s_nature.lua:103-121` (Regeneration),
 `lib/scpt/s_nature.lua:124-152` (Summon Animal)

### Requirement: Device School (Sticks And Artifact Activation)

s_stick.lua SHALL register twelve spells (all `random = -1`, never in random
books). The wand/staff eight:

- DEVICE_HEAL_MONSTER (level 3; wand rarity 17; a GF_OLD_HEAL ball of
 `20 + level(380)`);
- DEVICE_SPEED_MONSTER (level 10; wand rarity 7; a GF_OLD_SPEED ball);
- DEVICE_WISH `Wish` (level 50, mana 400, fail 99; staff charge 1+1d2, rarity 98
 (base/max both 1); `make_wish`);
- DEVICE_SUMMON (level 5; staff charge 1+1d20, rarity 13; `4 + level(30)` hostile
 summons);
- DEVICE_MANA (level 30; staff charge 2+1d3, rarity 78; restores
 `msp * (20 + level(50))%` mana);
- DEVICE_NOTHING (both wand and staff, rarity 3, does nothing);
- DEVICE_HOLY_FIRE (level 30; staff rarity 999 to keep it out of random generation
 (source comment); `project_los` GF_HOLY_FIRE `50 + level(300)`);
- DEVICE_THUNDERLORDS (staff rarity 999; inside a dungeon `recall_player(0, 1)`
 for an immediate trip to the surface).

The artifact-activation four (an `activate` field given as a number or as
`{base, dice}` over-time limit):

- DEVICE_LEBOHAUM (activate 3, sings a tune);
- DEVICE_MAGGOT (activate {10,50}; a GF_TURN_ALL ball of 40, radius two);
- DEVICE_ETERNAL_FLAME (activate {0,0}; takes one of the
 four base kinds (long sword / mage staff / heavy crossbow / Power dragon armor),
 sets `name1` to one of the four ultimate artifacts 147/127/152/17 and runs
 `apply_magic(-1, T, T, T)`, sets `found = OBJ_FOUND_SELFMADE`, and destroys the
 eternal-flame kind);
- DEVICE_DURANDIL (activate 3, sings a tune).

- **Anchors**: `lib/scpt/s_stick.lua:3-133` (the first six sticks),
 `lib/scpt/s_stick.lua:135-200` (Mana and Nothing),
 `lib/scpt/s_stick.lua:202-279` (the three activations and Holy Fire),
 `lib/scpt/s_stick.lua:283-342` (Eternal Flame),
 `lib/scpt/s_stick.lua:345-412` (Durandil and Thunderlords),
 `lib/scpt/s_stick.lua:414-444` (template comment)

### Requirement: Temporal School

s_tempo.lua SHALL register four spells:

- MAGELOCK (level 1, mana 1/35, fail 10; wand charge 7+1d5, rarity 30; three
 segments - at low level `wizard_lock`, at >= 30 `cave_set_feat(3)` placing a
 glyph on the own grid, at >= 40 `tgt_pt` remote placement (three refusals: not a
 floor / permanent / no LOS));
- SLOWMONSTER (level 10, mana 10/15, fail 35; wand charge 5+1d5, rarity 23;
 GF_OLD_SLOW, `40 + level(160)` damage, at >= 20 a ball of radius one);
- ESSENCESPEED (level 15, mana 20/40, fail 50; wand charge 3+1d3, rarity 80;
 inertia {5,20}; `fast` duration `10 + level(50) + 1d10`, strength
 `5 + level(20)`, not refreshed when already present);
- BANISHMENT (Temporal+Conveyance; level 30, mana 30/40, fail 95; wand charge
 1+1d3, rarity 98; inertia {5,50}; `project_los` GF_AWAY_ALL `40 + level(160)`,
 at >= 15 adding a GF_STASIS `20 + level(120)` time bubble).

- **Anchors**: `lib/scpt/s_tempo.lua:4-53` (Magelock),
 `lib/scpt/s_tempo.lua:55-95` (Slow Monster), `lib/scpt/s_tempo.lua:97-126`
 (Essence of Speed), `lib/scpt/s_tempo.lua:128-162` (Banishment)

### Requirement: Tulkas School

s_tulkas.lua SHALL register three spells (all `piety = TRUE`, A_WIS,
`random = SKILL_SPIRITUALITY`):

- TULKAS_AIM `Divine Aim` (level 1, mana 30/500, fail 20; `set_strike` duration
 `level(50) + 1d10`, at >= 20 adding `tim_deadly`);
- TULKAS_WAVE `Wave of Power` (level 20, mana 200/200, fail 75; `fire_bolt`
 GF_ATTACK with damage of `level(player.num_blow)`, i.e. capped by the
 projection count recorded in blows);
- TULKAS_SPIN `Whirlwind` (level 10, mana 100/100, fail 45; `fire_ball` GF_ATTACK
 on the own grid, damage one, radius one).

- **Anchors**: `lib/scpt/s_tulkas.lua:3-32` (Divine Aim),
 `lib/scpt/s_tulkas.lua:34-58` (Wave of Power), `lib/scpt/s_tulkas.lua:60-81`
 (Whirlwind)

### Requirement: Udun School

s_udun.lua SHALL register four spells and two book-inspection pieces:

- DRAIN (Udun+Mana; level 1, zero mana, fail 20; destroys a TV_WAND/TV_STAFF for
 `kind.level * pval * number` mana, or drains a TV_ROD_MAIN of its `timeout`,
 clearing it to zero and firing the three PN/WIN flag groups; `increase_mana`
 credits the amount);
- GENOCIDE (Udun+Nature; level 25, mana 50/50, fail 90; staff charge 2+1d2,
 rarity 85; below 10 a single-race `genocide(TRUE)`, at >= 10 a `get_check` may
 allow `mass_genocide(TRUE)`);
- WRAITHFORM (Udun+Conveyance; level 30, mana 20/40, fail 95; inertia {4,30};
 `set_shadow` duration `20 + level(40) + 1d30`);
- FLAMEOFUDUN (Udun+Fire; level 35, mana 70/100, fail 95; inertia {7,15};
 `set_mimic` into the Balrog form, duration `5 + level(30) + 1d15`, mimic level =
 `level(FLAMEOFUDUN)`).

The inspection pieces SHALL be: `udun_in_book(sval, pval)` (a 255 random book
temporarily attaches `{pval}` and then counts the spells of the Udun and Melkor
dual schools) and `levels_in_book` (accumulates each spell's `level`; 255 handled
the same way) - provided for Udun-specific consumption logic (such as spellbook
devouring) to consult.

- **Anchors**: `lib/scpt/s_udun.lua:3-57` (Drain), `lib/scpt/s_udun.lua:59-101`
 (Genocide), `lib/scpt/s_udun.lua:103-121` (Wraithform),
 `lib/scpt/s_udun.lua:123-141` (Flame of Udun), `lib/scpt/s_udun.lua:145-180`
 (the two inspection pieces)

### Requirement: Water School

s_water.lua SHALL register five spells and one dice component:
`get_geyser_damage` returns `level(GEYSER, 10)` and `3 + level(35)`.

- GEYSER (level 1, mana 1/35, fail 5; `fire_bolt_or_beam` with probability
 `2 * level(85)`, GF_WATER, damage from that damroll dice group);
- VAPOR (level 2, mana 2/12, fail 20; inertia {1,30}; `fire_cloud` GF_WATER
 centered on the player, `3 + level(20)` damage, radius `3 + level(9, 0)`,
 duration 5);
- ENTPOTION (level 6, mana 7/15, fail 35; inertia {1,30}; `set_food` to full
 satiation, at >= 5 removing fear, at >= 12 adding `hero` `25 + level(40) + 1d25`);
- TIDALWAVE (level 16, mana 16/40, fail 65; wand charge 6+1d5, rarity 54; inertia
 {4,100}; `fire_wave` GF_WAVE EFF_WAVE, `40 + level(200)` damage, expanding radius
 `6 + level(10)`);
- ICESTORM (level 22, mana 30/60, fail 80; wand charge 3+1d7, rarity 65; inertia
 {3,40}; `fire_wave` EFF_STORM, `80 + level(200)` damage, radius
 `1 + level(3, 0)`, duration `20 + level(70)`, switching to GF_ICE at >= 10).

- **Anchors**: `lib/scpt/s_water.lua:3-33` (Tidal Wave),
 `lib/scpt/s_water.lua:35-69` (Ice Storm), `lib/scpt/s_water.lua:71-103` (Ent's
 Potion), `lib/scpt/s_water.lua:105-124` (Vapor), `lib/scpt/s_water.lua:126-154`
 (Geyser)

### Requirement: Yavanna School

s_yavann.lua SHALL register five spells (all `piety = TRUE`, A_WIS,
`random = SKILL_SPIRITUALITY`):

- YAVANNA_CHARM_ANIMAL (level 1, mana 10/100, fail 30; a GF_CONTROL_ANIMAL ball,
 strength `10 + level(170)`, radius `level(2)`);
- YAVANNA_GROW_GRASS (level 10, mana 70/150, fail 65; `grow_grass` of radius
 `level(4)`; the grass-prayer regeneration bonus while praying on grass lives in
 `src/dungeon.c:1991-1998`);
- YAVANNA_TREE_ROOTS (level 15, mana 50/1000, fail 70; `set_roots` duration
 `10 + level(30)`, AC `10 + level(60)`, damage `10 + level(20)`);
- YAVANNA_WATER_BITE (level 20, mana 150/300, fail 90; `set_project` water brand,
 duration `30 + level(150) + 1d30`, `10 + level(50)` damage per strike, radius
 one at level >= 25);
- YAVANNA_UPROOT (level 35, mana 250/350, fail 95; turns the adjacent tree grid to
 grass and `find_position` places an Ent ally with `monster_set_level` to
 `30 + level(70)`; with no tree present the refusal message plays).

- **Anchors**: `lib/scpt/s_yavann.lua:3-27` (Charm Animal),
 `lib/scpt/s_yavann.lua:29-52` (Grow Grass), `lib/scpt/s_yavann.lua:54-77` (Tree
 Roots), `lib/scpt/s_yavann.lua:79-110` (Water Bite),
 `lib/scpt/s_yavann.lua:112-157` (Uproot)

### Requirement: Geomancy School

s_geom.lua SHALL register eight spells, two custom GFs and four helper pieces.

GF registration SHALL be: GF_ELEMENTAL_WALL (its grid handler lays a random wall
per `geomancy_random_wall`) and GF_ELEMENTAL_GROWTH (its grid handler lays a random
floor), with the angry handler never enraging.

The helper pieces SHALL be: `geomancy_random_wall`/`geomancy_random_floor` roll
terrain by skill ratio (walls in four tiers: Fire -> sand wall, Water -> tree/ice
wall, Earth -> granite, each requiring skill >= its threshold and
`magik(skill scale 100)`, looping until a tier hits; floors in nine tiers, three
per element, with `random_floor` additionally carrying the `kill_wall` bit and
permanent-grid protection); `geomancy_can_tunnel`, the sixteen-entry diggable
terrain table; `geomancy_dig` digs `length` grids along the direction (stopping at
an undiggable grid), lays random walls on diggable neighbor grids along the way,
and on the return trip lays floors with `random_floor(kill_wall)` and with twenty
percent probability branches via `rotate_dir` to dig 1/3 of the length.

The eight spells SHALL be (all `blind = FALSE`, `random = 0`):

- CALL_THE_ELEMENTS (level 1; a GF_ELEMENTAL_GROWTH ball of radius
 `1 + level(5, 0)`, directional at >= 17);
- CHANNEL_ELEMENTS (level 3; the `channel_the_elements` nine-terrain effect table -
 grass/flower heal, dark pit dark/nether bolt, shallow-deep water bolt or beam,
 ice bolt or ball, sand into a burning-blinding shield, shallow-deep lava
 fire/hellfire bolt-ball; after casting, a `magik(100 - level)` chance exhausts
 the grid into a floor (flower into grass));
- ELEMENTAL_WAVE (level 15; the adjacent terrain decides the GF pair and damage
 amount, at >= 20 the second GF is used (lava upgrades to holy fire), the terrain
 becomes floor, then `fire_wave` EFF_WAVE+EFF_LAST+EFF_DIR<dir>);
- VAPORIZE (level 4; depend Air >= 4; the stepped-on terrain decides the GF of a
 self-centered fire cloud, at >= 20 upgraded to the second GF, the terrain becomes
 floor);
- GEOLYSIS (level 7; depend Earth >= 7; `geomancy_dig` of length `5 + level(12)`);
- DRIPPING_TREAD (level 10; depend Water >= 10; the `dripping_tread` counter is
 persisted, and HOOK_MOVED lays a random floor per grid until the count reaches
 zero);
- GROW_BARRIER (level 12; depend Earth >= 12; a GF_ELEMENTAL_WALL ball of radius
 one, directional at >= 20);
- ELEMENTAL_MINION (level 20; the adjacent terrain picks the monster name pool per
 the four-element table (wall system earth elementals/Xorn/Xaren, dark pit air
 system four names, sand-lava-fire system two names, ice-water system three
 names), `get_skill_scale` caps the pool by skill, `rand_range` draws a name,
 `test_monster_name` summons it as an ally with `set_level` to `10 + level(120)`,
 and the terrain becomes floor).

- **Anchors**: `lib/scpt/s_geom.lua:3-43` (wall roll and GF_WALL),
 `lib/scpt/s_geom.lua:45-91` (floor roll and GF_GROWTH), `lib/scpt/s_geom.lua:93-124`
 (Call), `lib/scpt/s_geom.lua:127-267` (channel table and Channel),
 `lib/scpt/s_geom.lua:269-329` (Wave), `lib/scpt/s_geom.lua:331-385` (Vaporize),
 `lib/scpt/s_geom.lua:387-455` (dig pieces), `lib/scpt/s_geom.lua:457-488`
 (Geolysis), `lib/scpt/s_geom.lua:490-535` (Dripping Tread),
 `lib/scpt/s_geom.lua:537-570` (Grow Barrier), `lib/scpt/s_geom.lua:572-656`
 (Elemental Minion)
