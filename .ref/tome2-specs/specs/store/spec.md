# store Specification

## Purpose

Word-list contract of the stores: how `lib/edit/st_info.txt` declares a store —
appearance, stock table (keyed either by name template or by tval/sval, each row with an
appearance percentage), the six building actions, the owner list, the stock cap, the
store flags — and how the engine parses and loads it at startup. Store restocking,
trading and prestige pricing are specified in specs/store-runtime/spec.md; how the six
actions are presented and dispatched is specified in specs/building/spec.md. The store
flag name list is the source table at the anchored location; each entry is content data
and is not analyzed line by line.

The common file skeleton is specified in specs/edit-format/spec.md.

## Requirements

### Requirement: Name-Keyed Stock

A store SHALL support registering stock by item name over multiple `I:` rows:
`percentage:name template` (the percentage is an appearance chance out of 100); the name
template is compared case-insensitively against the full entry names of the object
word-list and resolves to an object index, with 0 registered when nothing matches; the
stock table is cleared and rebuilt when an entry begins.

#### Scenario: Name comparison

- **WHEN** an `I:` row supplies `100:& Wooden Torch~`
- **THEN** the template is compared against the object word-list entry names and a hit
 registers that index with appearance chance 100; no hit registers 0

- **Anchors**: `lib/edit/st_info.txt:11-13` (header note), `:23-27` (sample), `:29-30` (first entries); `src/init1.c:10062-10087`, name comparison `src/util.c:4414-4428`

### Requirement: Category-Keyed Stock

A store SHALL support registering stock by tval/sval over multiple `T:` rows:
`percentage:tval:sval`; a sval below 256 resolves through the object table to a concrete
index, a sval of 256 or more means every item of that tval (registered encoded as tval +
10000); an unknown tval registers 0.

#### Scenario: Whole category

- **WHEN** the sval column of a `T:` row is 256 or above
- **THEN** the row is registered as a whole-tval entry and restocking covers every item
 of that tval

- **Anchors**: `src/init1.c:10089-10107` (with the `tv1 + 10000` encoding)

### Requirement: Store Parameters

A store SHALL support: a `G:` line declaring the map appearance (symbol and color name,
an invalid color name is refused); an `A:` line declaring the six building action
indexes (into the building-action word-list); an `O:` line declaring four owner indexes
(into the owner word-list); a `W:` line declaring the stock cap (clamped to the built-in
stock cap when exceeded).

#### Scenario: Cap clamping

- **WHEN** the `W:` line declares more stock slots than the built-in cap
- **THEN** the effective stock cap is the built-in cap value

- **Anchors**: `src/init1.c:10109-10215` (`W:` clamped at `:10210`)

### Requirement: Store Flags

A store SHALL support multiple `F:` flag rows: names separated by spaces or pipes,
resolved through the store flag name table, with an unknown name refused.

#### Scenario: Unknown flag

- **WHEN** an `F:` row contains a flag name absent from the name table
- **THEN** parsing fails with an unknown-store-flag error

- **Anchors**: `src/init1.c:10154-10179`, table lookup `src/init1.c:9908-9927`
