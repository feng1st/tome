# wilderness-terrain Specification

## Purpose

The word-list contract for wilderness terrain (wilderness feats): how
`lib/edit/wf_info.txt` declares a wilderness terrain block — level, entrance
target, road direction bits, the map-symbol-to-index lookup, and the
eighteen-slot terrain table — and how the engine parses and loads it at startup.
How the wilderness map lays blocks out and how entrances are sailed to are
specified in `specs/wilderness/spec.md`. The flag name list is the source table at
the anchored location; individual entries are content data and are not enumerated
here.

The shared file skeleton is specified in `specs/edit-format/spec.md`.

## Requirements

### Requirement: Block Parameters

A wilderness terrain SHALL declare six columns with a `W:` line: level, entrance
target, road direction bits, terrain feature index, terrain table index, and map
symbol. The symbol SHALL be registered in reverse into the global symbol index
table (the wilderness map looks blocks up directly by character). The entrance
target carries two-part semantics per the header notes: below 1000 it points to a
town, at 1000 or above it points to the dungeon numbered minus 1000; the road
direction bits combine north/south/east/west = 1/2/4/8.

#### Scenario: Symbol index

- **WHEN** a `W:` line gives a symbol letter
- **THEN** the mapping from that letter to the current block index is written into
 the global index table, and wilderness map characters look blocks up through it

- **Anchors**: `lib/edit/wf_info.txt:13-29` (field and entrance/road header
 notes); `src/init1.c:10832-10855` (including the `wildc2i` registration)

### Requirement: Terrain Table

A wilderness terrain SHALL support an `X:` line declaring the eighteen-slot
terrain index table (the slot count matches the built-in wilderness terrain table
capacity, and a short line is rejected).

#### Scenario: Eighteen slots loaded

- **WHEN** an `X:` line carries fewer slots than the built-in capacity
- **THEN** parsing fails

- **Anchors**: `lib/edit/wf_info.txt:15`; `src/init1.c:10857-10879`

### Requirement: Block Flags

A wilderness terrain SHALL support multiple `F:` flag lines: names separated by
spaces or vertical bars resolve through the wilderness terrain flag name table,
and an unknown name is rejected.

#### Scenario: Unknown flag

- **WHEN** an `F:` line contains a flag name missing from the name table
- **THEN** parsing fails with an unknown-flag error

- **Anchors**: `src/init1.c:10881-10906`, name table from `src/init1.c:944`,
 loading function `src/init1.c:10659-10678`

### Requirement: Long Name

A wilderness terrain SHALL support a `D:` line carrying the long name (appended
directly).

#### Scenario: Loading

- **WHEN** a `D:` line gives text
- **THEN** the text is registered as the block's long name

- **Anchors**: `src/init1.c:10810-10830`
