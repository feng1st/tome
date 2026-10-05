# monster Specification

## Purpose

The word-list contract for monsters: how `lib/edit/r_info.txt` declares one monster —
identity, appearance, base numbers, body-part slots, drop tendencies, blow groups, and
flags — and how the engine parses, validates, and loads it at startup.

This capability covers only the word-list format and its load semantics. The runtime
effects of the flags (UNIQUE, FRIENDS, DROP_CORPSE, ...) are specified in
specs/monster-generation/spec.md, specs/monster-ai/spec.md, specs/monster-melee/spec.md,
and specs/monster-death/spec.md; memory is in specs/monster-memory/spec.md. The complete
name lists for flags, blow methods, and blow effects are not reproduced here; the
source tables at the anchors are authoritative. Entries are content data and are not
analyzed one by one.

## Requirements

### Requirement: Version Line Precedes Entries

A monster word-list file SHALL declare its format version with a `V:` line carrying the
version triple. The version line MUST appear before any entry line; when the parser
meets an entry line before the version line it rejects the file. When the version-verify
compile switch is enabled, the declared version MUST match the engine's built-in
version.

#### Scenario: Entry before version is rejected

- **WHEN** the first valid line of the word-list file is an entry line instead of a
 `V:` line
- **THEN** parsing fails and returns error 2 (`obsolete file`)

- **Anchors**: `lib/edit/r_info.txt` (header `V:` line), `src/init1.c:7709-7739`

### Requirement: Include Directive

The word-list SHALL support `<` lines that pull in other data files; the referenced
file's content is inlined and parsed line by line.

#### Scenario: Inline expansion

- **WHEN** parsing meets a line starting with `<`
- **THEN** the parser pushes the named file onto the line-source stack and subsequent
 lines are read from the referenced file

- **Anchors**: `src/init1.c:7742-7746` (`fp_stack_push`)

### Requirement: Entry Start And Index

An entry SHALL start with an `N:` line carrying the index and the name: the index MUST
be no smaller than the previous entry's index and MUST not pass the word-list capacity;
the name MUST be non-empty. When an entry starts, the four drop tendencies are set to
the engine's default genetic values, and the innate and spell casting frequencies are
zeroed. The first entry has index 0 and is named Player.

#### Scenario: Index rollback is rejected

- **WHEN** a new entry's index is smaller than the index of the last completed entry
- **THEN** parsing fails and returns error 4 (`non-sequential records`)

#### Scenario: Entry default values

- **WHEN** an entry begins parsing
- **THEN** its drop tendencies are the four default genetic values and its casting
 frequency is 0, until an `O:` or `S:` line overrides them

- **Anchors**: `lib/edit/r_info.txt:191` (`N:0:Player`), `lib/edit/r_info.txt:198`
 (first regular entry), `src/init1.c:7748-7799`

### Requirement: Description Line Concatenation

An entry SHALL support `D:` lines carrying description text; multiple description lines
are concatenated, in order of appearance, into the entry's single description.

#### Scenario: Multi-line description

- **WHEN** one entry carries several `D:` lines
- **THEN** the line texts are appended in order into one description string

- **Anchors**: `src/init1.c:7805-7825`

### Requirement: Appearance Symbol And Color

An entry SHALL declare its map display symbol and color with a `G:` line: the symbol is
a single character, and the color is a single-letter color name that is translated to
an engine color value; a color name outside the color table is rejected.

#### Scenario: Color name translation

- **WHEN** a `G:` line gives a symbol and a color-name letter
- **THEN** the symbol and the matching engine color value are written into the entry's
 appearance fields

- **Anchors**: `src/init1.c:7827-7853`; for special appearance semantics
 (transparent/multihued and the like) see the header comment at
 `lib/edit/r_info.txt:24-45`

### Requirement: Base Stat Line

An entry SHALL declare its base numbers with an `I:` line of five colon-separated
fields carrying six numeric values: speed, hit dice as `NdM` (stored as a separate dice
count and side count), perceptiveness (`aaf`), armor class, and sleepiness. All six
numeric values MUST be present; a missing value rejects the line. Only the dice count
and side count are recorded — actual hit points are rolled when the individual monster
is created.

#### Scenario: Stat loading

- **WHEN** an `I:` line carries the five fields `110:2d2:8:7:30`
- **THEN** speed=110, hit dice=2d2 (2 dice, 2 sides), aaf=8, armor=7, and sleepiness=30
 are stored in the entry

- **Anchors**: `lib/edit/r_info.txt:123` (field-order header note),
 `lib/edit/r_info.txt:1338-1352` (giant white rat entry), `src/init1.c:7855-7874`

### Requirement: Body Part Slots

An entry SHALL declare the slot counts of six body-part kinds with an `E:` line (weapon,
torso, arms, fingers, head, legs); the weapon slot count MUST NOT exceed the arms slot
count, and violating this aborts startup.

#### Scenario: More weapons than arms

