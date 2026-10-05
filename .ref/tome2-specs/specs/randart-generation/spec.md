# randart-generation Specification

## Purpose

Random artifact generation: combining one artifact at random out of the randart part
word-list (`lib/edit/ra_info.txt`) — rolling the power count, part filtering (tval
ranges, value sign, per-part cap, antagonist flags, out-of-depth and rarity checks),
flag and modifier accumulation, naming (player naming or syllable random names), and
the artifact scroll usage flow. The part word-list data and flag names are the
randart-part capability; curse application (`curse_artifact`) and flag valuation
(`flag_cost`) are recorded with the `spells*.c`/`object*.c` analyses.

## Requirements

### Requirement: Part Filtering

Picking parts for an artifact SHALL filter the part word-list in order: the
applicable tval/sval ranges must cover the target object; the part's pval cap must
accommodate the object's current pval; the power value sign must match the
expectation (good parts positive, bad parts negative); the per-part occurrence count
has not reached the part's cap; and the object's existing flags MUST NOT intersect
the part's antagonist flags. Excluded parts do not count as tries.

#### Scenario: Antagonist exclusion

- **WHEN** the object already carries a flag that a candidate part lists as an
 antagonist flag
- **THEN** that part is dropped from the candidate set

- **Anchors**: `src/randart.c:24-73`

### Requirement: Candidate Draw

Drawing parts at random from the candidate set SHALL be constrained by two checks: a
part whose level is above the player level passes leniently by the difference (a
random check against the difference must succeed to make it usable); the rarity
check is "a random number within the magic rarity span is not smaller than the
rarity". A hit consumes one occurrence of the part.

- The draw attempts are bounded at ten times the candidate count; the loop counter
 is reused as the candidate index in the implementation, so the actual attempt
 count floats with the drawn indices — this quirk is as-is behavior.

#### Scenario: Out-of-depth pass

- **WHEN** a part sits 3 levels above the player level
- **THEN** it is still usable with a 1/3 chance

- **Anchors**: `src/randart.c:76-109`

### Requirement: Artifact Assembly

Assembly SHALL first roll the total power count from the global generation table
(each generation row's dice roll plus its fixed bonus, accumulated; a creation that
is not cursed gets a 1/30 doubling — only non-scroll creations can be cursed, at
1/`A_CURSED` — and cursed creations halve), then draw good parts one by
one: merging the five flag groups and the sense bits, randomly adding title flags
(including the blows-limiting marker), rolling to-hit/to-dam/to-a adjustments per the
part caps, and keeping the tightest pval cap across parts to roll the final pval on
completion (with the blows cap in effect the pval is at most 2).

- The finished object SHALL always carry the four element-ignore flags.
- Cursed creations SHALL additionally pass through curse application.
- The object's existing title references are cleared, and the name inscription is
 written.

#### Scenario: Power doubling

- **WHEN** an uncursed creation rolls 1 in 30
- **THEN** the power total doubles before the parts are drawn one by one

- **Anchors**: `src/randart.c:268-341` (assembly main loop), `src/randart.c:343-351`
 (blows cap and ignore flags), `src/randart.c:352-356` (curse application),
 `src/randart.c:391-393` (name inscription)

### Requirement: Naming

Artifact naming SHALL take two paths: an object created by the player through the
artifact scroll is fully identified and can be named by hand (the default name is
"of '<player name>'"); everything else takes a random name — a first-letter-
capitalized word of 5 to 9 letters containing at least one vowel, generated from the
prebuilt syllable probability table, presented as a quoted word with one third
chance and otherwise as "of word".

#### Scenario: Random name

- **WHEN** a dungeon-generated artifact needs a name
- **THEN** a random word is generated from the syllable probability table and one of
 the two presentations is picked at 1/3 odds

- **Anchors**: `src/randart.c:159-254` (probability table build and word
 generation), `src/randart.c:257-265` (the two presentations),
 `src/randart.c:361-389` (the two-path selection)

### Requirement: Artifact Scroll

The artifact scroll SHALL only act on artifactable items: an object that is already
an artifact or an ego item is refused with a message; a stacked pile warns and
destroys the extra pieces, keeping one; a successful creation is marked as
self-made; a failure prints the failure message (and flushes input per the setting).

#### Scenario: Stack destruction

- **WHEN** the artifact scroll is used on a pile of three stacked objects
- **THEN** two pieces are destroyed and the remaining one goes through
 artifact creation

- **Anchors**: `src/randart.c:414-494`

### Requirement: Activation Status

Random artifact activation assignment SHALL be in an unimplemented state: assembly
does not grant the activation flag and the activation type field is cleared (the
source notes the feature as todo).

#### Scenario: No activation

- **WHEN** a random artifact finishes assembling
- **THEN** the item carries no activation flag and the activation type is 0

- **Anchors**: `src/randart.c:117-132`, `src/randart.c:135-144`
