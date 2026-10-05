# artifact Specification

## Purpose

The word-list contract for artifacts: how `lib/edit/a_info.txt` declares one artifact
— the item kind it hangs off, the base values, combat modifiers, description, flags
and activation ability — and how the engine parses and validates it at startup. The
file header fixes the artifact index range groups (1-15 special, 16-63 armor, 64-127
weapons; the indices are defined in `src/defines.h` and must not change); the
generation-time meaning of the ranges is recorded with the `src/object2.c` analysis.
The complete flag-name lists are the source tables at the anchored spots; individual
entries are content data and are not analyzed one by one.

The common file skeleton (version line, include directives, the `N:` index rules,
lexing, unknown-line rejection) is described in `specs/edit-format/spec.md`.

## Requirements

### Requirement: Attached Item Kind

An artifact SHALL declare the item kind it attaches to with an `I:` line: tval, sval
and pval; that tval:sval combination MUST exist in the item word-list — a miss
rejects the whole word-list.

#### Scenario: Kind not found

- **WHEN** the tval:sval combination of an `I:` line is not in the item word-list
- **THEN** parsing fails with the lookup error

- **Anchors**: `lib/edit/a_info.txt` (first entry sample after the header `V:` line);
 `src/init1.c:5279-5301`

### Requirement: Base Four Columns

An artifact SHALL declare level, rarity, weight and cost with a `W:` line.

#### Scenario: Four columns loaded

- **WHEN** a `W:` line carries four values
- **THEN** level, rarity, weight and cost are stored into the artifact entry one by
 one

- **Anchors**: `src/init1.c:5303-5321`

### Requirement: Combat Modifiers

An artifact SHALL declare combat values with a `P:` line: armor class, damage dice
(NdM form), to-hit modifier, to-dam modifier, to-ac modifier.

#### Scenario: Six columns loaded

- **WHEN** a `P:` line carries the five value groups in the `0:1d1:0:0:0` form
- **THEN** armor class 0, damage dice 1d1, and the to-hit/to-dam/to-ac modifiers are
 stored one by one

- **Anchors**: `src/init1.c:5323-5341`

### Requirement: Entry Defaults

An artifact entry SHALL begin with injected defaults: the four element-ignore flags
(acid/lightning/fire/cold) all set (artifacts are not destroyed by the elements); the
sense ESP field cleared; the granted-power field set to -1 (ungranted).

#### Scenario: Ignore defaults

- **WHEN** an artifact entry starts parsing
- **THEN** the four IGNORE flags for acid/lightning/fire/cold are already set, so
 element-destruction flags need not be declared per artifact

- **Anchors**: `src/init1.c:5223-5231`

### Requirement: Description Line Concatenation

An artifact SHALL support `D:` lines carrying description text; multiple description
lines are concatenated into the full description, and a missing space at the end of
the previous line is compensated by inserting one space before joining.

#### Scenario: Automatic space

- **WHEN** the previous description line does not end with a space and the next `D:`
 line begins
- **THEN** one space is inserted between the two lines before concatenating

- **Anchors**: `src/init1.c:5247-5277`

### Requirement: Activation Type

An artifact carrying the ACTIVATE flag MUST declare its activation type through an
`a:` line: `a:HARDCORE=<name>` points into the engine's built-in activation table
(an unknown name rejects); `a:SPELL=<name>` points at an existing spell and is
registered as a negative value (an unknown name rejects). A missing declaration or an
unknown name MUST get the word-list rejected.

#### Scenario: Built-in activation name

- **WHEN** the name given to `a:HARDCORE=` is not in the built-in activation table
- **THEN** parsing fails

#### Scenario: Spell activation is negative

- **WHEN** `a:SPELL=` names an existing spell
- **THEN** the activation field registers the negative of that spell's index

- **Anchors**: `lib/edit/a_info.txt` (an `F:ACTIVATE` sample entry);
 `src/init1.c:5233-5238` (missing check), `src/init1.c:5419-5437` (the two forms)

### Requirement: Granted Power

An artifact SHALL support a `Z:` line granting one existing power: the power name is
matched case-insensitively against the engine power table, and a miss rejects.

#### Scenario: Unknown power

- **WHEN** the power name of a `Z:` line is not in the power table
- **THEN** parsing fails with the lookup error

- **Anchors**: `src/init1.c:5343-5363`

### Requirement: Hidden And Visible Flags

An artifact SHALL support two kinds of flag lines: `F:` writes hidden flags (not
knowable by the player before identification) and `f:` writes visible flags
(registered into the obvious bits, shown without identification); flag names are
separated by spaces or vertical bars, and a name MUST resolve in the object flag name
tables — an unknown name rejects the entry. Both line kinds share one name table.

#### Scenario: Visible-flag split

- **WHEN** the same flag name appears once in an `F:` line and once in an `f:` line
- **THEN** the `F:` slot writes the hidden flag word and the `f:` slot the visible
 flag word; neither overwrites the other

#### Scenario: Unknown flag

- **WHEN** an `F:` or `f:` line contains a flag name missing from the name tables
- **THEN** parsing fails with an unknown-flag error

- **Anchors**: `src/init1.c:5365-5390` (F:), `src/init1.c:5392-5417` (f:),
 `src/init1.c:4999-5017` (the split and the shared name table)
