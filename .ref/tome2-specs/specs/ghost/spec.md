# ghost Specification

## Purpose

Player ghost mechanics: a dead character leaves a bone file, and a later game raises a
unique ghost monster configured from the dead one's name, race and class at the same
dungeon depth. The whole mechanism is disabled in tome2.3.5: all three implementation
stages — bone-file writing, ghost configuration, placement — sit inside source-level
`#if 0`, and the placement entry always returns failure, so the shipped game has no
player ghosts. The dedicated race slot the code targets is the last monster entry
`r_info[max_r_idx - 1]`, not the zeroth `N:0:Player` word-list entry (that entry exists
but the ghost code never references it). For when `make_bones` is invoked on death, see
specs/death-score/spec.md.

## Requirements

### Requirement: Mechanism Disabled

The ghost mechanism SHALL be inert: death writes no bone file (the writing body is
excluded by conditional compilation), and the dungeon-generation placement entry always
returns failure — no bone file is read, no ghost monster is configured, no monster is
placed.

#### Scenario: Placement always fails

- **WHEN** dungeon generation attempts to place a player ghost
- **THEN** the entry returns failure immediately and no ghost appears

- **Anchors**: `src/ghost.c:26-105` (bone writing disabled), `src/ghost.c:107-1015` (configuration disabled), `src/ghost.c:1021-1140` (placement always fails at `:1137-1139`)

### Requirement: Retained Shape Of The Disabled Implementation

The disabled implementation SHALL survive complete inside the source:

- the bone file is four text lines — name, max HP, race, class;
- the file is named `bone<dungeon>.<depth>` and never overwrites an existing file; the
 writer retries five depths around the death depth (death depth first, then
 `depth + 5 - damroll(2,4)`, minimum `randint(4)`) until it finds a free name;
- ghost configuration truncates the name at the first comma (within 16 characters),
 substitutes `Nobody` when under two characters remain, and capitalizes the initial
 letter; the dead one's max HP is doubled and folded into hit dice (dice = sides =
 smallest `i` with `i*i >= hp`); the race gets the unique/evil/undead/no-sleep/no-conf
 flags, awareness 100 (`aaf`), and never sleeps;
- configuration then fills drops, immunities, spells (branched on the dead one's class)
 and blows in one of two forms — town form or dungeon form — chosen by a 50% roll;
- placement is depth-gated (never in town, at most one ghost, pass a
 `randint(dun_level / 2 + 11)` gate of 14 and a further 45% magik gate), re-reads the
 50% form choice, and places the ghost on an empty grid farther than `MAX_SIGHT + 5`
 from the player.

#### Scenario: Bone file naming

- **WHEN** the (disabled) death-dump logic is evaluated
- **THEN** the target file is `bone<dungeon index>.<depth>` and an existing file of the
 same name makes the writer retry at another depth

- **Anchors**: `src/ghost.c:46-76` (naming and no-overwrite), `src/ghost.c:920-1014` (configuration body), `src/ghost.c:1045-1136` (placement conditions and site choice)
