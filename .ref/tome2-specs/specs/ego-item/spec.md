# ego-item Specification

## Purpose

The word-list contract for ego items: how `lib/edit/e_info.txt` declares one title
that can be attached to base items — the range of applicable base items, appearance
depth with the double rarity, cost, modifier caps, name position, the flag groups
grouped by rarity, base-item flag constraints, granted power and activation — and how
the engine parses and loads it at startup. Title picking and flag assignment during
item generation are recorded with the `src/object2.c` analysis. The complete flag-name
lists are the source tables at the anchored spots; individual entries are content data
and are not analyzed one by one.

The common file skeleton is described in `specs/edit-format/spec.md`. The `D:` line
that the file header still advertises is retired: description lines are not parsed,
and one appearing is rejected as an unknown line.

## Requirements

### Requirement: Entry Defaults

A title entry SHALL begin with injected defaults: the granted-power field set to -1;
all ten applicable-tval slots set to 255 (no applicable tval); the rarity values and
flag words of the five rarity groups all cleared.

#### Scenario: Defaults

- **WHEN** a title entry starts parsing
- **THEN** it has no applicable tvals and no flag groups until `T:`, `R:` and `F:`
 lines fill them in

- **Anchors**: `src/init1.c:6801-6819`

### Requirement: Applicable Base Item Range

A title SHALL support up to ten `T:` lines declaring the applicable base items: a
tval with a sval range (min:max); an eleventh line rejects the entry.

#### Scenario: Sval range

- **WHEN** a `T:` line carries the three-column `6:0:255` form
- **THEN** base items of that tval with svals 0 through 255 can all carry the title;
 an eleventh `T:` line rejects

- **Anchors**: `lib/edit/e_info.txt:39-40` (the header claims at most five lines;
 the code limit is ten); `src/init1.c:6855-6875`

### Requirement: Rarity Groups And Group Flags

A title SHALL support up to five `R:` lines opening flag groups, each carrying that
group's appearance percentage; `F:` lines (hidden flags) and `f:` lines (visible
flags) up to the next `R:` write into the current group, with F: and f: each sharing
one set of object flag name tables — an unknown name rejects; an `F:`/`f:` line
before the first `R:` rejects.

#### Scenario: Group membership

- **WHEN** an entry carries `R:100`, `F:MANA`, `R:70`, `F:PVAL_M2` in this order
- **THEN** MANA belongs to the first group and PVAL_M2 to the second, with
 appearance percentages 100 and 70

#### Scenario: Missing group

- **WHEN** an `F:` line appears before any `R:` line
- **THEN** parsing fails with the no-current-group error

- **Anchors**: `lib/edit/e_info.txt:42-44` (header example),
 `lib/edit/e_info.txt:72-81` (sample entry); `src/init1.c:6877-6895` (R:),
 `src/init1.c:7049-7105` (F:/f: group membership), the flag grab helper
 `src/init1.c:6465-6576`

### Requirement: Base Item Flag Constraints

A title SHALL support multiple `r:N:` and `r:F:` lines declaring the flags the base
item must have and must not have: the lists are separated by spaces or vertical bars,
resolved through the object flag table, and an unknown name rejects.

#### Scenario: Needed flags

- **WHEN** an `r:N:` line lists a flag name
- **THEN** the flag is registered as required — only base items carrying it can
 take this title

- **Anchors**: `lib/edit/e_info.txt:29-30`; `src/init1.c:6995-7047`, the constraint
 loader from `src/init1.c:6577`

### Requirement: Modifier Caps

A title SHALL declare its four modifier caps with a `C:` line: to-hit, to-dam,
armor, pval.

#### Scenario: Four columns loaded

- **WHEN** a `C:` line carries four values
- **THEN** the four caps are stored one by one

- **Anchors**: `lib/edit/e_info.txt:28`; `src/init1.c:6936-6952`

### Requirement: Appearance Parameters And Name Position

A title SHALL declare depth, rarity, magic rarity and cost with a `W:` line; and the
name position with an `X:` line (the position letter B places the title name before
the base item name, anything else after it; the case-insensitive reading does not
hold — only a capital B works) plus its rating (the level feeling); the equipment
slot column is ignored at load time.

#### Scenario: Name position

- **WHEN** the `X:` position letter is B
- **THEN** the title name renders before the base item name; the slot column is
 discarded, the rating is registered as usual

- **Anchors**: `lib/edit/e_info.txt:46-50` (header), `lib/edit/e_info.txt:73`
 (sample); `src/init1.c:6897-6914` (slot commented out), `src/init1.c:6916-6934`

### Requirement: Granted Power And Activation

A title SHALL support a `Z:` line granting one existing power (the power name is
looked up case-insensitively in the power table; an unknown name rejects) and SHALL
support declaring an activation with `a:HARDCORE=<name>` or `a:SPELL=<name>` (a spell
activation is registered as a negative value; an unknown name rejects).

#### Scenario: Activation declaration

- **WHEN** `a:SPELL=` names an existing spell
- **THEN** the activation field registers the negative of that spell's index

- **Anchors**: `lib/edit/e_info.txt:31`; `src/init1.c:6954-6993`
