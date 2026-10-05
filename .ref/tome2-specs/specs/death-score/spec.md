# death-score Specification

## Purpose

Death and scoring: `src/files.c` carries the savefile-and-exit flow — the save action,
suicide, the panic save, the wrap-up (tombstone / information reveal / bone file / high
score board), the score formula, the high score binary file and its board views, the
retirement coronation, the wiping of saved dungeon levels on death, random text line
extraction, and the signal-death semantics. The savefile format's read/write side is
specified in specs/save-load/spec.md.

## Requirements

### Requirement: Saving And Suicide

`do_cmd_save_game` SHALL clear the panic flag (fixing a long-lived bug), strip the
CAVE_VIEW flags before saving and restore them afterwards, run `save_dungeon` first on a
persistent level, disturb the player on a non-autosave, and guard `died_from` with
(saved) during the save and (alive and well) afterwards. Suicide SHALL: ask a winner to
confirm retirement, and ask anyone else to confirm and then re-confirm by typing an @
sign (noscore characters are exempt); taking effect clears alive, sets death, and
records Quitting as the cause of death. Naming SHALL sanitize the player base name (a
name over fifteen characters quits, a control character quits, letters and digits are
kept while @/space/dot/underscore become underscores and every other character is
dropped, Windows truncates to eight, an empty name becomes PLAYER) and build the
savefile path from it.

#### Scenario: Suicide verification

- **WHEN** a non-winner with scoring enabled issues the suicide command
- **THEN** a quit confirmation is required and, unless noscore, a second confirmation by
 typing the @ sign; only then do alive clear, death set, and the cause become Quitting

- **Anchors**: `src/files.c:4979-5067` (saving), `:4934-4976` (suicide), `:4774-4927` (naming and get_name)

### Requirement: Score Formula

`total_points` SHALL work as follows:

- the base is level to the fourth power, plus one hundred per depth of the deepest
 dungeon level, plus one fifth of maximum experience;
- this is multiplied by a difficulty factor counted in twentieths — base twenty, minus
 one for preserve, minus one for maximize, minus four for autoscum, minus ten for
 stupid monsters, plus ten for small levels (only plus four when always-small levels is
 set), plus two for empty arena levels, plus four for smart learn, plus four for smart
 cheat, floored at two;
- one fifth of the gold is added;
- each quest completed adds two thousand plus one hundred per danger level of the quest;
- the total is divided by the companion-loss factor (deaths times two divided by five,
 with zero recorded as one);
- the real-value sum of every aware flavored object kind is added;
- the kill score (fifty per dead unique, otherwise the per-race kill counts) times fifty
 is added;
- one hundred times the completed bounty count is added;
- a winner adds one million.

#### Scenario: Small-level bonus

- **WHEN** the small-levels option is on and always-small-levels is off
- **THEN** the difficulty factor gains ten; with always-small-levels on it gains only
 four

- **Anchors**: `src/files.c:5074-5189`

### Requirement: High Score Board

A high score record SHALL be a fixed 150-byte record (version / points / gold / turns /
date / name / uid / sex / race / subrace / class / class spec / current and maximum
level and depth / arena, quest and exit flags / a thirty-one-character cause of death).
Insertion SHALL scan sequentially for the first slot holding a smaller score and slide
the tail entries down, with a capacity of MAX_HISCORES. Entering the board SHALL refuse
wizards (noscore low four bits), Borgs (next four bits), cheaters (high eight bits) and
non-winners whose cause of death is Interrupting or Quitting — refusing only shows a
message and displays the board; the placement runs under a write lock; reaching the top
ten displays the top fifteen, otherwise the top five plus a window around the own entry.
The prediction board SHALL compute the placement with the TODAY date and a placeholder
cause of death. Further board views SHALL exist: the per-class building plaques
(show_highclass — nine kinds, kings/arena champions/fighters and so on, including an
eligibility line for the living player) and the per-race legends board (race_legends,
ten per race). Retirement coronation SHALL reset the dungeon depth, restore experience
and level, add ten million gold, and show the crown picture with the Vici text.

#### Scenario: Interrupted run refused

- **WHEN** a character that never became a winner dies with the cause of death
 Interrupting or Quitting
- **THEN** the score is not registered and the board is only displayed

- **Anchors**: `src/files.c:5521-5587` (record and read/write), `:5596-5661` (placement), `:6112-6263` (entering), `:6269-6350` (prediction), `:5881-6101` (class boards and race legends), `:6357-6403` (coronation)

### Requirement: Wrap-Up Flow

`close_game` SHALL, on death — record NOTE_WINNER for a winner, announce the exit over
IRC (retired or cause of death), wipe every saved dungeon-level file with `wipe_saved`,
write the death save, show the tombstone (the dead.txt picture plus title, level,
experience, gold, depth and cause of death, and the date; `tombstone_aux` may replace
it), let `show_info` reveal every pack, equipment and home item while looping the
character dumps, write the death note line, produce the bone file with `make_bones`, and
enter the high score board; while alive — save the game (with the autosave flag cleared,
so it disturbs), record NOTE_SAVE_GAME, announce over IRC, and ask for the prediction
board. The panic path SHALL: quit directly when nothing was generated or the character
is already saved, defer the death mark (a negative current HP does not die on the
spot), set panic_save, and force a save. Signals SHALL: judge a fifth consecutive
interrupt as death by Interrupting (warning from the fourth press on), and route fatal
signals into the panic save (with (panic save) recorded as the cause of death).

#### Scenario: Panic save

- **WHEN** a fatal error strikes a generated, not-yet-saved character with negative HP
- **THEN** death is deferred, panic_save is set, and a forced save is written with
 (panic save) as the recorded cause of death

- **Anchors**: `src/files.c:5220-5337` (tombstone), `:5343-5504` (reveal), `:6409-6444` (level wiping), `:6452-6581` (wrap-up), `:6593-6624` (panic), `:6897-7194` (signals)

### Requirement: Random Text Lines

`get_rnd_line` SHALL read the word-list's first line as the line count and pick a line
by randint (running past the end fails; the buffer lines are internalized in the count —
the source comment says so itself). `get_line` SHALL fetch a line by zero-based number
(skipping the leading comment line). `get_xtra_line` SHALL find the N: section matching
the monster race index, read the entry count, and when the monster is afraid skip the
ordinary section and read the afraid section, returning one random line from it.

#### Scenario: Afraid unique speaks

- **WHEN** a monster in the fear state is assigned a line by `get_xtra_line`
- **THEN** the ordinary section is skipped and the line comes from the afraid section

- **Anchors**: `src/files.c:6630-6702`, `:6713-6752`, `:6767-6894`
