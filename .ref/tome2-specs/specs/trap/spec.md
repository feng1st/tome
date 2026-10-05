# trap Specification

## Purpose

The word-list contract for object traps: how `lib/edit/tr_info.txt` declares a
trap — disarm difficulty, appearance chance, overlap chance, parameter
contribution, minimum depth, damage dice, and color — and how the engine parses
and loads it at startup. Trap placement and triggering in `src/traps.c` are
covered by the runtime requirements below. The flag name list is the source table
at the anchored location; individual entries are content data and are not
enumerated here.

The shared file skeleton is specified in `specs/edit-format/spec.md`. This
word-list's entry indexes are strictly increasing (equal indexes are rejected
too).

## Requirements

### Requirement: Trap Parameters

A trap SHALL declare seven parameters with an `I:` line: disarm difficulty,
appearance chance (per-mille), overlap chance (per-mille), parameter
contribution, minimum depth, damage dice (`NdM` form), and a single-letter color
(translated to an engine color value). All eight fields MUST be present; the
shorter form shown in the file header comments is not accepted by the parser —
the parser is authoritative.

#### Scenario: Eight columns loaded

- **WHEN** an `I:` line carries eight fields
- **THEN** the seven parameters and the color are each stored; any missing field
 is rejected

- **Anchors**: `lib/edit/tr_info.txt:9-17` (field header note), `:31-33` (short
 form header note, contradicting the code); `src/init1.c:9012-9037`

### Requirement: Flag Group Overwrite

An `F:` line SHALL zero the flag word first and then write this line's flags:
with multiple `F:` lines the later line overrides the earlier one, and flags do
not accumulate across lines; names are separated by spaces or vertical bars and
resolve through the trap flag name table, with an unknown name rejected.

#### Scenario: Overwrite semantics

- **WHEN** an entry carries two flag lines, `F:A` and then `F:B`
- **THEN** the final flag word contains only B; A is overwritten

- **Anchors**: `src/init1.c:9063-9091` (zeroing at `:9067`), name table and
 loading `src/init1.c:8860-8878`

### Requirement: Description

A trap SHALL support `D:` lines carrying description text (appended directly).

#### Scenario: Loading

- **WHEN** a `D:` line gives text
- **THEN** the text is registered as the trap's description

- **Anchors**: `src/init1.c:9040-9060`

### Requirement: Trigger Protocol and Identification

Trap triggering SHALL dispatch the effect by the trap's type index: a floor trap
takes the trap slot of the grid, a chest trap takes the item's pval field. The
effect returns whether an observable effect happened; when it does, the trap type
is identified and its name is announced to the player.

#### Scenario: Door trap linkage

- **WHEN** the player walks into a trapped door
- **THEN** the trap type is re-randomized first and triggered immediately, and a
 successful identification announces that trap's name

- **Anchors**: `src/traps.c:456-459` (the two sources), `src/traps.c:1959-1989`
 (door linkage)

### Requirement: Stat Decay Family

Each of the six stats SHALL have three decay traps causing temporary, normal, and
permanent stat loss respectively (eighteen traps in total).

#### Scenario: Three tiers

- **WHEN** the I/II/III traps of the same stat are triggered in order
- **THEN** they cause temporary, then normal, then permanent stat loss

- **Anchors**: `src/traps.c:477-531`

### Requirement: Curse and Destruction Family

The cursed weapon and cursed armor traps SHALL call the matching curse routines;
the earthquake trap SHALL trigger a radius-ten earthquake centered on the trigger
point; the explosive device SHALL deal 5d8 direct damage; the wall trap SHALL
turn grids in the player's 5x5 neighborhood into walls or floor with a
depth-dependent chance — trapped ordinary monsters take 4d8 (200 points when
there is nowhere to flee) and may be entombed alive, wall-passing and
wall-killing monsters are immune, and room and memory marks are cleared
throughout. The element damage traps SHALL cover every element type with two
families, bolt-shaped (radius one) and ball-shaped (radius three); the damage
dice deepen step by step past twice the minimum depth, scaled by the deepest
reached depth.

#### Scenario: Wall trap entombment

- **WHEN** a grid turns into a wall around an ordinary monster with nowhere to
 flee and its hit points drop below zero
- **THEN** the monster is deleted rather than killed normally, with an entombment
 message

- **Anchors**: `src/traps.c:533-554` (curses and earthquake), `:632-639`
 (explosion), `:1031-1036`, `:138-365` (wall trap), `:417-437` (element depth
 scaling), `:1658-1807` (bolt/ball table)

