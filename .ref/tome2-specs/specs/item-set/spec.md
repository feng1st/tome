# item-set Specification

## Purpose

The word-list contract for item sets: how `lib/edit/set_info.txt` declares one set —
name, description, the member artifacts with their per-piece threshold parameters, and
the flag groups that take effect when a threshold is reached — and how the engine
parses and loads it at startup. The set activation checks (how many pieces collected,
which flags take effect) are recorded with the `src/object2.c`/`src/cmd1.c` analyses.
The flag-name lists are the source tables at the anchored spots; individual entries
are content data and are not analyzed one by one.

The common file skeleton is described in `specs/edit-format/spec.md`.

## Requirements

### Requirement: Entry Defaults

A set entry SHALL begin with injected defaults: the collected count and the used count
are cleared; the six member slots are all cleared (artifact index 0, present flag
false, and the flag words and parameter values of all six threshold segments cleared).

#### Scenario: Defaults

- **WHEN** a set entry starts parsing
- **THEN** it has no members and no flags until `P:`/`F:` lines fill them in

- **Anchors**: `src/init1.c:5580-5597`

### Requirement: Member Thresholds

A `P:` line SHALL declare one member threshold: `artifact-index:pieces-needed:pval`.
An artifact index already present reuses its existing slot, otherwise a new slot is
opened; the pval is written into the threshold segment indexed by the piece count
(counting from 1); the flags of an `F:` line belong to the (member, threshold segment)
of the most recent `P:` line.

#### Scenario: Multiple threshold segments

- **WHEN** the same artifact is given `P:66:2:2` and later `P:66:3:1`
- **THEN** both lines reuse one member slot, filling the 2-piece and 3-piece
 threshold segments; a following `F:` line lands in the 3-piece segment

- **Anchors**: `lib/edit/set_info.txt:44-51` (Narthanc two-threshold sample);
 `src/init1.c:5628-5656`

### Requirement: Threshold Flags

An `F:` line SHALL write flags into the current threshold segment of the most recent
member: names are separated by spaces or vertical bars, resolved through the object
flag table (the five flag groups plus the sense table), and an unknown name rejects
the entry.

#### Scenario: Flag lookup

- **WHEN** an `F:` line contains a name missing from the flag tables
- **THEN** parsing fails with an unknown-flag error

- **Anchors**: `src/init1.c:5658-5689`, lookup from `src/init1.c:1704`

### Requirement: Description Concatenation

A set SHALL support `D:` lines carrying description: the first line opens the
description and later lines are appended directly (no separator).

#### Scenario: Multi-line description

- **WHEN** one set carries several `D:` lines
- **THEN** the line texts are concatenated in order without any separator

- **Anchors**: `src/init1.c:5606-5626`
