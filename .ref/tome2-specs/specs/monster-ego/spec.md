# monster-ego Specification

## Purpose

The word-list contract for monster egos: how `lib/edit/re_info.txt` declares a title
attached to an existing monster — the applicable and excluded monster ranges (flags
and display symbols), numeric modifiers (with the `=` / `+` / `-` / `%` operators),
blow group add/remove, flag and spell add/remove, and name position — and how the
engine parses and loads it at startup. The application of egos at monster generation
and the arithmetic of the numeric modifiers are specified in
specs/monster-generation/spec.md (selection and synthesis). Flag, method, and effect
name lists follow the source
tables at the anchors; entries are content data and are not analyzed one by one.

The common file skeleton is described in `specs/edit-format/spec.md`. Note: the file
header comment claims an `E:` body-part line, but the parser has no branch for it and
rejects such lines as unknown — the documentation and the code disagree, and the code
is authoritative.

## Requirements

### Requirement: Numeric Modifier Operators

An ego's numeric columns SHALL carry a single-character modifier operator prefix: `=`
sets, `+` adds, `-` subtracts, `%` applies as a percentage; each value and its
operator are packed together on load (the value is shifted left two bits, the low two
bits store the operator class); a missing operator fails the field scan and rejects
the line. The `I:`
line's six numeric values (speed, hit dice, sides, aaf, armor, sleepiness) and the
`W:` line's level, weight, and experience are all loaded this way; the rarity column
takes no operator; the `W:` line's last column is the name position (`B` places the
title before the name).

#### Scenario: Operator packing

- **WHEN** an `I:` line gives operator-tagged values in the `%100:+1d+1:...` form
- **THEN** each value is stored together with its operator class, and at runtime the
 operator is applied to the base monster's value

#### Scenario: Unknown operator

- **WHEN** a value column's operator character is not `=`, `+`, `-`, or `%`
- **THEN** the warning "Unknown mego value modifier X." is printed and the value
 loads with the add operator (the line is not rejected)

- **Anchors**: `lib/edit/re_info.txt:11-14` (operator header note),
 `lib/edit/re_info.txt:37` (`I:%100:+1d+1:+0:+5:-5` sample), `src/init1.c:8501-8521`
 (I:), `src/init1.c:8523-8543` (W:), operator translation `src/init1.c:1486-1504`

### Requirement: Appearance Inheritance

An ego SHALL support a `G:` line declaring inheritance of the base monster's symbol or
color through the `*` wildcard; without the wildcard the normal appearance rules load
the values, and an invalid color name is rejected.

#### Scenario: Wildcard inheritance

- **WHEN** a `G:` line gives `s:*`
- **THEN** the symbol is stored as `s` and the color field is registered as the
 inherit marker, so at runtime the base monster's color is kept

- **Anchors**: `lib/edit/re_info.txt:17` (wildcard note), `src/init1.c:8471-8499`

### Requirement: Blow Group Override

An ego SHALL support up to four `B:` blow declarations: the method and effect names
MUST be in the monster blow name tables; damage dice and sides each carry modifier
operators; an overflow past the slots is rejected. At entry start the blow list is
cleared and the default operator is `+`.

#### Scenario: Operator-tagged damage

- **WHEN** a `B:` line gives text in the `BITE:HURT:+1d+2` form
- **THEN** the method, the effect, and the two operator-tagged damage values are
 loaded

- **Anchors**: `lib/edit/re_info.txt:21`, `src/init1.c:8545-8602`, default operator
 `src/init1.c:8454-8462`

### Requirement: Applicable And Excluded Ranges

An ego SHALL support multiple `F:` lines (the base monster MUST have these) and `H:`
lines (the base monster MUST NOT have these) as applicability conditions:
`R_CHAR_<symbol>` entries register the monster display symbols allowed to carry the
ego on an `F:` line and the symbols excluded from it on an `H:` line (at most five
each; entries beyond five are silently ignored); all other names
resolve in the ego flag table, and an unknown name is rejected.

#### Scenario: Symbol range

- **WHEN** an `F:` line contains `R_CHAR_Z` alongside named flags
- **THEN** symbol Z is registered into the carrier list and the named flags are
 registered into the required flag words

- **Anchors**: `lib/edit/re_info.txt:22-25`, `src/init1.c:8604-8692`; ego flag
 loading from `src/init1.c:8245`

### Requirement: Flag Add And Remove

An ego SHALL support multiple `M:` lines (add flags) and `O:` lines (remove flags):
names resolve in the monster base flag table, and an unknown name is rejected; an
`O:` line containing `MF_ALL` fills every base-flag removal word (removing
everything).

#### Scenario: Removing everything

- **WHEN** an `O:` line contains `MF_ALL`
- **THEN** all removal words corresponding to the base flags are filled

- **Anchors**: `lib/edit/re_info.txt:26-27`, `src/init1.c:8694-8759`, `MF_ALL` at
 `src/init1.c:8737-8748`

### Requirement: Spell Add And Remove With Frequency

An ego SHALL support multiple `S:` lines (add spells) and `T:` lines (remove spells):
an `S:` line may contain `1_IN_X` declaring the casting frequency (100 divided by X);
names resolve in the monster spell-flag table, and an unknown name is rejected; a
`T:` line containing `MF_ALL` fills every spell-flag removal word.

#### Scenario: Removing every spell

- **WHEN** a `T:` line contains `MF_ALL`
- **THEN** all removal words corresponding to the spell flags are filled

- **Anchors**: `lib/edit/re_info.txt:28-29`, `src/init1.c:8761-8839`, `MF_ALL` at
 `src/init1.c:8817-8828`
