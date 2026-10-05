# player-melee Specification

## Purpose

Player melee: `src/cmd1.c` carries the player-versus-monster melee computation - hit and
critical roll judgment, slay and brand multipliers, the bare-hand and Bear-form blow
tables, the Nazgul weapon backlash, chaos-weapon effects, vampiric weapons, backstab and
stabbing, the full-weapon multi-wield round, the carried symbiote's assist attacks and the
possessed body's attacks. The monster-melee-against-player counterpart is specified in
specs/monster-melee/spec.md; `mon_take_hit`/fear and death are specified with the melee2.c
material; shooting and throwing are specified in specs/player-ranged/spec.md.

## Requirements

### Requirement: Hit And Critical Roll Judgment

The hit roll SHALL run outside the fifty-percent band: the first ten results of the
percentile roll form the certain zone (half always hit, half always miss), an invisible
target halves the attack power, and power plus luck +-10 competes against three quarters
of the target's armor.

Shooting criticals SHALL roll against 5000 with power computed as ammo weight plus four
times the hit bonus plus the skill folded to 100, plus the extra critical tier and luck
+-100; melee criticals use the same formula but the hit bonus times five and the melee
style folded to 150, and a light sword (TV_SWORD under weight 50) paired with the
SKILL_CRITS skill adds get_skill_scale(SKILL_CRITS, 2000) to the power. Criticals come in
five tiers - good (2 x damage + 5), great (2 x damage + 10), superb (3 x damage + 15),
*GREAT* (3 x damage + 20), *SUPERB* (7 x damage / 2 + 25); shooting only judges the first
three tiers. The tim_deadly state SHALL skip the roll and take the *GREAT* tier directly.

- **Anchors**: `src/cmd1.c:21-74` (hit), `:82-116` (shooting criticals), `:123-186`
 (melee criticals)

### Requirement: Slay And Brand Multipliers

Weapon and ammo slays/brands SHALL take the largest multiplier per monster race flag:
slay animal and slay evil x2; slay undead, demon, orc, troll, giant and dragon x3;
*slay dragon*, *slay undead* and *slay demon* x5; elemental brands x3, x6 against
vulnerable races, no gain against immune races.

Hitting a visible monster SHALL record the matching race flags into the monster memory.
The poison brand (including the poisoned-hands state) applies SPEC_POIS with 95% chance
against vulnerable races and 50% against normal ones; the wound flag applies SPEC_CUT with
50% chance. The special effects SHALL run after the damage lands: the cutting brand makes
the monster bleed (adds twice the damage; immune races are recorded into the memory), the
poison brand poisons the monster (twice the damage against vulnerable races, against
normal races one times the damage after a level-based roll).

- **Anchors**: `src/cmd1.c:199-519` (multipliers), `:2154-2225` (special effects)

### Requirement: Bare-Hand Blows

Bare-hand attacks SHALL pick from the blow table by style: Bear mimicry with the SKILL_BEAR
skill uses bear_blows, the SKILL_HAND style uses ma_blows; the effective level is the
matching skill. One blow-selection iteration happens per seven levels (at least one),
drawing from the table by level threshold and probability and keeping the highest drawn
(stun or confusion locks in the previous selection).

The technique effects SHALL be: the knee strike triggers only on male monsters (if not
killed, the monster moans and is stunned for 7+d13, with stun resistance cut to a third);
the full-slow and ankle-kick techniques slow monsters that are neither NEVER_MOVE nor of
the special d_char set (not unique, a level roll that exceeds the monster's level, and
speed above 60 lower the speed by ten); the stun technique adds power/2 + randint(power/2)
stun (it lands when the effective level exceeds a roll of level + resistance + 10 - unique
adds 88, no-confusion and no-sleep each add 44, undead and nonliving add 88); the wound
technique applies SPEC_CUT with a power-based probability. Damage goes through the
critical table with effective level x d10 as the weight.

- **Anchors**: `src/cmd1.c:2231-2381`

### Requirement: Nazgul Weapon Backlash

Striking a Nazgul SHALL be judged by the weapon tier: a plain weapon (no ego, not an
artifact) disintegrates on the spot and ends the round's attacks; a mundane ego weapon
(without slay evil, slay undead or *slay undead*) deals zero damage and has a 25% chance
to be destroyed; a mundane artifact deals zero damage, suffers the draining strike, and
has a 1/1000 chance to be destroyed; TR5_RES_MORGUL is immune to disintegration.

