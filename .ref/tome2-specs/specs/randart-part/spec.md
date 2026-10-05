# randart-part Specification

## Purpose

The word-list contract for randart parts: how `lib/edit/ra_info.txt` declares one
part that can be combined at random onto an artifact — the applicable object range,
the power value and per-part occurrence cap, appearance depth with the double rarity,
modifier caps, flags and antagonist flags — plus the global power-count generation
table; and the parse/load at engine startup. Part combination and value selection
during random artifact generation are recorded with the `src/randart.c` and
`src/wizard1.c` analyses. The complete flag-name lists are the source tables at the
anchored spots; individual entries are content data and are not analyzed one by one.

The common file skeleton is described in `specs/edit-format/spec.md`. Entries in this
word-list carry only an index and no name, with indices non-decreasing (an index
equal to the previous one passes the check); there are additionally `G:` global lines
registering into the global generation table — nothing enforces their placement
outside entries.

## Requirements

### Requirement: Power Count Generation Table

A `G:` line SHALL declare one global power-count generation parameter: the appearance
weight, the NdM-form power count dice and the numeric bonus; each line registers
independently into the global generation table, from which the random artifact
generation draws its power count.

#### Scenario: Generation table registration

- **WHEN** the file head carries `G:100:1d5:1`-form rows
- **THEN** each row's weight, count dice and bonus register into the global
 generation table

- **Anchors**: `lib/edit/ra_info.txt:24-27`; `src/init1.c:7322-7340`

### Requirement: Entry Defaults

A part entry SHALL begin with injected defaults: the granted-power field set to -1;
all twenty applicable-tval slots set to 255; the five flag groups, the sense bits and
the title flag word all cleared.

#### Scenario: Defaults

- **WHEN** a part entry starts parsing
- **THEN** it has no applicable tvals and no flags until `T:`/`F:` lines fill them
 in

- **Anchors**: `src/init1.c:7360-7374`

### Requirement: Part Parameters

A part SHALL support: an `X:` line declaring the power value and the per-part
occurrence cap (two columns); a `W:` line declaring the minimum player level required
for generation and the double rarity (three columns); a `C:` line declaring the four
modifier caps (to-hit, to-dam, to-ac, pval); and `T:` lines declaring the applicable
tval and sval range (up to twenty lines, beyond that the entry is rejected).

- **Discrepancy:** the `ra_info.txt` header lists the `C:` columns as "max to dam:
 max to hit: ...", but the parser reads to-hit first, to-dam second.

#### Scenario: Tval range

- **WHEN** a `T:` line carries the three-column `6:0:255` form
- **THEN** objects of that tval with svals 0 through 255 can carry this part; a
 twenty-first `T:` line rejects

- **Anchors**: `src/init1.c:7383-7403` (T:), `src/init1.c:7405-7456` (X:/W:/C:)

### Requirement: Flags And Antagonist Flags

A part SHALL support multiple `F:` lines (granted flags) and `A:` lines (antagonist
flags): names are separated by spaces or vertical bars, resolved through the object
flag name table into the granted flag word and the antagonist flag word respectively
(two independent words) — an unknown name rejects; an `F:` line may additionally use
the title flag name table.

#### Scenario: Two flag words

- **WHEN** the same flag name appears once in an `F:` line and once in an `A:` line
- **THEN** the granted flag word and the antagonist flag word are each set in their
 own word; neither overwrites the other

- **Anchors**: `src/init1.c:7480-7532` (F:/A:), the dispatch loader
 `src/init1.c:7128-7240`

### Requirement: Granted Power

A part SHALL support a `Z:` line granting one existing power (the power name is
looked up case-insensitively in the power table; an unknown name rejects).

#### Scenario: Unknown power

- **WHEN** the power name of a `Z:` line is not in the power table
- **THEN** parsing fails with the lookup error

- **Anchors**: `src/init1.c:7458-7478`
