# monster-death Specification

## Purpose

The monster death pipeline: `src/xtra2.c` carries `mon_take_hit` (player-side death
check, experience conversion, and fear rolls), `monster_death` (drops, special
by-name scenes, self-destruction, corpse placement), `place_corpse` (corpse and
skeleton creation), and `get_coin_type` (coin monsters). The monster-vs-monster
version `mon_take_hit_mon` is specified in specs/monster-ai/spec.md; quest
completion checks are specified in specs/quest/spec.md.

## Requirements

### Requirement: Death Check In mon_take_hit

`mon_take_hit` SHALL apply player-caused damage to a monster and resolve death,
experience, piety, and fear.

- While the monster is tracked, the health bar is redrawn (`PR_HEALTH`).
- `RF7_NO_DEATH` monsters cannot die.
- The monster wakes and the damage is subtracted.
- When hp drops below 0:
 - a possessor first returns to life via `ai_deincarnate` (the monster survives);
 - `RF7_DG_CURSE` monsters put a Morgothian curse on the player half the time
 (`curse_equipment_dg(100, 50)` plus a loop of `2 + randint(5)` calls to
 `activate_dg_curse`);
 - with `speak_unique` and `RF2_CAN_SPEAK`, the monster reads dying words from
 `mondeath.txt`;
 - the death wording is picked by attack channel: a `note` is appended; on a
 physical hit against an unseen monster "You have killed" is used; DEMON / UNDEAD
 / STUPID / NONLIVING / an `Evg` `d_char` gets "You have destroyed"; the rest get
 "You have slain";
 - when the monster's status is below `MSTATUS_FRIEND`, the player gains
 `mexp x monster level / max_plv`, with the remainder carried into `exp_frac` as a
 0x10000-scale fraction, via `gain_exp`;
 - on a physical kill (`note` empty) a weapon with `TR4_LEVELS` additionally gains
 half-rate experience and runs `check_experience_obj`;
 - a unique is permanently destroyed (`max_num = 0`) and unfinished random quests
 targeting it are invalidated;
 - `monster_death` runs.
- Piety SHALL shift with the kill: GOOD monsters cost Eru `7 x level`, Manwe
 `10 x level`, and pay Melkor `3 x level`; any other monster pays Melkor
 `1 + level/2`.
- For EVIL monsters: a praying Manwe follower gains `level/2`; Tulkas gains half of
 that unconditionally, and a praying Tulkas follower gains half again, plus the full
 amount for demons.
- NONLIVING / DEMON / UNDEAD kills pay Yavanna `level/2` (minimum 1).
- A praying Yavanna follower loses `level/2` for any kill, and three times that much
 again for killing an animal.
- In the `absorb_soul` state (against monsters that are neither undead nor
 non-living) the player heals `1 + level/2 + get_skill_scale(SKILL_NECROMANCY, 40)`.
- A unique kill is noted as "Killed %s" (tag `'U'`) when `take_notes` and
 `auto_notes` are on.
- Visible or unique kills increment `r_pkills` / `r_tkills` and call
 `monster_race_track`.
- `delete_monster_idx` removes the monster and the function returns dead.
- The survival path SHALL roll fear: pain reduces fear (`randint(dam)` cut from
 `monfear`); a monster with no fear and no `RF3_NO_FEAR` becomes afraid when (hp at
 10% or less and `rand_int(10) < percentage`) or (`dam >= current hp` and
 `rand_int(100) < 80`), gaining `monfear = randint(10) + ((dam >= current hp and
 percentage > 7) ? 20 : (11 - percentage) * 5)`.
- **Discrepancy:** the source comment claims the fear path triggers "when hit for
 half its current hit points", but the code tests `dam >= m_ptr->hp` — the whole
 current hp.

#### Scenario: Possessor survives death

- **WHEN** a possessed monster drops below 0 hp
- **THEN** `ai_deincarnate` runs, the monster survives, and the function reports it
 as not dead

- **Anchors**: `src/xtra2.c:4533-4884`

### Requirement: Death Handling In monster_death

`monster_death` SHALL run the death side effects and the drop.

- The `HOOK_MONSTER_DEATH` hook fires.
- A `COMPANION` kill increments `companion_killed`.
- In player undead form, `necro_extra2` counts down; at zero the player gets
 "Your death has been avenged", a revival, and the `CLASS_UNDEAD` flag cleared,
 otherwise the remaining count is announced.
