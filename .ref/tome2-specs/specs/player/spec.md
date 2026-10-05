# player Specification

## Purpose

Word-list contract for the playable character: `lib/edit/p_info.txt` declares, in one file,
four record classes - races (`R:` prefix), subraces (`S:` prefix), classes (`C:` prefix) and
metaclasses (`M:` prefix) - plus the skills granted to every character at birth (`G:k:`) and
the birth-history table (`H:`). The second character selects the subtype (for example
`R:S:` holds a race's six-stat modifiers). How the birth flow consumes this data (stat
rolling, modifier merge, skill initialization) is specified in
specs/character-birth/spec.md. Skill, power and flag name lists are whatever the source
tables at the anchored lines define; the entries themselves are content data and are not
enumerated here.

The common file skeleton is specified in specs/edit-format/spec.md. Sections are separated
by `I:` lines, which reset the index validation; an index that regresses within a section
is rejected.

## Requirements

### Requirement: Sectioning And Index Reset

`I:` lines SHALL act as section separators in the word-list: each `I:` line resets the
index-validation baseline, so the race, subrace, class and metaclass sections may each
declare from low indexes again; an index that regresses inside one section is still
rejected.

#### Scenario: Section restart

- **WHEN** an entry with index 0 appears again after an `I:` line
- **THEN** the entry is accepted; an index regression inside the same section is still
 rejected

- **Anchors**: `src/init1.c:1888-1893`, in-section regression check `src/init1.c:1961`

### Requirement: Common Skills

`G:k:` lines SHALL declare skills that every character owns at birth:
`base:modifier:skill-name` (the values carry the `=`/`+`/`-`/`%` modifier operators); the
skill name MUST resolve in the skill table - an unknown name is rejected.

#### Scenario: Operator decoration

- **WHEN** `G:k:+1000:+0:Spell-learning` loads
- **THEN** that skill's base value is registered with the `+` operator in the global skill
 table

- **Anchors**: `lib/edit/p_info.txt:14-18`, `src/init1.c:1922-1940`, operator translation
 `src/init1.c:1486-1499`

### Requirement: Birth History Table

`H:` lines SHALL declare birth-history entries with six fields
`index:dice:chart:next:bonus:text`; the text field is registered as that entry's birth
description. The behavior for an out-of-range index depends on the capacity of the birth
history table.

#### Scenario: Entry loading

- **WHEN** an `H:` line provides all six fields
- **THEN** dice, chart, next, bonus and the description text are written into the birth
 history table at that index

- **Anchors**: `src/init1.c:1895-1920`

### Requirement: Race And Subrace Records

Race (`R:`) and subrace (`S:`) records SHALL support parallel subtype sets:

- name (`N`);
- newline-joined description (`D`; a subrace `D` additionally carries a first column that
 is the name-position marker - `A` there sets the subrace's `place` flag, any other value
 clears it);
- body parts (`E`);
- flag-level switch (`R:level:pval`);
- stat modifiers (`S`; the race row has seven columns with luck last, the subrace row has
 eight columns with luck and mana as the last two);
- skill modifiers (`k`, with operators; the skill name must resolve);
- power prerequisite (`b`; the power name must resolve);
- physique parameters (`M`, ten columns: age plus male and female height/weight);
- additional parameters (`P`; the race row has four columns including the birth chart
 number, the subrace row has three);
- player flags (`G`);
- level flags (`F`, written into the per-level flag word of the most recent `R:` level);
- starting objects (`O`, `tval:sval:NdM` with an optional pval field between sval and the
 dice);
- granted powers (`Z`, at most four; power names are looked up case-insensitively).

Subraces additionally support `A:` (the playable race list) and `C:` (the class choice
list).

#### Scenario: Level flag layering

- **WHEN** a race opens a level-20 flag layer with `R:R:20:1` and then writes flag names
 with `R:F:`
- **THEN** the flags are written into the level-20 layer flag word and have no effect below
 that level

#### Scenario: Subrace-only lists

- **WHEN** a subrace carries an `S:A:` line
- **THEN** the list entry is registered into the race list that subrace allows

- **Anchors**: race section `src/init1.c:1943-2314`, subrace section
 `src/init1.c:2317-2700`, `S:A:` at `src/init1.c:2666-2675`, flag lookup
 `src/init1.c:1704`, near `src/init1.c:2159` (player flags)

### Requirement: Class Records

Class (`C:`) records SHALL support the subtypes:

- name (`N`);
- description and titles (`D:0:` description, `D:1:` per-level titles);
- starting objects (`O`);
- body parts (`E`);
- flag levels (`R`);
- eight-column stats (`S`: six stats, mana, extra blows);
- skill modifiers (`k`);
- power prerequisites (`b`);
- god list (`g`);
- granted powers (`Z`);
- base skills (`K`, eight columns);
- extra skills (`X`, eight columns);
- hit dice and experience rate (`P`, two columns);
- perception parameters (`C`, heavy/light perception columns plus three parameters);
- blows parameters (`B`, three columns);
- player flags (`G`);
- level flags (`F`).

#### Scenario: Eight-column stats

- **WHEN** a `C:S:` line provides eight numeric columns
- **THEN** the six stat modifiers, the mana bonus and the extra blows count are written
 into the class one by one

- **Anchors**: `lib/edit/p_info.txt:23-35` (subtype header notes), class section
 `src/init1.c:2728-3150`, `C:g:` at `src/init1.c:2961`, `C:C:` at `src/init1.c:3060`

### Requirement: Class Specialities

Classes SHALL support a speciality list: `C:a:N:` opens one speciality (a count above the
built-in cap is rejected); beneath it `C:a:D:` (description), `C:a:O:` (starting object),
`C:a:g:` (gods), `C:a:k:` (skill modifiers), `C:a:b:` (powers), `C:a:G:` (player flags) and
`C:a:K:` (target skill and value) each load.

#### Scenario: Speciality cap

- **WHEN** the speciality count exceeds the built-in cap
- **THEN** parsing fails with an out-of-bounds error

- **Anchors**: `lib/edit/p_info.txt:37-42`, speciality section `src/init1.c:3151-3360`,
 cap check `src/init1.c:3166`

### Requirement: Metaclass

An `M:N:` line SHALL declare one metaclass (the class-combination layer) and `M:C:` lines
take member classes by name: names are compared case-insensitively against existing class
titles; an unknown name is rejected.

#### Scenario: Combining classes

- **WHEN** an `M:C:` line names a class that does not exist
- **THEN** parsing fails with a lookup error

- **Anchors**: `src/init1.c:3360-3438`
