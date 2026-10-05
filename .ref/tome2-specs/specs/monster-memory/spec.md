# monster-memory Specification

## Purpose

Monster recall and the monster library: how the player's accumulated knowledge of a
monster is presented — kill history, depth, speed, experience value, attack methods
and effects, breath and spell lists with their frequency, armor and hp, special
abilities, vulnerabilities / immunities / resistances, sleepiness, drops — plus the
withholding rules when knowledge is insufficient. Also the monster habitat filter
hooks (filtering generatable monsters against the nine wilderness/dungeon terrain
types and against water and lava fit) and the terrain passage check. The kill
counters accumulate in `src/xtra2.c` (see specs/monster-death/spec.md), the waking
and casting counters in `src/melee2.c` (see specs/monster-ai/spec.md).

## Requirements

### Requirement: Knowledge Gates

Recall content SHALL be disclosed by knowledge amount: armor becomes known once the
kill count exceeds `304 / (4 + level)`; a unique gets a second gate at
`304 / (38 + (5 x level) / 4)`; attack damage becomes known once
`(4 + level) x observed blows` exceeds `80 x dice x sides`, with a unique's observed
blows counted double. Flag disclosure rules: the unique, gender, group, and escort
flags are always visible; race flags and the forced-depth/force-maxhp flags become
visible after at least one kill; every other flag is presented as the intersection
of the actually observed bits with the true values.

#### Scenario: Unique gate

- **WHEN** a unique's armor knowledge is evaluated
- **THEN** the divisor is the much larger `38 + (5 x level) / 4`, so the required
 kill count is far below a normal monster's at the same level

- **Anchors**: `src/monster1.c:38-89`, flag disclosure `src/monster1.c:219-261`

### Requirement: Combat History Narration

The recall SHALL narrate the combat history first: a unique is narrated as "killed
our ancestors / avenged / not yet avenged"; a normal monster is narrated over three
states — killed this life, killed in past lives, never yet defeated. With the detail
option on, the recall SHALL append the monster's description text and, depending on
whether the unique still lives, announce that the player has slain it personally.

#### Scenario: Unavenged unique

- **WHEN** a unique has killed the player's ancestors and still lives
- **THEN** the recall states that it has not faced retribution

- **Anchors**: `src/monster1.c:264-357`

### Requirement: Numeric Disclosure

The recall SHALL disclose by knowledge: the usual depth (times 50 in feet mode,
out-of-depth monsters shown in red); movement speed (110 as the norm, described from
very slow to very fast across the 90/100/120/130 thresholds, with the random-move
flag as a two-tier modifier); kill experience (monster experience times level
divided by player level, with a two-decimal fraction and English ordinals); the
aura (fire / electric / both); reflection; escort or group; casting frequency (above
one hundred observations the exact "one in a hundred" style is given, otherwise an
approximation rounded up to a multiple of ten); armor and hit dice (the force-maxhp flag
shows the maximum value); sleepiness (eleven wording tiers by the sleep value, plus
the perception radius in tens of feet); drop limits (gold and items counted together
with good/great qualifiers).

#### Scenario: Frequency approximation

- **WHEN** a monster's spell observations are below one hundred
- **THEN** the frequency is rounded up to a multiple of ten and shown as an
 approximation

- **Anchors**: `src/monster1.c:453-495` (depth), `src/monster1.c:498-579` (speed),
 `src/monster1.c:582-649` (experience), `src/monster1.c:651-690` (aura and groups),
 `src/monster1.c:879-909` (frequency), `src/monster1.c:912-934` (armor and hp),
 `src/monster1.c:1182-1236` (sleepiness), `src/monster1.c:1239-1325` (drops)

### Requirement: Ability List Disclosure

The recall SHALL list, each filtered by its knowledge bits: the six innate attacks,
the 22 breaths, the 67 spells (with the intelligent-casting modifier), the nine
special abilities (open door / bash door / pass wall / kill wall / move body / kill
body / take item / kill item / has light), personality lines such as invisible, the
seven vulnerabilities, the five immunities, the six resistances, and the four
free-states (stun / fear / confusion / sleep). Physical attacks SHALL list each blow
group's method (of the 24) and effect (of the 34), with the damage dice annotated
once known; with no attack flags at all the recall reports no physical attack, and
when nothing at all is known it reports total ignorance.

#### Scenario: Immunity listing

- **WHEN** a monster has fire immunity and poison immunity
- **THEN** the recall lists them in its immunity sentence form

- **Anchors**: `src/monster1.c:693-876` (innate / breath / spells),
 `src/monster1.c:938-1005` (abilities and personality),
 `src/monster1.c:1008-1179` (vulnerabilities / immunities / resistances /
 free-states), `src/monster1.c:1328-1603` (methods and effects)

### Requirement: Monster Library Presentation

The monster library title line SHALL prefix non-uniques with the definite article,
join the names of title-carrying monsters (egos) according to their before/after
marker, and show both the standard and the optional attribute colors (monochrome
mode converts to white, big-word mode pads the attribute letter). The full-screen
recall, the window recall, and the in-line description — three entries — SHALL share
one rendering core. Cheat omniscience SHALL temporarily fill the knowledge fields
and restore them at the end.

#### Scenario: Title position

- **WHEN** a monster name with a before-title is rendered
- **THEN** the title name is placed in front of the base name

- **Anchors**: `src/monster1.c:1620-1667` (title line),
 `src/monster1.c:1686-1732` (the three entries), `src/monster1.c:160-211`
 (cheat omniscience and restore)

### Requirement: Habitat Filter

Monster generation SHALL pass the habitat filter hooks: the nine wilderness terrain
types (town / deep water / shore / wasteland / grass / trees / volcano / mountain /
default dungeon) each hook their filter by the wilderness map block's terrain index;
dungeon levels always use the dungeon filter. The per-grid secondary filter SHALL
re-filter by terrain feature: shallow water excludes fire-aura monsters, deep water
allows only aquatic races, lava allows only fire-immune or flying
monsters that are not cold-aura. A random quest target SHALL additionally require
the dungeon flag, non-aquatic, and non-breeding.

#### Scenario: Lava fit

- **WHEN** a monster is drawn for a lava grid
- **THEN** only fire-immune or flying non-cold-aura monsters enter the selection

- **Anchors**: `src/monster1.c:1735-1898` (nine filters and random quest
 conditions), `src/monster1.c:1892-1932` (main hook attachment),
 `src/monster1.c:1979-1998` (secondary hook)

### Requirement: Terrain Passage

Whether a monster may enter a terrain SHALL be judged: deep water needs the
water-passing flag, flight, or swimming; shallow water repels fire-aura monsters; a
water-passing monster that cannot fly may not leave the water; lava needs fire
immunity or flight; all other terrain passes.

#### Scenario: Aquatic monster ashore

- **WHEN** a water-passing monster without flight tries to step onto normal ground
- **THEN** the passage is refused

- **Anchors**: `src/monster1.c:1938-1976`