- An arena win sets `p_ptr->exit_bldg` and increments `arena_number`.
- A clone is flagged via `cloned`; a DOPPLEGANGER death clears the doppleganger.
- Every carried object is dropped with `delete_object_idx` followed by `drop_near`,
 counting into `dump_item` / `dump_gold`.
- `object_level` is set to `(dungeon level + monster level) / 2`.
- By-name special scenes SHALL fire:
 - Stormbringer — forges `SV_BLADE_OF_CHAOS` (to_h/to_d 16, dd/ds 6, pval 2,
 vampiric plus the strength and constitution stats plus blows, free action plus
 hold life plus resist
 nexus/chaos/nether/conf, the four ignore flags, NO_TELE, CURSED|HEAVY, and half
 the time DRAIN_EXP, otherwise AGGRAVATE);
 - "the Dawn" — 19 times out of 20, a `scatter(20)` placement around the player adds
 a SUMMON_DAWN
 replacement (side follows the original monster's stance);
 - an Unmaker — self-destructs with `project(GF_CHAOS, 100, radius 6)`;
 - "ink horror" — `scatter(3)` around the player places two SUMMON_BLUE_HORROR
 monsters;
 - `RF1_DROP_CHOSEN` — Morgoth drops ART_GROND and the ART_MORGOTH crown (through
 `apply_magic(-1, T, T, T)`), Smeagol drops the invisible ring, a NAZGUL drops
 `SV_RING_SPECIAL` through `create_artifact` with an "of %s" inscription, and the
 rest follow a name table (Marda/the ring of Marda 50%, Saruman/Palantir 30%,
 Hagen/Nimloth 66%, Durin's Bane/Calris 60%, Gothmog/Gothmog 50%, Eol/Anguirel
 50%; in wizard mode the roll always succeeds), pre-forged and passed through
 `random_artifact_resistance` with `cur_num = 1`;
 - `RF9_WYRM_PROTECT` — announces the protector text and places a Great Wyrm of
 Power (original stance) with `scatter(6)`.
- Self-destruction and the corpse SHALL be handled: an `RBM_EXPLODE` blow maps its
 effect onto a GF (HURT to MISSILE, POIS/DISEASE to POIS, UN_BONUS to DISENCHANT,
 ACID/ELEC/FIRE/COLD directly, HALLU/CONFUSE to CONFUSION, SHATTER to ROCKET, TIME
 to TIME, the rest to MISSILE) and then calls `project(m_idx, 3, damroll)`.
- A corpse is placed via `place_corpse` on a `magik(10 + skill scale of
 SKILL_PRESERVATION at 75)` roll, when the monster lacks `MFLAG_NO_DROP`.
- A visible monster with drops runs `lore_treasure`.
- The closing `create_stairs` branch is currently unreachable (the local variable is
 always FALSE).

#### Scenario: Unmaker self-destruct

- **WHEN** a monster named as an Unmaker dies
- **THEN** it explodes with a chaos projection of 100 damage and radius 6

- **Anchors**: `src/xtra2.c:3917-4501`

### Requirement: Corpse Casting

`place_corpse` and `get_coin_type` SHALL create the remains and resolve coin drops.

- With `RF9_DROP_CORPSE`, a `SV_CORPSE_CORPSE` is forged: a unique corpse additionally
 becomes aware and is artifact-ized with `name1 = 201`; `pval` (decay period) is
 `weight + rand_int(weight)`; `weight` is `weight + rand_int(weight)/10 + 1`;
 `pval2` is the `r_idx`; `pval3` is `(maxroll + player mhp) / 2`, reduced by a
 one-third slice (`randint(pval3) / 3`).
- With `RF9_DROP_SKELETON` and no corpse flag, a `SV_CORPSE_SKELETON` is forged:
 `pval = 0`, `weight = weight/4 + rand_int(weight)/40 + 1`; a unique skeleton
 likewise becomes aware and gets `name1 = 201`.
- `found` is set to `OBJ_FOUND_MONSTER` with the four aux values (`r_idx`, ego,
 dungeon type, level) and the object is placed with `drop_near`.
- `get_coin_type` SHALL return a coin type for `d_char` '$' races by name substring
 (copper 2, silver 5, gold 10, mithril 16, adamantite 17, each in the leading-space
 and capitalized spellings), and 0 for everything else.

- **Anchors**: `src/xtra2.c:3778-3802` (coin type), `src/xtra2.c:3808-3900`
 (corpses)
