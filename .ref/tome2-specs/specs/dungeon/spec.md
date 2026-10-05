# dungeon Specification

## Purpose

The word-list contract for dungeons: how `lib/edit/d_info.txt` declares a dungeon —
name, depth range and entry requirements, floor and wall composition, drop
tendencies, lingering damage, monster generation rules (which monsters appear in
which proportions), and the level generator plus completion elements (final
object / final artifact / final guardian) — and how the engine parses and loads it
at startup. Level-generation behavior (how the word-list is used to lay out a
level, how monsters are allocated per the rules) is specified in
`specs/dungeon-generation/spec.md`. The flag and damage-type name lists are the
source tables at the anchored locations; individual entries are content data and
are not enumerated here.

The shared file skeleton is specified in `specs/edit-format/spec.md`.

## Requirements

### Requirement: Entry Skeleton and Defaults

When a dungeon entry begins, the parser SHALL inject defaults: boundary and
entrance coordinates set to -1 (undefined); fill method set to 1; monster
generation rules cleared (up to five groups, each with mode NONE, percent 0, and
an empty monster-symbol table); the four drop tendencies set to the engine's
default gene values; and the generator name set to "dungeon".

#### Scenario: Default generator

- **WHEN** a dungeon entry carries no `G:` line
- **THEN** its generator name is "dungeon"

- **Anchors**: `src/init1.c:9392-9419`

### Requirement: Short and Long Name

A dungeon SHALL declare its name with a `D:` line: the first three characters are
the three-letter short name, and the text after one further separator character
is the long name.

#### Scenario: Two-part name

- **WHEN** a `D:` line carries text of the form `Wil:a way to the Wilderness`
- **THEN** the short name is `Wil` and the long name is `a way to the
 Wilderness` (the text starting one character after the short name)

- **Anchors**: `lib/edit/d_info.txt:14` (field header note); `src/init1.c:9428-9453`

### Requirement: Depth Range and Entry Conditions

A dungeon SHALL declare six columns with a `W:` line: minimum and maximum depth,
the player level required to enter, the index of the next dungeon pointed to once
cleared, the per-level monster allocation base, and the random-encounter
allocation weight.

#### Scenario: Six columns loaded

- **WHEN** a `W:` line carries six numeric columns
- **THEN** the depth bounds, entry level, successor dungeon, and allocation
 parameters are each stored

- **Anchors**: `lib/edit/d_info.txt:15`; `src/init1.c:9455-9476`

### Requirement: Floor Composition

A dungeon SHALL declare its floors with an `L:` line: the six-column form
`floor1:percent1:floor2:percent2:floor3:percent3` sets the floor types and both
depth-slot percents at once; the three-column form `percent1:percent2:percent3`
rewrites only the deep half's percents.

#### Scenario: Two depth slots

- **WHEN** an `L:` line uses the three-column form
- **THEN** the three percents apply only to the deep half (the second depth slot)
 and the shallow half keeps its existing values

- **Anchors**: `lib/edit/d_info.txt:16`; `src/init1.c:9478-9515`

### Requirement: Wall Composition

A dungeon SHALL declare its walls with an `A:` line: the eight-column form
`wall1:percent1:wall2:percent2:wall3:percent3:outer:inner` sets the fill types and
both depth-slot percents plus the outer and inner walls; the three-column form
rewrites only the deep half's percents.

#### Scenario: Outer and inner walls

- **WHEN** an `A:` line uses the eight-column form
- **THEN** the three fill types and percents and the outer and inner wall terrain
 indices are each stored

- **Anchors**: `lib/edit/d_info.txt:17`; `src/init1.c:9545-9584`

### Requirement: Drop Tendencies

A dungeon SHALL support an `O:` line declaring the four drop tendencies (treasure,
combat, magic, tools), overriding the entry defaults.

#### Scenario: Defaults overridden

- **WHEN** a dungeon carries an `O:` line
- **THEN** the four drop tendencies take the `O:` line values