### Requirement: Summoning Family

The summoning traps SHALL come in three tiers (ordinary monsters, undead, greater
undead), each summoning one to three creatures around the trigger point at the
deepest reached dungeon depth; the fast quylthulg trap SHALL additionally slow the
player; the calling-out trap SHALL summon the highest-level monster on the level
to a grid adjacent to the player (with no monster to call, it converts into a
nightmare or fear per resistances).

#### Scenario: Calling out with nothing to call

- **WHEN** the calling-out trap fires with no monster on the level
- **THEN** it converts into a nightmare or a frightened state per the save and
 resistance

- **Anchors**: `src/traps.c:572-605` (the three summon tiers), `:815-829` (fast
 quylthulg), `:1038-1063` (calling out), `:16-74` (the call-out implementation)

### Requirement: Teleport and Trapdoor Family

The teleport trap SHALL teleport the player far away; the item teleport trap SHALL
throw every object underfoot to random floor grids across the level (the One Ring
is immune); the trapdoor trap SHALL drop the player one level down (up one level
in tower dungeons instead), with floaters landing unharmed and the rest taking 2d8
fall damage, saving the game first when the level autosave option is on.

#### Scenario: Trapdoor in a tower

- **WHEN** the trapdoor trap fires in a tower dungeon and the player can levitate
- **THEN** the player lands gently on the previous level instead of the next

- **Anchors**: `src/traps.c:607-614` (teleport), `:641-662`, `:76-133` (item
 teleport, One Ring immunity at `:90`), `:831-867` (trapdoor)

### Requirement: Mind and Memory Family

The lose-memory trap SHALL remove a quarter of experience, decay wisdom and
intelligence, and add confusion per resistance; the bitter-regret trap SHALL lower
all six stats by 25; the blindness/confusion trap SHALL add blindness and
confusion per resistance; the paralysis trap SHALL paralyze when free action is
absent; the bowel-cramps trap SHALL push the player to starving, cure poison, and
may paralyze; the hallucination trap SHALL inflict hallucination.

#### Scenario: Resistance gate

- **WHEN** the blindness/confusion trap fires on a player resistant to blindness
- **THEN** only the confusion part applies; the blindness part is skipped

- **Anchors**: `src/traps.c:664-686` (lose memory), `:687-699` (bitter regret),
 `:717-731` (blindness/confusion), `:616-630` (paralysis), `:701-715` (bowel
 cramps), `:1916-1923` (hallucination)

### Requirement: Item Interference Family

The item traps SHALL cover: theft (a save check first, then random non-artifact
pack items are whisked away and thrown into the distance), charge drain (ten
random drain rounds across wands and staves, artifacts immune), wand wastage (one
chance in five turns a wand/staff into its "nothing" variant, identification
lost), no return (burns recall scrolls, zeroes the recall rod's recharge timer,
breaks a pending recall), silent exchange (swaps an equipped item with a pack item in the
same slot, permanently cursed items immune), speed drain (halves the pval of a
non-artifact speed item, halving the chance again after each hit), scatter (each
pack item has a 30% chance to stay, the rest is dropped over fifteen surrounding
grids), the three drop tiers (shaking pack or equipment items to the player's feet
with tier-dependent chances, One Ring immune), gold theft (a share of current gold
with lower and upper clamps), mana drain (mana to zero), trap proliferation (new
traps in the 3x3 neighborhood), filling (probabilistically filling a 17x17 area
with new traps), the gain-item trap (drops one good item and re-traps, never
identified), the new-trap trap (re-traps in place, no identification), and stair
movement (every staircase on the level swaps positions; refused when the deepest
reached depth of the dungeon is 99).

#### Scenario: Theft save

- **WHEN** the player's dexterity-based save modifier plus level is high enough
- **THEN** the pack only shakes and nothing is stolen

- **Discrepancy:** the no-return trap zeroes the recall rod's `timeout` — which
 makes the rod ready to use at once, since `timeout` counts down to usability —
 while the code comment says "a long time", so the rod is not actually
 neutralized.
- **Anchors**: `src/traps.c:758-813` (theft), `:1069-1108` (charge drain),
 `:1352-1396` (wand wastage), `:922-967` (no return), `:969-1029` (exchange),
 `:1419-1471` (speed), `:1293-1346` (scatter), `:1554-1656` (three drop tiers),
 `:892-920` (gold), `:869-891` (mana), `:741-756` (proliferation), `:1398-1417`
 (filling), `:1264-1291` (gain item), `:1241-1262` (new trap), `:1110-1239`
 (stair movement)

