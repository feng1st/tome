# vault Specification

## Purpose

The word-list contract for vault templates: how `lib/edit/v_info.txt` declares a
room template that can be embedded in a dungeon — type, rating, size, glyph layout
lines, and the quest-vault monster and item configuration — and how the engine
parses and loads it at startup. How layout glyphs translate into terrain and
entities when a vault is laid out is specified in the vault parts of
`specs/dungeon-generation/spec.md` (the glyph tables are carried by the map format
and special.txt — see `specs/content-maps/spec.md`). Layout content is data and is
not enumerated here.

The shared file skeleton is specified in `specs/edit-format/spec.md`. This
word-list's entry indexes are strictly increasing (equal indexes are rejected
too).

## Requirements

### Requirement: Type and Size

A vault SHALL declare four columns with an `X:` line: type, rating, height, and
width.

#### Scenario: Four columns loaded

- **WHEN** an `X:` line carries four columns of the form `7:5:12:20`
- **THEN** type 7, rating 5, height 12, and width 20 are stored on the template

- **Anchors**: `lib/edit/v_info.txt:28` (sample entry); `src/init1.c:3596-3613`

### Requirement: Layout Lines

A vault SHALL support multiple `D:` lines carrying the layout: the line texts are
appended directly into one glyph canvas, whitespace inside a line is significant
(the header notes that spacing matters); the canvas is interpreted at the height
and width declared by the `X:` line.

#### Scenario: Canvas assembly

- **WHEN** an entry carries multiple `D:` layout lines
- **THEN** the lines concatenate in order into the canvas, and layout resolves each
 character through the lookup tables into terrain and entities

- **Anchors**: `lib/edit/v_info.txt:12` (header note), `:29-40` (sample layout);
 `src/init1.c:3573-3593`

### Requirement: Quest Vault Configuration

A vault SHALL support a `Y:` line declaring fifteen columns of quest
configuration: ten monster indexes, three item indexes, a level, and a dungeon
type.

#### Scenario: Fifteen columns loaded

- **WHEN** a `Y:` line carries fifteen numeric columns
- **THEN** the monster and item rosters, the level, and the dungeon type are each
 stored

- **Anchors**: `src/init1.c:3618-3648`

### Requirement: Ignored Lines

`Q:` lines and `T:` lines of this word-list SHALL be skipped whole (comments and
blank lines are skipped likewise) and take no part in parsing.

#### Scenario: Skip

- **WHEN** parsing encounters a `Q:` or `T:` line
- **THEN** the line is ignored and parsing continues

- **Anchors**: `src/init1.c:3477-3478`

### Requirement: Incremental Loading

The vault parser SHALL support incremental calls: the start marker distinguishes
the first load (resetting the index and text baselines, and skipping the tail
padding) from later appends (no reset; the name and text size bases are each
padded by one at completion), for use by run-time reloads.

- **Dead code:** the shipped code only ever calls the first-load form
 (`init2.c` boot passes start true), so the incremental path is never
 exercised.

#### Scenario: First and incremental

- **WHEN** the parser is called first with the start marker and then again
 incrementally
- **THEN** the first call resets the baselines and the incremental call keeps the
 existing baselines and keeps registering

- **Anchors**: `src/init1.c:3445-3467` (the `start` parameter),
 `src/init1.c:3656-3661` (the tail padding condition)
