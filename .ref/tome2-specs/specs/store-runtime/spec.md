# store-runtime Specification

## Purpose

Store runtime: pricing (the three standing tiers, charisma, owner greed, black-market
multiplier), mass production and discounts, stock merging and capacity, the full
buy/sell haggling flow (asking price / final price / incremental haggling / annoyance
ejection / fixed prices / haggle tax), stealing, examination, store upkeep and owner
rotation, the home and museum special cases, and the merchant purchase request. The
store and owner word-list contracts are specified in specs/store/spec.md and
specs/store-owner/spec.md; building action dispatch is specified in
specs/building/spec.md.

## Requirements

### Requirement: Pricing

A single-unit price SHALL be the object value times an adjustment factor composed of
three inputs: the standing-tier cost (liked/normal/hated taken from the owner's matching
tier), the charisma adjustment table, and owner greed (when buying, the greed premium
never drops the factor below 100; when selling, the discount never pushes it above 100 —
so the price a buyer pays only rises and the price a seller receives only falls). The
black market SHALL charge double when selling to the player and pay half when buying
from the player (`SF1_ALL_ITEM`). Ammunition kinds SHALL be forced to one fifth of the
value when purchased by a shop. Zero-value items SHALL not be traded, and a price is at
least one gold.

#### Scenario: Black market spread

- **WHEN** the same item is valued at the black market and at an ordinary store
- **THEN** the black market asks double and offers half

- **Anchors**: `src/store.c:365-444` (black market at `:420`/`:433`)

### Requirement: Stocking And Discounts

Restocking SHALL attempt at most four items per pass: a Lua hook may define the whole
item; the black market draws randomly over all themes and prunes junk (an item whose
kind is also stocked by another store in town — ego items and items with a positive
to-hit, to-dam or armor bonus are exempt — or value under ten gold); an ordinary store draws from its word-list
table by appearance chance (an index over 10000 expands to the whole tval, filtered by
level), with artifact base kinds and ready-made artifacts excluded and worthless items
pruned; finished items are always identified and carry the store-bought mark, and chests
are banned. Cheap items SHALL be mass-produced into piles (pile dice accumulated by
tval and value tier) and receive a 25/50/75/90 percent discount rolled by rarity
(random artifacts are never discounted); wand charges are multiplied by the pile size.
The stock level SHALL be composed from the store flags (random / plus dungeon depth /
shallow-medium-deep bonuses / black market plus player level).

#### Scenario: Random artifact skips the discount

- **WHEN** mass production rolls a discount for a random artifact
- **THEN** the discount is cancelled (announced under the cheat-peek option)

- **Anchors**: `src/store.c:1147-1332` (stocking main flow), `:462-586` (mass production and discounts), `:1094-1111` (level composition)

### Requirement: Stock Merging

Store stock SHALL merge under strict homogeneity (same kind, same sval, same pval, same
pval2/pval3, same to-hit/to-dam/to-ac, same artifact, same ego, same flags, same
discount; no random-power items on either side; wands may merge across charges and their
charges add up; light sources need equal fuel; ordinary items need both timeouts clear;
chests never merge). The home and the museum SHALL follow the player merge rules instead
of the store rules. With stock full, only merging into an existing pile is possible.
Items sold to a store SHALL be fully known, stripped of inscriptions, and shelved;
worthless items are destroyed outright. The home inserts by descending tval, ascending
sval (aware items first), identification state, increasing rod timeout for otherwise
equal rods, then descending value; a store the same way without the awareness and
identification passes.

**Discrepancy:** the merge helper's comment credits rods as well, but only `TV_WAND`
may stack across differing charges; the charge pooling on absorb is `TV_WAND`-only too.

#### Scenario: Wand charge merging

- **WHEN** two stacks of the same wand kind end up in a store
- **THEN** the stacks combine into one and the charges add up

- **Anchors**: `src/store.c:600-683` (merge rules), `:691-730` (capacity check), `:780-967` (home and store shelving)

### Requirement: Haggling

