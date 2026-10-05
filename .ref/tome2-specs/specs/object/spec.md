# object Specification

## Purpose

The word-list contract for basic item kinds: how `lib/edit/k_info.txt` declares one
kind of basic item — tval/sval/parameters, appearance, depth, weight and cost, combat
modifiers, the allocation table (at which depth, with which chance, the kind appears),
flags, granted powers and activation — and how the engine parses and loads it at
startup. Generation picks, identification, and wield/use effects are recorded with the
`src/object1.c` and `src/object2.c` analyses. The complete flag-name lists are the
source tables at the anchored spots; individual entries are content data and are not
analyzed one by one.

The common file skeleton is described in `specs/edit-format/spec.md`. Entry indices in
this word-list must increase strictly (an index equal to the previous one is also
rejected).

## Requirements

### Requirement: Entry Defaults

An item entry SHALL begin with injected defaults: the ESP field cleared to 0 and the
granted-power field set to -1.

#### Scenario: Defaults

- **WHEN** an item entry starts parsing
- **THEN** `esp` is 0 and `power` is -1, i.e. the ungranted state

- **Anchors**: `src/init1.c:4283-4285`

### Requirement: Category Parameter Line

An item SHALL declare tval, sval and parameter values with an `I:` line, in one of
three forms: four columns (with the second pval), three columns (second pval left at
0), or `tval:sval:pval:SPELL=<spell name>` (the second pval becomes the spell index).

- The SPELL form resolves the name through the Lua `find_spell` bridge
 (`src/cmd5.c:2562-2580`, defined in `lib/core/s_aux.lua:87-95`) and stores whatever
 index comes back into `pval2`.
- The parser never rejects an unresolvable spell name: the SPELL form matches any name
 text, and an unknown name simply stores the -1 that `find_spell` returns.

#### Scenario: Spell form

- **WHEN** an `I:` line gives a spell name after `SPELL=`
- **THEN** the second pval becomes that spell's index; if the name matches no loaded
 spell, `-1` is stored instead and the entry is still accepted

- **Anchors**: `src/init1.c:4354-4388`, `src/cmd5.c:2562-2580` (find_spell bridge)

### Requirement: Appearance Parameters

An item SHALL declare depth, extra, weight and cost with a `W:` line.

#### Scenario: Four columns loaded

- **WHEN** a `W:` line carries four values
- **THEN** depth, extra, weight and cost are stored one by one

- **Anchors**: `src/init1.c:4390-4408`

### Requirement: Allocation Table

An item SHALL support an `A:` line declaring one allocation table made of
`:depth` or `:depth/chance` segments; each segment registers an appearance depth and a
chance, the chance defaults to 1, and only a positive chance overrides the default.

#### Scenario: Default chance

- **WHEN** one segment of an `A:` line is just `:5` with no slash chance
- **THEN** the segment is registered with chance 1

- **Anchors**: `lib/edit/k_info.txt:40` (`A:5/1` sample); `src/init1.c:4469-4499`

### Requirement: Associated Target

An item SHALL support a `T:` line declaring a pair of target btval/bsval (the parser
comment calls this "arTifact Info"); its runtime use is recorded with the item code
analyses.

#### Scenario: Two columns loaded

- **WHEN** a `T:` line carries two values
- **THEN** btval and bsval are stored one by one

- **Anchors**: `src/init1.c:4410-4425`

### Requirement: Combat Modifiers

An item SHALL support a `P:` line declaring combat values: armor class, damage dice
(NdM form), to-hit modifier, to-dam modifier, to-ac modifier.

#### Scenario: Six columns loaded

- **WHEN** a `P:` line carries the six value groups
- **THEN** armor class, damage dice and the three modifiers are stored one by one

- **Anchors**: `src/init1.c:4501-4519`

### Requirement: Hidden And Visible Flags

An item SHALL support two kinds of flag lines: `F:` writes hidden flags (not knowable
by the player before identification) and `f:` writes visible flags (registered into
the obvious bits, shown without identification); flag names are separated by spaces or
vertical bars, and a name MUST resolve in the object flag name tables (which include
the trap flag table and the sense/ESP flag table) — an unknown name rejects the entry.

#### Scenario: Visible-flag split

- **WHEN** the same flag name appears once in an `F:` line and once in an `f:` line
- **THEN** the hidden flag word and the visible flag word are each set in their own
 word; neither overwrites the other

- **Anchors**: `src/init1.c:4521-4573` (F:/f: loading), `src/init1.c:4058-4158`
 (lookup: five flag groups, trap table, sense table)

### Requirement: Description Line Concatenation

An item SHALL support `D:` lines carrying description text; multiple description lines
are concatenated into the full description, and a missing space at the end of the
previous line is compensated by inserting one space before joining.

#### Scenario: Automatic space

- **WHEN** the previous description line does not end with a space and the next `D:`
 line begins
- **THEN** one space is inserted between the two lines before concatenating

- **Anchors**: `src/init1.c:4294-4324`

### Requirement: Appearance Symbol And Color

An item SHALL declare its map display symbol and color with a `G:` line: the symbol is
a single character, the color name is translated to the engine color value, and an
invalid color name rejects the entry.

#### Scenario: Color translation

- **WHEN** a `G:` line gives a symbol and a color-name letter
- **THEN** symbol and color value are stored into the appearance fields

- **Anchors**: `src/init1.c:4326-4352`

### Requirement: Granted Power And Activation

An item SHALL support a `Z:` line granting one existing power (the power name is
looked up case-insensitively in the power table; an unknown name rejects) and SHALL
support declaring an activation with `a:HARDCORE=<name>` or `a:SPELL=<name>` (a spell
activation is registered as a negative value; an unknown name rejects).

#### Scenario: Built-in activation name

- **WHEN** the name given to `a:HARDCORE=` is not in the built-in activation table
- **THEN** parsing fails

- **Anchors**: `src/init1.c:4427-4467`
