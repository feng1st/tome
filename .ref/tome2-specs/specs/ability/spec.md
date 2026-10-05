# ability Specification

## Purpose

The ability word-list contract: how `lib/edit/ab_info.txt` declares one learnable ability
- description, cost, active activation, the three prerequisite classes (skill levels, the
six stat values, existing abilities) and the exclusion pairs - plus how the engine parses
and loads it at boot. The learning flow and effects are specified in specs/skills/spec.md.
The entries themselves are content data and are not enumerated here.

The common file skeleton is specified in specs/edit-format/spec.md. This word-list does
not check entry order.

## Requirements

### Requirement: Entry Defaults

An ability entry SHALL start with injected defaults: the activation key set to 0 and the
learned marker false; the skill-prerequisite, ability-prerequisite and exclusion lists -
ten slots each - and every stat-prerequisite slot are all set to -1 (undeclared).

#### Scenario: Defaults

- **WHEN** an ability entry begins parsing
- **THEN** the activation key is 0 and every prerequisite slot of every class is in the
 undeclared state

- **Anchors**: `src/init1.c:6221-6233`

### Requirement: Description Line Joining

Abilities SHALL support `D:` lines carrying the description: the first line opens the
description text, later lines append with a newline prefix.

#### Scenario: Multi-line description

- **WHEN** one entry carries several `D:` lines
- **THEN** from the second line on, each line is joined with a newline, keeping the
 line-broken display

- **Anchors**: `src/init1.c:6242-6273`

### Requirement: Active Activation

Abilities SHALL support an `A:` line declaring the active activation: the text before the
colon is the activation key (mkey) number, the text after it is the action description.

#### Scenario: Activation declaration

- **WHEN** an `A:` line provides text of the shape `10:Forge ammo`
- **THEN** the ability's activation key is set to 10 and the action description is
 registered as Forge ammo

- **Anchors**: `lib/edit/ab_info.txt:72`, `src/init1.c:6275-6302`, the mkey dispatch
 site is specified in specs/skills/spec.md

### Requirement: Learning Cost

Abilities SHALL declare the learning cost (skill points) with an `I:` line.

#### Scenario: Cost loading

- **WHEN** an `I:` line provides a single number
- **THEN** that number is registered as the learning cost

- **Anchors**: `src/init1.c:6304-6320`

### Requirement: Skill Prerequisites

Abilities SHALL support repeated `k:` lines declaring skill prerequisites: `level:skill
name`; the skill name resolves through the skill table, an unknown name is rejected; at
most ten prerequisites, further entries are silently dropped.

#### Scenario: Unknown name rejected

- **WHEN** a `k:` line names a skill missing from the skill table
- **THEN** parsing fails

#### Scenario: Slot cap

- **WHEN** the ten skill-prerequisite slots are full and another `k:` line appears
- **THEN** that entry is dropped and parsing continues

- **Anchors**: `lib/edit/ab_info.txt:19`, `:41`, `src/init1.c:6322-6353`

### Requirement: Ability Prerequisites

Abilities SHALL support repeated `a:` lines declaring ability prerequisites: prerequisite
abilities resolve by name, an unknown name is rejected; at most ten, further entries are
silently dropped.

#### Scenario: Unknown name rejected

- **WHEN** an `a:` line names an ability missing from the ability table
- **THEN** parsing fails

- **Anchors**: `lib/edit/ab_info.txt:21`, `:67`, `src/init1.c:6355-6374`

### Requirement: Stat Prerequisites

Abilities SHALL support repeated `S:` lines declaring stat prerequisites: `value:stat
name`; the stat name is compared against the six-stat name table, an unknown name is
rejected; re-declaring the same stat overwrites the previous value.

#### Scenario: Stat name lookup

- **WHEN** an `S:` line provides text of the shape `17:DEX`
- **THEN** the dexterity prerequisite is set to 17; a stat name missing from the six-stat
 name table fails parsing

- **Anchors**: `lib/edit/ab_info.txt:20`, `:42`, `src/init1.c:6376-6403`

### Requirement: Ability Exclusions

The ability word-list SHALL support `E:` lines declaring exclusion pairs: both ability
names on the sides of the colon must resolve (any unknown name is rejected); the
exclusion is registered in both directions - each ability's exclusion list records the
other, at most ten each, further entries silently dropped.

#### Scenario: Two-way registration

- **WHEN** an `E:` line declares abilities A and B exclusive
- **THEN** A's exclusion list records B and B's exclusion list records A

- **Anchors**: `lib/edit/ab_info.txt:23`, `src/init1.c:6405-6441`, the learning-time
 exclusion ruling is specified in specs/skills/spec.md