Buying SHALL start from the highest asking price with the final price as the floor;
selling SHALL start from the lowest offer with the owner's purse as the cap (touching
the purse cap closes the deal instantly). Haggling SHALL converge stepwise: each
round the merchant's price steps toward the player's bid by a proportion of the
remaining gap (the step is randomly offset around the offer ratio), reaching the
final price shows "Final Offer", and a fourth ineffective offer in a row ejects the
player. The incentive applies both ways: cheap trades skip haggling, and once
the accumulated successful haggles exceed `3 x failures + 5 + price/50` haggling is
skipped permanently; the auto-haggle option deals straight at the final price plus a
ten-percent haggle tax (minus ten percent when selling). Success/failure counts SHALL be
recorded at deal time by whether the final price was reached; auto-haggled deals and
trades under ten gold record nothing. Ejection SHALL close the
store for 25000 to 50000 turns.

#### Scenario: Veteran haggler skips the ritual

- **WHEN** the player's successful haggles at this store far exceed the failures
- **THEN** later trades close straight at the final price with the eventual-agreement
 message

- **Anchors**: `src/store.c:1968-2140` (purchase haggling), `:2148-2337` (selling haggling), `:1339-1388` (skip rule and counts), `:1752-1803` (annoyance and ejection)

### Requirement: Buying And Selling

Buying SHALL split into the home (take items for free) and stores (a fixed-price slot
closes the deal directly, otherwise haggling; wand charges are apportioned by count);
the quantity prompt SHALL estimate the affordable maximum from held gold and the unit
price. Selling SHALL distinguish stores (haggled deal, shelved with the haggle record
improved by the deal, and the buy/sell value gap triggers one of four owner comments),
the museum (donated for good after confirmation, shelved fully known, never taken back)
and the home (shelved directly). Cursed worn equipment cannot be sold, and equipment
carrying the no-drop curse cannot be dropped.

#### Scenario: Fixed-price lock

- **WHEN** the player buys a slot's item at exactly the best price
- **THEN** that slot later closes quickly at the fixed price

- **Anchors**: `src/store.c:2584-2945` (buying), `:2951-3285` (selling), `:277-318` (owner comments)

### Requirement: Stealing

Stealing SHALL be limited to non-home stores: the success roll combines the dexterity
adjustment, the total item weight and the stealing skill; a stolen item is marked with a
stolen origin, its discount is set to the full 100 percent (it cannot profitably be
resold to the original store), its fixed-price mark is cleared, and the player's
knowledge of it is granted. Failure SHALL trigger the ejection dialogue and close the
store for roughly 500000 turns. Emptying the shelves rolls for an owner change
(otherwise ten restock rounds run).

#### Scenario: Reselling the loot

- **WHEN** the player tries to sell a stolen item back to its original store
- **THEN** the full discount prevents any profit

- **Anchors**: `src/store.c:2365-2579`

### Requirement: Store Interaction And Upkeep

Entering a store SHALL first run up to ten maintenance rounds scaled by the absence
duration (maintenance also clears the insult counter, prunes black-market junk, and
trims and refills the stock between its lower and upper bounds by the turnover rate);
within the lock-out window entry is refused. Inside, the store SHALL support page
browsing, the whole dungeon command subset it enables, and the six action letters; pack
overflow expels the player (in the home the overflow drops into the home instead); a
charisma change SHALL refresh the price list immediately. An owner change SHALL rotate
the owner at random and reset the counters, sell off the old stock at half price (random
artifacts excepted) inscribed `on sale`. The merchant purchase request SHALL resolve
through the wish parser and deliver the item to the black market at five times its real
value.

#### Scenario: Upkeep window

- **WHEN** the player returns to a store after a long absence
- **THEN** the absence is converted into maintenance rounds (capped at ten) which first
 trim and then restock

- **Anchors**: `src/store.c:3725-3991` (entering loop), `:3998-4051` (owner rotation), `:4057-4142` (upkeep), `:4148-4185` (initialization), `:4489-4531` (purchase request)