### Requirement: Polymorph and Deity Family

The body-alteration traps SHALL cover: the three-state sex change (rewrites sex
directly and deals damage), aging (adds a random number of years up to half the
sum of the race's and race modifier's base ages), growing and shrinking (random quarter of the sex's base height added or
removed, shrinking clamped at a floor); the deity traps come in two tiers —
anger (piety reduced by 3000) and wrath (piety reduced by 500 times level), while
a player without a god only gets a taunting message. Every body-alteration trap also
delivers one standard trap damage roll.

#### Scenario: Shrink floor

- **WHEN** the shrinking trap pushes height down to a quarter of the race's base
 height
- **THEN** height is clamped at that floor

- **Anchors**: `src/traps.c:1809-1877` (the four alteration states),
 `:1879-1914` (the two deity tiers)

### Requirement: Trap Placement

Floor trap placement SHALL require: depth above one, not an open door, and a
walkable floor or door grid; per grid type only the floor or door trap flag group
qualifies, and within a hundred draws a trap is picked matching minimum depth,
flags, and appearance chance; trapdoors are excluded on the bottom level, on flat
terrain, and on quest levels. Chest traps follow the same rule (flags restricted
to the chest group). Debug placement SHALL set a type unconditionally. An empty
chest on a low level SHALL be marked known right away.

#### Scenario: Trapdoor exclusion

- **WHEN** placement on the dungeon's bottom level draws a trapdoor
- **THEN** that draw is voided and redrawn

- **Anchors**: `src/traps.c:1997-2057` (floor), `:2065-2101` (chest),
 `:2104-2112` (debug)

### Requirement: Monster Trap Setting

Players holding the Trapping ability (prerequisite Disarming at 15) or the
lay-trap power SHALL be able to set monster traps on clean floor, with four
gate conditions: not blind (sight), not in darkness (lighting), not confused
(a clear head), and a clean floor grid; first a trap kit is chosen
(bow / crossbow / slingshot / potion / scroll / device, six classes), then ammo
per the kit class; the ammo count is bounded by the extra shots flag (adding the
kit's pval), the auto-99 flag, and the held quantity; the ammo goes into the
special slot and the kit into the extra-special slot underfoot, the floor
becomes a monster trap, and 100 energy is spent.

#### Scenario: Auto loading

- **WHEN** the kit carries the auto-99 flag
- **THEN** the ammo cap is taken straight from 99 (still bounded by the held
 quantity)

- **Anchors**: `src/traps.c:2148-2293`

### Requirement: Monster Trap Triggering

A monster stepping on a monster trap SHALL be judged in order: a wall-passing
monster is immune unless the trap carries the ghost-killing flag; a restricting
target flag (dragon / demon / undead / evil / animal) that does not match
prevents triggering. The perception check uses base difficulty 25 (the hidden flag
adds ten times the pval) against monster smartness (level, smart adds ten,
trap-noticed adds twenty, stupid or empty-minded drops to -150); once noticed the
monster is marked and enters the disarm check (trap armor class against a fifth of
the level, doubled for smart monsters, stupid monsters and non-smart animals drop
out); a successful disarm removes the trap and announces it to a player who can
see. On triggering, execution follows the kit class: shooting kits use the bow
x3 / crossbow x4 / slingshot x2 multipliers (plus extra-power bonuses) through the
hit, special damage, and critical flow step by step, explosive ammo changes the
burst damage per class, ammo is consumed and thrown back or shattered; potion and
scroll kits project effects by their sval table (including experience gain, the
life and death potions rebounding on undead, genocide / greater genocide and
more); device kits fire by rod stored mana (timeout per cost)
or wand/staff charges (the wand effect table is currently disabled wholesale —
every wand sval sits inside an `#if 0` block, so a wand kit spends charges for
no effect). When the
teleport-to-player flag is set the monster is teleported next to the player
afterwards; a non-automatic trap is disarmed by triggering (a
`TRAP2_AUTOMATIC_5` one is removed with one chance in five — it survives four
in five — while `TRAP2_AUTOMATIC_99` never removes), and after disarming the
terrain reverts to floor.

#### Scenario: Wall-passing immunity

- **WHEN** a wall-passing monster steps on a monster trap without the
 ghost-killing flag
- **THEN** the trap does not trigger at all

- **Anchors**: `src/traps.c:2934-3129` (immunity, perception, and disarm),
 `:3106-3403` (the four kit executions), `:3414-3425` (teleport-to-player and
 terrain revert), effect table `:2300-2928`
