# pref-file Specification

## Purpose

Preference file system: `src/files.c` hosts the `.prf` line format parsing — token
splitting, the full line-command set (R/G/K/F/B/S/U/E/A/P/L/C/V/T/X/Y/W/Q/% and the
G: sub-forms), the conditional expression evaluator (?:) with its environment
variables ($SYS and friends), the file loaders (the user directory overriding the
system directory), and the run-time time and load gates. Graphics-related lines are
recorded here at the mechanism level only, not their on-screen rendering.

## Requirements

### Requirement: Line Command Set

Preference lines SHALL dispatch on their first character (the second character must
be a colon): %: recursively loads a file; R/G:M/G:P/G:T/K/F/B/S set the display
attr/char of the monster / ego-monster / race-modifier / trap / object-kind /
terrain-feature / store / special-things tables; U/E set default display values per
tval in bulk (unaware items, and the inventory attribute); A stores a macro action,
P attaches a normal macro, C creates a keymap (restricted to a single key), L adds a
CLI extended command (two or three segments); V sets the four channels of the color
table; T creates the macro trigger set from a template (template plus modifier list
plus trigger names and keycodes, clearing the previous set first); X/Y switch an
option by name; W sets a window flag (the main window refuses changes); Q is
deprecated and passes straight through. An out-of-range index SHALL be recorded as
a parse error. The tokenizer SHALL accept both colon and slash as separators, plus
single-quoted character literals and backslash escapes.

#### Scenario: Main window flag refusal

- **WHEN** a preference line `W:0:5:1` sets a flag on window 0
- **THEN** the line is recorded as a parse error and no window flag changes

- **Anchors**: `src/files.c:129-188` (tokenize), `src/files.c:258-710` (line
 commands)

### Requirement: Conditional Expressions

A `?:` line SHALL evaluate its expression and set the bypass flag for the following
lines: parentheses evaluate recursively; the function set is IOR / AND / NOT / EQU /
LEQ / GEQ / LEQN / GEQN (string and numeric comparisons) and SKILL (current value of
a named skill); an operand prefixed with `$` reads the SYS / KEYBOARD / GRAF / RACE
/ RACEMOD / CLASS / PLAYER environment strings, anything else is a literal; an
evaluation other than "0" lets the subsequent lines through. A parse error is
reported with the error code and the line number of the file, followed by the
offending line.

#### Scenario: Bypass flag from an expression

- **WHEN** a `?:` line evaluates to "0", and later a `?:` line evaluates to "1"
- **THEN** the lines after the first are skipped and the lines after the second
 are processed again

- **Anchors**: `src/files.c:723-985` (expressions), `src/files.c:998-1109`
 (loader and ?:)

### Requirement: Load Order And Time / Load Gates

Loading SHALL try the user directory first and fall back to the system directory.
The time gate (CHECK_TIME) SHALL read the seven-day table of time.txt — 24 hourly
slots per day, each row `DAY:` plus 24 X/dot flags, the default opening only the
weekend all day (weekdays blocked 8:00-16:59) — and report a violation when the current
hour's slot is not X. The load gate (CHECK_LOAD) SHALL read load.txt for the limit
matching the host name and compare it against the fifteen-minute load average
(`avenrun[2]`). Both gates feed the game's closing check that runs every 1000 game
turns.

#### Scenario: Weekday midday closure

- **WHEN** CHECK_TIME is enabled and the game runs at 12:00 on a Monday with the
 default time table
- **THEN** the hour slot is a dot and the check reports a time violation

- **Anchors**: `src/files.c:1010-1033` (directory order), `src/files.c:1122-1168`
 (time table), `src/files.c:1234-1298` (load), `src/dungeon.c:1361-1366`
 (1000-turn check)
