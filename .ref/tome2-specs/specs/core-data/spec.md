# core-data Specification

## Purpose

Core data model: the entity structures and their field-semantic groups declared in
`src/types.h`. This capability is the field index for all other capabilities — fields a
requirement cites when describing behavior are registered here with their home and meaning.
Field-level detail defers to the source at the anchor.

## Requirements

### Requirement: Word-List Header

Every word-list SHALL share one header structure: the version quadruple, the record count,
the per-record length, and the four storage pool sizes (header/records/name pool/text
pool) — the name pool and text pool are the offset bases for variable-length strings.

- **Anchors**: `src/types.h:80-102`

### Requirement: Object Genealogy

The object system SHALL be built from four word-list layers: object kind (tval/sval/two
parameter values, five flag groups plus the sense bit, the five obvious-flag parallel sets,
four allocation tiers, flavor, sense/tried state, transformation target kind, artifact
occupation marker, granted powers); artifact (obvious flags, on-floor count, set membership
marker); ego item (ten applicable kind groups, five rarity groups each with five flag
groups plus sense plus ego flags, required and forbidden flag sets, name prefixes and
suffixes); randart part (twenty applicable kind groups, value and per-item cap, resistance
flag set). All four layers share the dual-track "real flags plus obvious flags" scheme —
the obvious half is known without identification.

#### Scenario: Dual-track flags

- **WHEN** an object carries both real flags and obvious flags
- **THEN** only the obvious flags apply to and are shown for the player before
 identification

- **Anchors**: `src/types.h:163-236` (kind), `:248-298` (artifact), `:305-363` (ego), `:369-414` (randart part)

### Requirement: Monster Race And Ego Title

A monster race SHALL carry nine flag groups, four attacks, six body-part segments,
appearance counts (current on-floor and maximum — a unique's continued existence is
expressed by exactly these) and the full memory field set (first sight/deaths/personal
kills/ancestral kills/awakening/ignores/drop extremes/spell sightings/attack sightings/
nine seen-flag groups). An attached ego title SHALL carry four attacks each with a pair of
modifier operators, six numeric values each with a modifier operator, the required/
excluded/added/removed four flag-word sets, a symbol allowlist and exclude list, name
prefixes and suffixes, and an override appearance.

#### Scenario: Unique survival

- **WHEN** the game evaluates whether a unique can regenerate
- **THEN** the on-floor count is judged against the maximum count (and the -1 special cap)

- **Anchors**: `src/monster1.c` (isomorphic), `src/types.h:459-548` (race), `:551-625` (ego title)

### Requirement: Grid And Effects

