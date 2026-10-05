# monster-melee Specification

## Purpose

Monster melee: the full computation of a monster striking the player with its four
blows (`make_attack_normal` in `src/melee1.c`, with the symbiote pet variant
`carried_make_attack_normal`) — the hit check, the pre-hit exemption chain (dodge,
divine intervention,
pro-good/pro-evil wards), the cut and shatter attributes of the 24 attack methods
(two of them XXX placeholders), the damage and state attachment of the 34 effects,
the critical tables for cuts (bleeding) and shatters (stun), aura retaliation and
shield rebound, black breath contagion, explosion self-destruction, thief escape,
and the simplified variant for symbiote pet attacks. The ongoing evolution of the
inflicted states over time is specified in specs/player-states/spec.md.

## Requirements

### Requirement: Hit Check

The hit SHALL first roll a percentile: five percent of the values force a miss and
five percent force a hit; the rest compare the attack power (the effect's base plus
three times the level) against three quarters of the player's total armor — the hit
lands when the power roll, reduced by the luck modification, exceeds that value.
After a normal attack (HURT/SHATTER) hits, the damage SHALL be reduced by armor
(armor capped at 150, reduction at the ratio of armor out of 250).

#### Scenario: Forced band

- **WHEN** the hit percentile roll lands on 0 through 4 or on 5 through 9
- **THEN** the outcome is forced to a hit or a miss respectively, without armor or
 power taking part

- **Anchors**: `src/melee1.c:60-81`, armor reduction `src/melee1.c:1881-1893`

### Requirement: Pre-Hit Exemption Chain

After a hit, the attack SHALL pass the exemption chain in order.

- The player's dodge chance (reduced by the monster level weighting) dodges the whole
 attack on success.
- While praying to Eru, a probability derived from piety minus three hundred times
 the monster level makes an evil monster's attack miss entirely.
- The pro-good / pro-evil wards, when the player's level is not below the monster's,
 repel the matching good or evil monster on a roll of `level + 100` against 50 and
 memorize its alignment.
- A non-enemy monster may not attack (controlled monsters swap places instead).
- An `RF7_MORTAL` monster may not attack a player fated never to die at a mortal's
 hand (the `FATE_NO_DIE_MORTAL` fate sets `no_mortal`).

#### Scenario: Ward repulsion

- **WHEN** the pro-evil ward triggers against a low-level evil monster
- **THEN** the attack is turned away and the monster's evil flag is memorized

- **Anchors**: `src/melee1.c:1565-1636` (exemption chain),
 `src/melee1.c:1390-1400` (friendly and fated refusals)

### Requirement: Attack Methods

The 24 attack methods (two of them XXX placeholders) SHALL each carry their message
and attributes: the strike
family adds cuts and shatters, claw and bite add cuts, punch/kick/butt/crush add
shatters, the touch family adds nothing.

- Insult and moan have their own random line tables (the moan table switches to a
 second set for particular monsters).
- Singing monsters have fixed lyrics.
- An exploding monster SHALL destroy itself after its attack.
- A bite from a monster with vampire in its name triggers the vampiric behavior with
 5% probability (the current implementation is an empty stub).

#### Scenario: Cut/shatter exclusivity

- **WHEN** an attack method carries both the cut and the shatter attributes
- **THEN** only one of the two is taken, at random

- **Anchors**: `src/melee1.c:1641-1835` (method table), cut/shatter exclusivity
 `src/melee1.c:2820-2834`

### Requirement: Damage Effect Families

Hit damage SHALL be dispatched over the 34 effects.

- The plain and shatter families take the armor reduction and deduct hp directly
 (shatter damage over 23 triggers an earthquake of radius 8 centered on the
 monster, quest levels and towns excepted).
- The four elements run the elemental damage routine (with armor/equipment wear) and
 feed the player-resistance learning.
- Poison/disease attach poisoning (disease additionally has a 10% chance of a CON
 drain, one tenth of those permanent).
- Blind, confuse, fear, and paralysis attach according to resistance, saving throw,
 or free action thresholds (paralysis first enforces a minimum damage to prevent
 permanent helplessness).
- The six stat drains and the drain-all use the normal drain routine.
- The four experience drain tiers use hold-life probabilities of 95/90/75/50 to
 reduce the loss to one tenth.
- The time effect has three tiers (experience rollback, one stat quartered, all
 stats quartered, with a floor of 3).
