# terrain Specification

## Purpose

The word-list contract for terrain: how `lib/edit/f_info.txt` declares a terrain
feature — appearance, name, the three text fields, mimicry, shimmering, lingering
damage, and flags — and how the engine parses and loads it at startup. The terrain
index is the feature constant (the file groups entries under hexadecimal index
comments); the meaning of the indices is specified under `src/cave.c` and
`src/generate.c` (see `specs/cave-lighting/spec.md` and
`specs/dungeon-generation/spec.md`). The complete flag name list is the name table
starting at `src/init1.c:790`; individual entries are content data and are not
enumerated here.

The shared file skeleton is specified in `specs/edit-format/spec.md`.

## Requirements

### Requirement: Strictly Increasing Entry Index

A terrain entry's index MUST be strictly greater than the largest index seen so
far in the file: an equal index is rejected, and re-declaring the same index is not
allowed unlike the word-lists that permit it.

#### Scenario: Same-index re-declaration rejected

- **WHEN** a new terrain entry's index equals the largest index seen so far
- **THEN** parsing fails with an index error

- **Anchors**: `src/init1.c:3812` (`i <= error_idx` is rejected)

### Requirement: Entry Defaults

When a terrain entry begins, the parser SHALL inject defaults: the mimic target is
the entry's own index; the three text fields take the engine's built-in defaults —
the seen description and the blockage message are "a wall blocking your way", and
the tunneling failure message is "You cannot tunnel through that.".

#### Scenario: Default texts

- **WHEN** a terrain entry carries no `D:` line at all
- **THEN** its seen description, blockage message, and tunneling failure message
 are all the built-in default texts, and the mimic target is itself

- **Anchors**: `src/init1.c:3709`, `src/init1.c:3726-3737` (built-in default
 texts), `src/init1.c:3835-3840`

### Requirement: Three Text Fields

Terrain SHALL support `D:` lines declaring the three text fields by subtype:
`D:0:` is the seen description, `D:1:` is the tunneling failure message, `D:2:` is
the blockage message; a missing or unknown subtype is rejected.

#### Scenario: Subtype routing

- **WHEN** an entry carries `D:0:`, `D:1:`, and `D:2:` lines
- **THEN** the three texts are registered independently and never concatenated

- **Anchors**: `lib/edit/f_info.txt:66-67` (the up-staircase `D:0:`/`D:1:` sample);
 `src/init1.c:3850-3886`

### Requirement: Mimicry

Terrain SHALL support an `M:` line declaring the mimic target: the terrain
presents itself as the target index's terrain under certain conditions; without a
declaration the mimic target is the terrain itself.

#### Scenario: Mimic target

- **WHEN** an `M:` line gives a target index
- **THEN** the terrain's mimic field takes the target index, overriding the entry
 default

- **Anchors**: `src/init1.c:3889-3903`; the run-time presentation is specified in
 `specs/cave-lighting/spec.md`

### Requirement: Shimmering Appearance

Terrain SHALL support an `S:` line declaring the shimmering appearance: seven
single-letter color names (colon separated) are translated to engine color values
and form a cycling color set.

#### Scenario: Seven-color cycle

- **WHEN** an `S:` line gives seven color-name letters
- **THEN** the seven color values are written into the shimmer array and the run
 time cycles through the seven colors

- **Anchors**: `src/init1.c:3905-3925`

### Requirement: Appearance Symbol and Color

Terrain SHALL declare its map display symbol and color with a `G:` line: the
symbol is a single character, the color name is translated to an engine color
value, and an invalid color name is rejected.

#### Scenario: Color name translation

- **WHEN** a `G:` line gives a symbol and a color-name letter
- **THEN** the symbol and color value are written into the appearance fields

- **Anchors**: `src/init1.c:3928-3950`; the zero terrain (nothing) sample sits at
 the head of `lib/edit/f_info.txt`

### Requirement: Lingering Damage

Terrain SHALL support up to four `E:` lines declaring lingering damage:
`NdM:frequency:type` or `NdM:frequency:type-name`; the frequency is written as a
percentage and stored converted to per-mille (x10) at load time; the type name is
either a numeric index or a name from the engine's damage-type name table, and the
line is rejected when neither is recognized.

#### Scenario: Frequency conversion

- **WHEN** the frequency column of an `E:` line is 25
- **THEN** that damage group's trigger frequency is stored as 250

#### Scenario: Type name lookup

- **WHEN** the third column of an `E:` line gives a damage type name
- **THEN** it is translated through the name table to the type index; a name
 outside the table fails parsing

- **Anchors**: `src/init1.c:3952-4005` (including `freq *= 10` and the name-table
 fallback); name table `d_info_dtypes`

### Requirement: Terrain Flags

Terrain SHALL support multiple `F:` flag lines, names separated by spaces or
vertical bars; a name MUST resolve in the terrain flag name table, and an unknown
name is rejected.

#### Scenario: Unknown flag

- **WHEN** an `F:` line contains a flag name missing from the name table
- **THEN** parsing fails with an unknown-flag error

- **Anchors**: `src/init1.c:4007-4032`, name table from `src/init1.c:790`, loading
 function from `src/init1.c:3676`