A cave grid SHALL carry: the info flags, the feature, the floor object chain head, the
monster index, the trap index, the special and sub-special slots (a monster trap's loaded
piece and kit), the grid inscription, the grid mana, the mimic feature, the flow cost (by
compile switch), and the persistent effect handle. Floor items and monster inventory SHALL
be organized as singly linked lists (the grid holds the chain head plus the object's next
field; held items point back at the holder's index with zeroed coordinates). A persistent
effect SHALL record duration, damage, type, center, radius, and flags.

#### Scenario: Object stacks

- **WHEN** several objects occupy one grid
- **THEN** only the chain head index sits on the grid, the rest chained through the
 objects' next fields

- **Anchors**: `src/types.h:705-734` (grid), `:777-853` (object chain fields), `:737-747` (effect)

### Requirement: Object Instance

An object instance SHALL copy the kind's kind/subkind/weight and may deviate: three
parameter slots, discount, number, object experience and level (objects can level up),
artifact index, two ego indexes, two extra-info slots (activation type among them), the
hit/damage/armor three modifiers, timeout, identification and marker bits, inscription and
random artifact name (word-list pool indexes), the five random artifact flag groups plus
sense plus the obvious set, the pseudo-identification state, and the origin quadruple
(acquisition route plus dungeon/depth auxiliary information).

#### Scenario: Origin tracing

- **WHEN** an object is generated from a monster drop
- **THEN** the origin fields record the route, monster, ego, dungeon, and depth quadruple

- **Anchors**: `src/types.h:777-853`

### Requirement: Player Structure

The player structure SHALL have three segments: the resident segment (identity quadruple
plus subrace and spec, mimic form, pack, the HP/mana/sanity triples of current, maximum,
and fraction, the HP modifier, experience, piety with god and praying state, five sets of
the six stats (maximum/current/modifier/index/temporary counter), the three luck values,
roughly sixty timed states, the resistance/immunity/held triple families of booleans, the
eight skills, the double modifiers (display and real kept apart, melee and missile listed
separately), the antimagic field, the disposition boolean family, the five random artifact
outer flag groups plus sense, the corruption table, pet settings, possession, the
transformed body, the destiny marker, prophecy, and the out-of-race flag channel), the
scratch segment (the notice/update/redraw/window flag groups) and the behavior segment
(the leaving marker). The player SHALL additionally hold the loan amount and repayment
deadline, the companion death counter, and the tactical and movement preferences.

#### Scenario: Double modifiers

- **WHEN** the player views the character screen
- **THEN** the display bits show the known modifiers while combat resolution uses the real
 bits — the two sets are maintained independently

- **Anchors**: `src/types.h:1451-1876` (player structure; double modifiers `:1743-1758`; timed states `:1534-1617`)

### Requirement: Character Definition Layers

Character definition SHALL stack race, subrace, and class: the three layers share the four
skill arrays (base/base operator/modifier/modifier operator), the per-layer level-flag
ladders (flag groups plus parameters per level), the five birth-object slots and the
ten-entry ability prerequisite table; the race additionally carries the body plan and the
origin charts, the subrace name prefixes and suffixes plus the class allow/deny masks, and
the class the title ladder, the eight extra skills, the six spell-book parameters, the
attack parameters, the perception parameters, the god mask, and the spec list; a meta class
SHALL compose classes as a dynamic sequence.

#### Scenario: The four skill arrays

- **WHEN** the birth process stacks racial skill values
- **THEN** the base is merged through its operator and the modifier through its operator,
 two independent operations

- **Anchors**: `src/types.h:1131-1205` (race), `:1207-1287` (subrace), `:1327-1416` (class and spec `:1294-1325`), `:1418-1424` (meta class)

### Requirement: Quests And Random Quests

A quest SHALL carry the silent marker, the dynamic description switch, the name, ten
description lines, the status, the depth, the plot pointer (quests of one line share a
single pointer, which is what makes switching work), the C/Lua type, the C init function,
and four data slots. A random quest SHALL register per level the type (kill count), the
target monster, and the done bit.

- **Anchors**: `src/types.h:2265-2288` (quest), `:2289-2295` (random quest)

### Requirement: Stores And Buildings

A store runtime instance SHALL hold the owner reference, insult counter, good and bad
purchase counters, the open-until deadline, the last visit, and the stock array; the store
word-list SHALL hold the legal item table, the capacity cap, the four owners, the six
actions, and the appearance; an action SHALL hold the three cost tiers, the primary and
fallback letters, the action code, and the restriction tier; an owner SHALL hold the purse
cap, the markup range, the haggling step, the eviction cap, the liked/hated race and class
bitmasks, and the three cost tiers.

- **Anchors**: `src/types.h:1008-1098`

### Requirement: Skills And Spells

A skill SHALL carry the initial value and modifier each with an operator, the current value
and modifier, the decay rate, the usage count, the relation matrix to all skills, the tree
structure (parent, development state, order), the hidden state, the random-acquire chance,
and flags. A spell SHALL carry the name, the skill level needed to learn it, the level-1
and max-level mana costs, the minimum failure rate, and the spell level (zero means not
learned); a school SHALL hold only the name and the linked skill. A power SHALL carry the
cost, the acquired bit, and the three prerequisite slots (ten skills, six stats, ten powers
plus ten exclusivities).

- **Anchors**: `src/types.h:2442-2470` (skill), `:2476-2492` (spell and school), `:2532-2551` (power)

### Requirement: Dungeons And Wilderness

A dungeon SHALL carry the name, the generator name, the three floor types and three fill
types with their two-part ratios each, the inner and outer walls, the smoothing parameters,
the depth range, the main-line membership and successor, the entry level, the monster
allocation parameters, the two flag words, the expected panel count, the whole-hundred
monster rule allocation table with five rules, the completion object/artifact/guardian,
the wilderness entrance and exit coordinates, the drop tendencies, the four persistent
damage groups, and the linked town table. The wilderness SHALL be composed of block
information (name, entrance, road bit, difficulty, flags, features, the eighteen-segment
terrain table) and map grids (feature, random seed, entrance, known bit); a town SHALL hold
its seed, the store array, the store count, and the destroyed bit.

- **Anchors**: `src/types.h:2145-2205` (dungeon), `:1928-1958` (wilderness), `:1964-1977` (town)

### Requirement: Supporting Structures

The rest SHALL be registered per domain: the trap word-list (probability/stacking/parameter
contribution/difficulty/minimum level/color/flags/identification/damage dice); the vault
(type/rating/size/quest depth and dungeon/ten monsters three objects); word-list headers
and the assorted description pairs (tval with name); fates (type/level/certainty/the five
target indexes/quantity/deadline/foreseen bit); timers (chained, delay, countdown, Lua
callback); the player birth-roll snapshot (race/class/subrace/spec, god and piety, the six
stats, the four origin history rows); hook chains (function pointer, name, script, type,
next) and the hook return union; origin history (text, weight, chart, successor, social
class bonus); item sets (six pieces each with six parameter segments); alchemy recipes and
artifact optional flags (group, level, experience, parameter bit, six corpse flag slots);
the martial arts and random spell tables, the power and activation and song description
pairs; the between-gate exit correspondence table; the remaining CLI commands besides
fates, the tactic and movement preference tables, and the monster power table.

- **Anchors**: `src/types.h:657-675` (trap), `:633-652` (vault), `:2094-2111` (fate), `:2507-2518` (timer), `:2334-2385` (snapshot and hooks), `:2390-2428` (origin history and item sets), `:1988-2011` (alchemy and optional flags), `:1879-2092` (remaining miscellany)
