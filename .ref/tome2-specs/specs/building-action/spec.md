# building-action Specification

## Purpose

Word-list contract of the store/building actions: how `lib/edit/ba_info.txt` declares a
building service action — name, prestige-tiered costs, action index and restriction,
menu letters — and how the engine parses and loads it at startup. How actions are laid
out inside a building (which building offers which actions, what an action does) is
specified under specs/building/spec.md. Each entry is content data and is not analyzed
line by line.

The common file skeleton is specified in specs/edit-format/spec.md.

## Requirements

### Requirement: Action Name

A building action SHALL declare its index and name with an `N:` line; the name is
mandatory, and the index follows the common contract (going back is refused, out of
range is refused).

#### Scenario: Name loading

- **WHEN** an `N:` line supplies an index and a name
- **THEN** the name text is registered in the word-list name pool and the action is
 indexed by its number

- **Anchors**: `lib/edit/ba_info.txt:11`; `src/init1.c:10314-10358`

### Requirement: Tiered Costs

A building action SHALL declare three cost tiers with a `C:` line: hated, normal, liked
(mirroring the three standing tiers the runtime holds the player in).

#### Scenario: Three-tier loading

- **WHEN** a `C:` line supplies three values
- **THEN** the three costs are stored one by one as hated/normal/liked

- **Anchors**: `lib/edit/ba_info.txt:12`; `src/init1.c:10363-10379`

### Requirement: Action Parameters

A building action SHALL declare its action index, action restriction and menu letter
with an `I:` line; the menu letter may carry an auxiliary letter (four-column form) or
omit it (three-column form, auxiliary letter left at zero).

#### Scenario: Auxiliary letter omitted

- **WHEN** an `I:` line uses the three-column form
- **THEN** the action index, restriction and main letter are stored and the auxiliary
 letter is the empty character

**Discrepancy:** the word-list header documents restriction one as "Restrict to normal
& liked", but the runtime refuses restriction-one actions to liked players
(`bldg_process_command`; the screen presentation agrees with the header instead — see
specs/building/spec.md).

- **Anchors**: `lib/edit/ba_info.txt:13`, `:15-18` (restriction header note); `src/init1.c:10381-10400`
