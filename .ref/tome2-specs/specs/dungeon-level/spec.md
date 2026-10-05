# dungeon-level Specification

## Purpose

The level command file mechanism: every dungeon can carry one command file per
level (`lib/dngn/dun<dungeon index>.<depth - mindepth>`), declaring that level's
branch destination, father branch, father depth, savefile extension, generator,
special level, name, dungeon flags, and description through single-letter command
lines. This capability is the sequential reader of that file and the value
semantics of each command. When the files are loaded and how special levels are
actually laid out are specified in `specs/dungeon-generation/spec.md`; the dungeon
flag name table used by the `F` command is documented under `src/init1.c`
(see `specs/dungeon/spec.md`).

## Requirements

### Requirement: Sequential Command Reading

Command files SHALL be read line by line in order: blank lines and `#` comments
are skipped. The reader maintains a line cursor: a query matches only lines after
the cursor and, on a hit, leaves the cursor on that line — so a loop over the
reader fetches successive lines of the same command one after another. A query
fails when the file does not exist or no matching line remains.

#### Scenario: Cursor advance

- **WHEN** the same command in the same file is matched twice in a row and the
 file carries two lines of that command
- **THEN** the first match takes the first line's parameter and the second match
 takes the second line's parameter

- **Anchors**: `src/levels.c:19-76`

### Requirement: File Naming

A level command file SHALL be named from the current dungeon index and the
relative depth: `dun<dungeon type>.<depth - dungeon mindepth>`.

#### Scenario: Path assembly

- **WHEN** any level command is queried
- **THEN** the reader opens the command file matching the current dungeon and
 depth

- **Anchors**: `src/levels.c:82-94` (naming sample); the same form in every query
 function

### Requirement: Command Semantics

Command files SHALL support the following commands: `B` branch destination (next
dungeon index, default 0), `A` father branch (default 0), `L` father depth
(default 0), `S` savefile extension, `G` level generator name, `U` special level
identifier, `N` special level name, `D` level description. A missing numeric
command returns 0; a missing text command returns failure. Every public query
function resets the reader cursor to the top of the file before its query.

#### Scenario: Default value

- **WHEN** the command file exists but carries no `B` line
- **THEN** the branch query returns 0

- **Anchors**: `src/levels.c:82-188`

### Requirement: Level Flags

The `F` command SHALL be repeatable: the flag reader resets the cursor once, then
queries one line per iteration until the file runs out; names are separated by
spaces or vertical bars and are written into the global dungeon flag words through
the dungeon flag name table; an unknown name aborts loading.

#### Scenario: Line-by-line accumulation

- **WHEN** the command file carries three `F` lines
- **THEN** the three flag sets are parsed one line at a time and accumulated into
 the global dungeon flag words

- **Anchors**: `src/levels.c:193-225`