Dealing any damage SHALL carry a 25% chance to inflict black breath on the wielder.

- **Anchors**: `src/cmd1.c:2387-2474`

### Requirement: Melee Main Flow

`py_attack` SHALL gate first: the current form with NEVER_BLOW cannot attack; an
RF7_IM_MELEE target is immune; a frightened player cannot attack; attacking breaks invuln
and disrupt_shield, wakes the target, and records the picture and health bar when visible.

A friendly target SHALL stop the attack - the player makes way - unless the wielded
weapon's artifact name is 'Stormbringer', in which case the black blade greedily attacks
the friendly monster and the attack proceeds (no probability roll is involved).

The backstab SHALL, against a visible sleeping target, multiply damage by the backstab
skill folded to 100, and against a visible fleeing target by seventy; a miss voids the
backstab.

The weapon round SHALL: the mastery style attack once per body weapon part, bare-hand a
single blow; weapons with the no-attack flag are skipped; each hit stacks in order -

- a chaos weapon draws an effect with 50% chance (of the draws: vampiric on a 3-in-5 roll,
 otherwise an earthquake on a 1-in-250 roll, otherwise confusion on a 9-in-10 roll,
 otherwise teleport away or polymorph fifty-fifty);
- a vampiric weapon records the target's current hp;
- a vorpal blade triggers the chain deep cut on a 1-in-6 roll (damage accumulates until a
 1-in-4 roll breaks the chain);
- weapon damage passes through the multiplier and critical tables;
- the impact brand triggers an earthquake when the damage exceeds fifty or on a 1-in-7
 roll;
- a critical with a blunt weapon heavier than fifty and the SKILL_STUN skill on hit adds a
 stun (against targets without sound or wall breath, capped at 200);
- the Tulkas prayer (a wisdom_scale(130) minus monster level roll, with piety above 1000
 doubling the damage bonus);
- the timed projection adds the projectile;
- the Melkor curse (with spell level ten or above and piety above 5000, casts a spell with
 probability wisdom_scale(30) x level / monster level);
- the clone flag duplicates the monster with 30% chance.

Damage lands with to_d plus to_d_melee added; the striking power scales its energy cost
down by the hit index. Hitting a friendly monster turns it hostile. The vampiric drain
SHALL restore 4d(one sixth of the difference) health, at most 100 per round. The
earthquake brand SHALL, after all attacks, trigger a radius-10 earthquake centered on the
player (except in quest levels and towns).

- **Discrepancy:** the chaos-draw code comments label the effect chances
 20%/0.12%/26.9%/1.49%/1.49%, which match neither the conditional nor the overall rolled
 rates.

#### Scenario: Backstab bonus

- **WHEN** a player with the backstab skill attacks a visible, sleeping monster
- **THEN** the weapon damage gains the backstab skill folded to 100 percent and the
 sneak-attack text plays

- **Anchors**: `src/cmd1.c:2482-3075` (main flow), earthquake landing `:3068-3074`

### Requirement: Symbiote And Possession Attacks

A carried symbiote SHALL attack the player's target monster: from the carried monster's
race, the four-blow table projects GF effects (armor folded in, the crush impact above 23
triggers a radius-8 earthquake, after stealing gold or objects both flee together with 50%
chance each, paralysis substitutes monster level for damage), both sides' auras burn each
other, and when it flees the player and symbiote teleport away together.

A possessed body attacking bare-handed SHALL substitute the body_monster's blow table for
the player's attack: first-person messages, damage adds the player's to_d, paralysis
substitutes twice the player level for damage, and the blow count follows the player's
attack count. **Quirk:** the loop bound is broken by ternary-operator precedence and in
practice relies on the blow table's method reaching zero to stop - as implemented.

- **Anchors**: `src/cmd1.c:824-1466` (symbiote), `:1472-2099` (possession, precedence
 state at `:1525`)

### Requirement: Damage Messages

Damage messages SHALL pick one of five tiers by the damage's percentage of the target's
max hp (below 5/30/60/95 picks scratch/hit/wound/cripple, otherwise demolish); monsters of
the special d_char set are never embellished. An insane attacker picks from the
dam_none/med/lots/huge/xxx five-word ladder by the insanity percentage instead.

- **Anchors**: `src/cmd1.c:2106-2148`

### Requirement: Whirlwind Attack

The whirlwind attack SHALL strike every monster in the eight adjacent squares, one blow
each.

- **Anchors**: `src/cmd1.c:5485-5501`
