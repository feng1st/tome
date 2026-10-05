# store-owner Specification

## Purpose

Word-list contract of the store owners: how `lib/edit/ow_info.txt` declares an owner —
name, pricing parameters (price cap, price-inflation range, haggling tolerance, insult
cap before ejection), prestige-tiered costs, and the liked/hated race/class lists — and
how the engine parses and loads it at startup. The owner-to-store binding is declared by
the store's `O:` rows (see specs/store/spec.md) and the effect of standing on prices is
specified in specs/store-runtime/spec.md. Race/class names resolve through the parser
functions at the anchored location; each entry is content data and is not analyzed line
by line.

The common file skeleton is specified in specs/edit-format/spec.md.

## Requirements

### Requirement: Pricing Parameters

An owner SHALL declare five pricing parameters with an `I:` line: price cap, highest
inflation, lowest inflation, inflation decrement per haggle round, and the insult cap
before ejection.

#### Scenario: Five-column loading

- **WHEN** an `I:` line supplies five values
- **THEN** the five pricing parameters are stored one by one into the owner entry

- **Anchors**: `lib/edit/ow_info.txt:11`; `src/init1.c:10567-10585`

### Requirement: Tiered Costs

An owner SHALL declare three cost tiers with a `C:` line: hated, normal, liked.

#### Scenario: Three-tier loading

- **WHEN** a `C:` line supplies three values
- **THEN** the three costs are stored one by one as hated/normal/liked

- **Anchors**: `lib/edit/ow_info.txt:12`; `src/init1.c:10549-10565`

### Requirement: Liked And Hated Lists

An owner SHALL support declaring race/class lists over multiple `L:` (liked) and `H:`
(hated) rows: list items are separated by spaces or pipes, each item resolves through
the race table or the class table (race first, then class), and an item found in neither
is refused; the same name may sit in both a hated and a liked list — the hated and the
liked lists are registered separately.

#### Scenario: List item resolution

- **WHEN** an `L:` row supplies a list of the form `Elf | Hobbit`
- **THEN** each item is resolved through the race/class tables in turn and registered as
 liked; an unresolvable item is refused

- **Anchors**: `lib/edit/ow_info.txt:13-14`, `:17-18` (header), `:23-24` (sample lists); `src/init1.c:10587-10638` (L:/H: loading), `src/init1.c:9877-9903` (race/class table lookup)
