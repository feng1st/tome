# alchemy Specification

## Purpose

The alchemy word-list contract: `lib/edit/al_info.txt` loads two data sets at once —
the alchemy recipes (which object needs how much of which essence) and the artifact
flag selection table (the flags, activations and corpse flags selectable when forging
or enchanting artifacts, with their group, level and experience thresholds) — plus
the parse/load at engine startup. Recipe presentation in the alchemy interface and the
essence extraction rules are recorded with the relevant engine file analyses. The
complete essence-name and flag-name lists are the source tables at the anchored
spots; individual entries are content data and are not analyzed one by one.

The common file skeleton is described in `specs/edit-format/spec.md`. This word-list
has no `N:` entry lines and line order does not matter: recipe lines register in
order of appearance, and flag entries are opened by an `A:` line.

## Requirements

### Requirement: Object Recipe Lines

An `I:` line SHALL declare one object recipe: `tval:sval:qty:essence-name`; the
essence name is resolved through the essence name table and an unknown name rejects;
content after the first space in the line is ignored; the recipe capacity is bounded
by the alchemy recipes key in misc.txt, and overflow rejects.

#### Scenario: Essence name resolution

- **WHEN** an `I:` line carries the text `80:0:1:LIFE`
- **THEN** one recipe "food kind 0, quantity 1, LIFE essence" registers; an essence
 name missing from the table fails the parse

- **Anchors**: `lib/edit/al_info.txt:32` (format header note),
 `lib/edit/al_info.txt:74-75` (samples); `src/init1.c:4748-4778`, the essence
 table at `src/init1.c:4663`

### Requirement: Flag Recipe Lines

An `a:` line SHALL declare one recipe keyed by an object flag: `qty:flag essence` —
the flag name and the essence name are two space-separated fields after the last
colon; at registration the tval is fixed to 0 and the sval is the flag index
(resolved through the object flag table); the essence and capacity rules match the
object recipe lines.

- The parser does not validate the flag name: an unknown flag name is not rejected,
 it simply registers sval = -1 (only the essence name is checked).
- The data-file header itself marks this form "(not used)", and the shipped
 `al_info.txt` contains no `a:` lines.

#### Scenario: Flag-keyed

- **WHEN** an `a:` line carries the text `2:STR LIFE`
- **THEN** one recipe "STR flag, quantity 2, LIFE essence" registers with tval 0

- **Anchors**: `lib/edit/al_info.txt:198` (header note that a: lines are valid
 anywhere); `src/init1.c:4779-4802`

### Requirement: Artifact Flag Entries

An `A:` line SHALL open one artifact flag selection entry, carrying seven columns:
group, reference tval, reference sval, reference pval, pval, level threshold,
experience threshold. Before a new entry opens, the previous entry SHALL be validated
as complete: group and description non-empty, and the item description and the
reference tval must both exist or both be absent — a missing part rejects.

#### Scenario: Previous entry completeness

- **WHEN** a new `A:` line appears while the previous entry lacks the description,
 or its item description and reference tval disagree
- **THEN** parsing fails

- **Anchors**: `src/init1.c:4803-4828`

### Requirement: Entry Flags And Activations

An artifact flag entry SHALL declare its granted object flag with an `F:` line
(resolved through the object flag table, an unknown name rejects), or its granted
activation with an `x:` line (the activation name is registered negated, an unknown
name rejects); with an `x:` line the group is forced to 88 and the pval cleared.

#### Scenario: Activation entry

- **WHEN** an entry carries an `x:` line
- **THEN** the entry's flag slot registers the negative of the activation index,
 with group 88 and pval 0

- **Anchors**: `lib/edit/al_info.txt:755` (lowercase x: header note);
 `src/init1.c:4834-4851`

### Requirement: Corpse Race Flags

An artifact flag entry SHALL support at most one `f:` line declaring up to six
monster flags (resolved through the monster flag table, an unknown name rejects);
the line is only valid for entries whose reference tval is a corpse — a nonzero
reference pval, an existing `f:` line, or a non-corpse reference rejects.

#### Scenario: Scope restriction

- **WHEN** an `f:` line appears on an entry whose reference tval is not a corpse
- **THEN** parsing fails with the restriction error

- **Anchors**: `src/init1.c:4853-4905`, the monster flag lookup
 `src/init1.c:4625-4660`

### Requirement: Entry Text Lines

An artifact flag entry SHALL support three text lines: `D:` the group description,
`d:` the item description (at most one per entry, a duplicate rejects), and `p:` the
plural item description (valid only for entries with a nonzero pval, otherwise
rejected).

- **Discrepancy:** the data-file header comment says `p:` is "Illegal if pval != 1",
 but the code rejects only a zero pval.

#### Scenario: Duplicate item description

- **WHEN** a second `d:` line appears on the same entry
- **THEN** parsing fails

#### Scenario: Plural description precondition

- **WHEN** a `p:` line appears on an entry whose pval is zero
- **THEN** parsing fails

- **Anchors**: `src/init1.c:4907-4980`

### Requirement: Unknown Line Rejection

Any line in the word-list other than comments, blank lines, the common skeleton lines
and the line types above MUST be rejected while no `A:` entry is open yet.

#### Scenario: No leading entry

- **WHEN** a text line appears before any `A:` line
- **THEN** parsing fails with the no-current-entry error

- **Anchors**: `src/init1.c:4830-4832`, `src/init1.c:4982-4983`
