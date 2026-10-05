# capacities Specification

## Purpose

Global capacity declarations: `lib/edit/misc.txt` declares with `M:` directives the entry
capacity of every word-list, the player-related subkind capacities, and the runtime array
caps; these caps take effect first during startup, and the word-lists then load in their
fixed order. The file does not go through the template word-list parser but is handled by
the generic directive parser (the same parsing entry as the level layout files).

## Requirements

### Requirement: Capacity Declarations Precede Word-List Loading

Startup SHALL load misc.txt first so every capacity cap is in place, and only then load
the word-lists in their fixed order (skills, abilities, alchemy, players, features, objects,
artifacts, sets, ego-items, randarts and more); any word-list parser rejects entries beyond
the capacity.

#### Scenario: Order guarantee

- **WHEN** the startup loads the word-lists
- **THEN** every capacity cap is already in place from misc.txt, and over-capacity entries
 are refused at load time

- **Anchors**: `src/init2.c:6751-6752` (loaded first), `src/init2.c:5960` (misc.txt entry), `src/init2.c:6762-6799` (the subsequent load sequence)

### Requirement: Directive Shape

An `M:` line SHALL have the shape `M:<key>:<value>` (tokenized on colons or slashes, at
least two segments); the key is a single letter, and the player keys carry a second-level
subkey `M:P:<subkey>:<value>`. Failing to split two segments is a parse failure.

#### Scenario: Tokenizing

- **WHEN** a directive line yields a key segment and a value segment
- **THEN** the key letter routes to the matching capacity assignment

- **Anchors**: `src/init1.c:11547-11550`

### Requirement: Word-List Capacity Keys

The word-list capacity keys SHALL cover: town count (T), non-random town count (t),
monsters (R), monster egos (r), objects (K), artifacts (A), item sets (s), self-attached
items (E), vaults (V), dungeons (D), traps (U), wilderness terrains (W), building actions
(B), stores (S), owners (N), features (F), alchemy recipes (a), randart parts (Z), skills
(k), abilities (b), and the wilderness horizontal (X) and vertical (Y) dimensions. Each key's
value is the maximum number of entries the matching word-list may load.

#### Scenario: Key to capacity

- **WHEN** `M:R:1078` declares the monster capacity
- **THEN** the monster word-list loads at most 1078 entries, and more are refused

- **Anchors**: `lib/edit/misc.txt:3-88` (keys and their actual values); `src/init1.c:11552-11719` (key routing)

### Requirement: Player Capacity Keys

The player capacity keys `P:` SHALL carry a second-level subkey declaring the five
subkind capacities: races (R), subraces (S), classes (C), meta classes (M), origin
histories (H).

#### Scenario: Subkey routing

- **WHEN** `M:P:H:266` declares the origin history capacity
- **THEN** the origin history entry cap is set to 266

- **Anchors**: `lib/edit/misc.txt:72-85`; `src/init1.c:11636-11659`

### Requirement: Runtime Array Caps

The `M:` directive SHALL additionally declare two runtime array caps: the maximum length of
the visible object list (O) and of the visible monster list (M).

#### Scenario: Array caps

- **WHEN** `M:O:1024` is declared
- **THEN** the visible object list capacity is set to 1024

- **Anchors**: `lib/edit/misc.txt:66-70`; `src/init1.c:11630-11634`, `src/init1.c:11661-11665`

### Requirement: Skill Capacity Compile Cap

The skill capacity key (k) SHALL be constrained by a compile-time built-in cap: a declared
value above the built-in cap is a parse failure; no other key gets this re-check.

#### Scenario: Beyond the compile cap

- **WHEN** `M:k:` declares a value greater than the built-in skill cap
- **THEN** parsing fails

- **Anchors**: `src/init1.c:11576-11580`

### Requirement: Unknown Key Tolerance

When an `M:` directive meets an undefined key letter it SHALL be silently accepted with no
effect and not treated as a parse failure.

#### Scenario: Unknown key

- **WHEN** the key letter of an `M:` line is outside the defined set
- **THEN** the line parses successfully and no capacity is modified

- **Anchors**: `src/init1.c:11547-11722` (the key routing has no else error branch); note: the
 non-random town count below 20 is a file-comment convention only, not parser-enforced
 (`lib/edit/misc.txt:6-7`)