- **WHEN** an `E:` line gives a weapon slot count greater than the arms slot count
- **THEN** the engine aborts startup with an error message that includes the entry index

- **Anchors**: `lib/edit/r_info.txt:125`, `src/init1.c:7876-7898`

### Requirement: Drop Tendencies

An entry SHALL declare four drop tendencies (treasure, combat, magic, tool) with an
`O:` line, overriding the defaults injected by the `N:` line.

#### Scenario: Overriding the defaults

- **WHEN** an entry carries an `O:` line
- **THEN** the four drop tendencies take the `O:` values instead of the default genetic
 values injected by the `N:` line

- **Anchors**: `lib/edit/r_info.txt:126`, `src/init1.c:7900-7917`

### Requirement: Level And Reward Line

An entry SHALL declare level, rarity, corpse weight, and kill experience with a `W:`
line; a corpse weight of zero is loaded as 100.

#### Scenario: Weight fallback

- **WHEN** the corpse weight column of a `W:` line is 0
- **THEN** the entry's corpse weight is loaded as 100

- **Anchors**: `lib/edit/r_info.txt:124`, `src/init1.c:7919-7939`

### Requirement: Blow Groups

An entry SHALL support up to four `B:` blow groups, each carrying a blow method, a blow
effect, and `NdM` damage dice; the method and effect names MUST be in the engine name
tables; an overflow past the four slots or an unknown name is rejected.

#### Scenario: Name table lookup

- **WHEN** a `B:` line gives a method or effect name that is not in the name tables
- **THEN** parsing fails and returns error 1 (`parse error`)

#### Scenario: Slot limit

- **WHEN** an entry already carries four blows and another `B:` line arrives
- **THEN** parsing fails and returns error 1 (`parse error`)

- **Anchors**: `lib/edit/r_info.txt:127`, `src/init1.c:7941-8000`; method and effect
 name tables `src/init1.c:46-123`

### Requirement: Base Flags

An entry SHALL support multiple `F:` base-flag lines, with flag names separated by
spaces or vertical bars; every name MUST resolve to a single bit in one of the six
base flag-name tables (`r_info_flags1`-`r_info_flags3`, `r_info_flags7`-`r_info_flags9`;
the remaining three tables are spell flags handled by `S:` lines), and an unknown name
is rejected.

#### Scenario: Unknown flag

- **WHEN** an `F:` line contains a flag name absent from the name tables
- **THEN** a message naming the unknown flag is shown and parsing fails with error 5
 (`invalid flag specification`)

- **Anchors**: `lib/edit/r_info.txt:128`, `src/init1.c:8002-8027`; name tables
 `src/init1.c:124-477`, lookup `src/init1.c:4625-4660`, base-flag loading
 `src/init1.c:7550-7624`

### Requirement: Spell Flags And Frequency

An entry SHALL support multiple `S:` spell-flag lines; the first `S:` line of an entry
MUST declare the casting frequency with `1_IN_X` (frequency = 100 divided by X); the
other names MUST resolve in the spell-flag name table, and an unknown name is rejected.

#### Scenario: Frequency conversion

- **WHEN** an `S:` line declares the frequency with `1_IN_5`
- **THEN** both the innate and the spell casting frequency of the entry are set to 20

#### Scenario: Missing frequency

- **WHEN** an entry's `S:` lines lack a `1_IN_X` declaration
- **THEN** the entry's casting frequency stays at the 0 injected by the `N:` line

- **Anchors**: `lib/edit/r_info.txt:129-130`, `lib/edit/r_info.txt:169-173` (header
 note requiring the frequency first), `src/init1.c:8029-8067`; spell-flag loading
 `src/init1.c:7625-7669`

### Requirement: Unknown Line Rejection

Any word-list line that is not a comment, a blank line, or one of the line kinds above
MUST be rejected, ending the parse with an unknown-line error.

#### Scenario: Unknown line

- **WHEN** parsing meets an undefined leading letter
- **THEN** parsing fails and returns error 6 (`undefined directive`)

- **Anchors**: `src/init1.c:7702` (comments and blank lines skipped),
 `src/init1.c:8069-8070`

### Requirement: Post-Load Normalization

After the word-list is loaded the engine SHALL run one normalization pass over all
entries above index 0: the wilderness flag bit is invert-mapped, and an entry that
declares only the all-wilderness survival flag with no other wilderness flag bits gets
its wilderness flag word set to a fixed combination value (`0x0463`).

- **Discrepancy:** the code comment claims this assignment "enables all flags", but
 `0x0463` sets only five bits (`WILD_ONLY`, `WILD_TOWN`, `WILD_WASTE`, `WILD_WOOD`,
 `WILD_GRASS`).

#### Scenario: Normalization takes effect

- **WHEN** the word-list load completes
- **THEN** every entry above index 0 has its wilderness flag word invert-mapped, and
 entries declaring only all-wilderness survival receive the fixed wilderness flag
 combination

- **Anchors**: `src/init1.c:8078-8086`