- Parasite implants when no parasite is present.
- Hallucination adds the hallucination state.
- Sanity attacks run the sanity damage routine.
- Abomination morphs the player, opposed by the mimicry skill.
- UN_BONUS triggers dispel enchantment.
- UN_POWER drains the charges of non-artifact staffs and wands and heals the monster.
- EAT_GOLD (dexterity save, or steal a share of the current gold, turned into carried
 gold; the thief escapes afterward).
- EAT_ITEM (save, or steal a non-artifact backpack object — normal monsters carry it
 away, black-market monsters teleport it straight to the black market half the
 time, and under the testing carry mode it hooks onto the monster's carry pile; the
 thief escapes afterward).
- EAT_FOOD (steals one food item).
- EAT_LITE (reduces the light source fuel by 250 plus a roll, to a remainder of 1).

#### Scenario: Holding life against experience drain

- **WHEN** the player holds life and faces the level-40 experience drain tier
- **THEN** three quarters of the time the loss is held off entirely, and on failure
 the loss is reduced to one tenth

- **Anchors**: `src/melee1.c:1867-2817` (effect dispatch main table), EAT_GOLD
 `src/melee1.c:2018-2090`, EAT_ITEM `src/melee1.c:2092-2232` (black market
 `src/melee1.c:2185-2214`), experience tiers `src/melee1.c:2580-2694`, time
 `src/melee1.c:2740-2807`

### Requirement: Criticals And State Attachment

A method's cut or shatter attribute SHALL be converted through the monster critical
tables: a critical requires damage at 95% of the perfect damage (weak blows have an
extra probability gate); the critical tier comes in six grades by damage amount with
a small chance of a super double; cuts attach 1 to 500 points of bleeding by tier,
shatters attach 1 to 200 points of stun by tier.

- **Anchors**: `src/melee1.c:22-49` (critical judgment), `src/melee1.c:2837-2916`
 (the two tables)

### Requirement: Aura Rebound And Shields

Touch-family attacks SHALL trigger the player's counters: the fire aura deals 2d6 to
a non-fire-immune attacker and may destroy it on the spot (fire-immune attackers are
memorized), and the electric aura works the same way.

- The shield's rebound / fire / greater fire options retaliate with strength-scaled
 parameters (the fire version does nothing to fire-immune attackers; the greater
 fire version has no immunity check).
- The terror shield adds fear to a non-unique attacker whose damage exceeds its
 level.
- When visible, even an immunity is recorded into the lore.

#### Scenario: Fire aura burn

- **WHEN** a touch-family attack hits a player with a fire aura
- **THEN** a non-fire-immune attacker takes 2d6 retaliation and may be destroyed on
 the spot

- **Anchors**: `src/melee1.c:2936-3031`

### Requirement: Black Breath Contagion

A successful attack by an undead-family monster SHALL spread the black breath:
nazgul with a one-in-four chance, unique undead of level 35 and above with a
probability decreasing as the level rises (`300 - level` sides), normal undead of
level 40 and above likewise (`450 - level` sides). The pro-undead ward grants full
immunity throughout.

- **Anchors**: `src/melee1.c:1840-1856`, `src/melee1.c:1351-1359` (implementation)

### Requirement: Explosion And Escape

The explode attack method SHALL destroy the attacker after the attack computation by
dealing it damage of its own current hp plus one. When a theft succeeds or is saved
against, the monster SHALL
teleport beyond twice sight range plus five and announce the thief's laughing
escape.

- **Anchors**: `src/melee1.c:2926-2934` (explosion), `src/melee1.c:3084-3089`
 (escape)

### Requirement: Lore Registration

Every attack of a visible monster SHALL accumulate that blow group's seen count when
the effect is obvious, when damage is dealt, or when the blow group has already been
seen more than ten times. A player death at its hands SHALL enter the kill count. A
visible monster's fear announces its flight.

- **Anchors**: `src/melee1.c:3068-3081` (blow lore), `src/melee1.c:3092-3102`
 (kills and fear)

### Requirement: Symbiote Variant

A symbiote pet attack SHALL use the simplified computation: the damage source is
"your symbiote", with no dodge or divine-intervention saves, no black breath, no
smart learning, and no thief escape; the fire/electric aura deals no retaliation
damage but a touched attack still memorizes the attacker's fire/electric immunity,
and the shield options do nothing; the rest of the effect
dispatch and the critical tables match the main path.

- **Anchors**: `src/melee1.c:121-1345`
