# edit-format Specification

## Purpose

The common parsing contract of the `lib/edit/` data word-list files: the file
skeleton (version line, include directive), the entry skeleton (index and name
rules of the `N:` line), the lexing (comments, blank lines, colon format), and
error handling. The per-domain word-lists (monster, artifact, terrain, and so on)
add their own line types on top of this contract.

Each parser is an independent implementation (one `init_*_info_txt` per word-list
in `init1.c`), but the common behavior below is uniform across the checked
parsers; when a new word-list domain appears, recheck it against that parser's
anchors.

## Requirements

### Requirement: Version Line Before Entries

Every word-list file SHALL declare a three-part format version with a `V:` line;
the version line MUST precede any entry line, and a parser that meets an entry
line before the version line rejects the file. With the version-verification
compile-time switch enabled, the declared version MUST match the built-in version
in that word-list's header.

#### Scenario: Rejected without a leading version

- **WHEN** the first effective line of a word-list file is an entry line rather
 than a `V:` line
- **THEN** parsing fails with a version error

- **Anchors**: `src/init1.c:7709-7739` (r_info), `src/init1.c:5141-5172` (a_info)

### Requirement: Include Directive

A word-list SHALL support `<` lines pulling in other data files, with the pulled
file's contents expanded in place line by line.

#### Scenario: Inline expansion

- **WHEN** parsing encounters a line starting with `<`
- **THEN** the parser pushes the named file onto the line-source stack and
 subsequent lines are read from that file

- **Anchors**: `src/init1.c:7742-7746` (r_info), `src/init1.c:5174-5179` (a_info)

### Requirement: Lexing

A word-list SHALL skip blank lines and `#` comment lines; every other line MUST
have the two-character `X:...` colon format, and a line whose second character is
not a colon is rejected.

#### Scenario: Colon format

- **WHEN** the second character of an effective line is not a colon
- **THEN** parsing fails with a format error

- **Anchors**: `src/init1.c:7702-7705` (r_info), `src/init1.c:5134-5138` (a_info)

### Requirement: Entry Start

An entry SHALL begin with an `N:` line carrying an index and a name: the name
MUST be non-empty; the index MUST be below that word-list's capacity limit (out
of range is rejected). Order validation differs per word-list:
monster/artifact/dungeon/building-action reject a backward index but allow
re-declaring the same index; terrain requires strict increase (equal indexes are
rejected too); ability checks no ordering.

#### Scenario: Backward index rejected (most word-lists)

- **WHEN** a new entry's index in the monster/artifact/dungeon/building-action
 word-lists is below the completed entry's index
- **THEN** parsing fails with an index error

#### Scenario: No order check (ability)

- **WHEN** entries in the ability word-list are declared in any order
- **THEN** parsing is unaffected by the index ordering

- **Anchors**: `src/init1.c:7764-7770` (monster), `src/init1.c:5197-5203`
 (artifact), `src/init1.c:9366-9372` (dungeon), `src/init1.c:10330-10336`
 (building-action), `src/init1.c:3812` (terrain strict increase),
 `src/init1.c:6198-6204` (ability, no order check)

### Requirement: Entry Default Injection

The `N:` line handling of each word-list SHALL inject that domain's defaults for a
new entry (drop tendencies, flag bits, derived field initial values, and the
like); the specific default items are declared by each domain's spec.

#### Scenario: Defaults in effect

- **WHEN** an entry starts parsing
- **THEN** the declared defaults take effect first and are then overridden by the
 entry's later lines

- **Anchors**: `src/init1.c:7790-7795` (r_info drop and frequency defaults),
 `src/init1.c:5223-5231` (a_info indestructibility flag and other defaults)

### Requirement: Unknown Line Rejection

Any line in a word-list other than comments, blank lines, and the line types
declared by that domain MUST be rejected, with parsing ending in an unknown-line
error.

#### Scenario: Unknown line

- **WHEN** parsing meets a line letter the domain does not define
- **THEN** parsing fails with an unknown-line error

- **Anchors**: `src/init1.c:8069-8070` (r_info), `src/init1.c:5440-5441` (a_info)

### Requirement: Parse Errors Carry Location

Word-list parsing SHALL maintain a global error line number and error index: when
parsing fails, the offending line and entry are known, for startup-time error
reporting.

#### Scenario: Location information

- **WHEN** a line of a word-list fails to parse
- **THEN** the error line number and the most recent entry index are readable by
 the reporting side

- **Anchors**: `src/init1.c:7684-7699` (r_info), `src/init1.c:5121-5132` (a_info)
