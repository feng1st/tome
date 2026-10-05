# skill Specification

## Purpose

The skill word-list contract: how `lib/edit/s_info.txt` declares one skill - description,
growth rate, active activation, random-gain chance, flags - plus the three relations
between skills (the parent/child tree, exclusion, and the hostile/friendly percentage
modifiers), and the engine's boot-time parsing and loading. Skill growth, spending and use
are specified in specs/skills/spec.md. The skill flag name list is whatever the source
table at the anchored lines defines; the entries themselves are content data and are not
enumerated here.

The common file skeleton is specified in specs/edit-format/spec.md. This word-list does
not check entry order; tree-relation lines and relation lines may appear before the entry
they reference (they refer by name).

## Requirements

### Requirement: Entry Defaults

A skill entry SHALL start with injected defaults: the action key set to 0, the developed
marker false, the random-gain chance set to 100, and the relation values against all other
skills cleared.

#### Scenario: Defaults

- **WHEN** a skill entry begins parsing
- **THEN** it carries no relation modifier against any other skill until an `E:`/`O:`/`f:`
 line writes one

- **Anchors**: `src/init1.c:5944-5951`

### Requirement: Skill Tree

`T:` lines SHALL declare the parent/child relation: the names on both sides of the colon
are parent and child; the child skill name MUST resolve (an unknown name is rejected) and
the parent name is not looked up - an unknown parent makes the child a root skill (parent
set to -1); the child's sibling index is registered incrementally by the order in which
the `T:` lines appear.

#### Scenario: Root skill

- **WHEN** the parent name of a `T:` line is not in the skill table
- **THEN** the child is registered as a root skill (parent -1); only an unresolvable child
 name is rejected

- **Anchors**: `lib/edit/s_info.txt:23`, `src/init1.c:5788-5812`

### Requirement: Exclusion Relation

`E:` lines SHALL declare two skills mutually exclusive: both skill names must resolve
(any unknown name is rejected); the exclusion is registered in both directions.

#### Scenario: Two-way registration

- **WHEN** an `E:` line declares skills A and B exclusive
- **THEN** the relation value of A against B and of B against A are both set to the
 exclusion marker

- **Anchors**: `lib/edit/s_info.txt:19`, `src/init1.c:5814-5838`

### Requirement: Hostile And Friendly Relations

Hostile and friendly relations SHALL be declared as `skillA:skillB%percentage`: an `O:`
line registers a negative percentage (hostile) and a lowercase `f:` line a positive
percentage (friendly); an unresolvable skill name or percentage field is rejected.
**Discrepancy:** the file header documents the friendly relation as an `A:` line, but the
parser reads lowercase `f:` - the code prevails.

#### Scenario: Sign split

- **WHEN** the same skill pair is declared by an `O:` line and by an `f:` line
- **THEN** the hostile one registers as a negative percentage and the friendly one as a
 positive percentage

- **Anchors**: `lib/edit/s_info.txt:20-21` (the header documents the friendly
 line as `A:`, the typo), `src/init1.c:5841-5903`

### Requirement: Growth Parameters

Skills SHALL support: an `I:` line declaring the growth rate (single value); a `G:` line
declaring the random-gain chance (single value); `D:` lines carrying a newline-joined
description; an `A:` line declaring the active activation (action key and description
text, same shape as the ability word-list); an `F:` line declaring skill flags (names
separated by spaces or pipes, resolved through the skill flag table, unknown names
rejected).

#### Scenario: Flag lookup

- **WHEN** an `F:` line contains a name missing from the skill flag table
- **THEN** parsing fails with an unknown-flag error

- **Anchors**: `lib/edit/s_info.txt:15-17`, `src/init1.c:5960-6085`, flag lookup
 from `src/init1.c:1635`