- **Anchors**: `lib/edit/d_info.txt:18`; `src/init1.c:9517-9534`

### Requirement: Generator Name

A dungeon SHALL support a `G:` line naming the level generator (at most 30
characters), overriding the default generator "dungeon"; registration and
dispatch of generator names are specified in `specs/dungeon-generation/spec.md`.

#### Scenario: Default overridden

- **WHEN** a `G:` line carries a name
- **THEN** the dungeon's generator name is the `G:` line text

- **Anchors**: `src/init1.c:9536-9543`

### Requirement: Lingering Damage

A dungeon SHALL support up to four `E:` lines declaring dungeon-wide lingering
damage: `NdM:frequency:type` or `NdM:frequency:type-name`; the frequency is written
as a percentage and converted to per-mille (x10) at load time; the type is either a
number or a name from the engine's damage-type name table, and the line is rejected
if neither form is recognized.

#### Scenario: Frequency conversion

- **WHEN** the frequency column of an `E:` line is 25
- **THEN** that damage group's trigger frequency is stored as 250

- **Anchors**: `lib/edit/d_info.txt:19`; `src/init1.c:9586-9639` (same form as the
 terrain `E:` line)

### Requirement: Dungeon Flags and Inline Parameters

A dungeon SHALL support multiple `F:` flag lines, with flag names separated by
spaces or vertical bars. The line also supports six parameterized forms:
`WILD<a>_<b>__<c>_<d>` (wilderness entrance coordinates), `SIZE_<a>_<b>` (dungeon
size), `FILL_METHOD_<n>` (fill method), `FINAL_OBJECT_<n>` (completion object),
`FINAL_ARTIFACT_<n>` (completion artifact), `FINAL_GUARDIAN_<n>` (guardian monster
index); any other name MUST resolve in the dungeon flag name table, and an unknown
name is rejected.

#### Scenario: Completion elements

- **WHEN** an `F:` line contains a parameterized form such as `FINAL_ARTIFACT_203`
 (as the Nether Realm entry does at `lib/edit/d_info.txt:125`)
- **THEN** the dungeon's final artifact index is set to 203 (the number after the
 underscore is stored verbatim)

#### Scenario: Unknown flag

- **WHEN** an `F:` line name is neither a parameterized form nor in the dungeon
 flag name table
- **THEN** parsing fails with an unknown-flag error

- **Anchors**: `lib/edit/d_info.txt:20`; `src/init1.c:9641-9749` (six parameterized
 forms), `src/init1.c:9115-9148` (flag table lookup), name table
 `src/init1.c:829-864`

### Requirement: Monster Generation Rules

A dungeon SHALL support up to five `R:` lines declaring monster generation rules,
each carrying an appearance percent (a percentage) and a mode index; the percents
of all groups together build a flat 100-entry allocation table mapping each number
0-99 to one rule. The `M:` and `S:` lines apply to the most recent `R:` rule: an
`R_CHAR_<symbol>` parameter of an `M:` line appends to that rule's generatable
monster-symbol table (at most five; further ones are silently ignored), and its
other names resolve through the monster base-flag table; `S:` line names resolve
through the monster spell-flag table, and an unknown name is rejected.

#### Scenario: Allocation table

- **WHEN** two rules declare percents 30 and 20
- **THEN** numbers 0-29 map to the first rule, 30-49 map to the second rule, and
 50-99 keep whatever value the table already held (0 on a first load, i.e. the
 first rule)

#### Scenario: Symbol table cap

- **WHEN** a rule already holds five monster symbols and another `R_CHAR_`
 parameter arrives
- **THEN** the parameter is ignored and parsing continues

- **Anchors**: `lib/edit/d_info.txt:21-23`; `src/init1.c:9751-9784` (R: and the flat
 100 mapping), `src/init1.c:9786-9829` (M:), `src/init1.c:9831-9856` (S:),
 `src/init1.c:9149-9223` (base flag loading), `src/init1.c:9224-9267` (spell flag
 loading)
