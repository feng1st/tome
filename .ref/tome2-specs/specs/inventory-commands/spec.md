# inventory-commands Specification

## Purpose

Inventory and equipment commands: list viewing, wearing and taking off, dropping
and destroying, inspecting and inscribing, refueling, targeting and panel looking,
symbol lookup and monster research, grid mana sensing, the portable hole, and the
command line interface. The underlying object holding / merging / sorting is
documented under `src/cmd6.c` (usage commands) and `src/object1.c`.

## Requirements

### Requirement: List Viewing

The inventory and equipment lists SHALL show empty slots and indicate the burden as
a percentage of capacity; a non-Escape key SHALL enter the "show held items" mode,
keeping the item list on screen under subsequent command screens.

- **Anchors**: `src/cmd3.c:19-171`

### Requirement: Wearing Protocol

Wearing SHALL pass a wearability filter: only one `TR4_ULTIMATE` item may be worn
across the whole body; the target slot must exist in the body plan, and an item
bound for a weapon hand is refused unless the melee style is weapon mastery
(`SKILL_MASTERY`); a cursed item on the
target slot blocks the swap; an identifiably cursed item asks for confirmation
before wearing; the wear hook can veto; and the two-handed constraint applies — a
must-use-two-hands weapon and a shield exclude each other, while pairing a shield
with a two-hands-capable weapon asks for confirmation. An existing item SHALL be
taken off first (the takeoff hook can veto the removal), quiver slots merge same
ammo, and the action costs 100 energy. After wearing, the game SHALL announce a
per-slot verb, sense and mark a cursed item immediately, re-judge the artifact set,
and trigger the bonus / torch / hitpoint / mana recalculation.

#### Scenario: Cursed slot refusal

- **WHEN** the item already on the target slot is cursed
- **THEN** the wearing is refused with a description of that cursed item

- **Anchors**: `src/cmd3.c:177-212` (wear filter), `src/cmd3.c:231-501`
 (wearing protocol)

### Requirement: Taking Off And Dropping

Taking off SHALL refuse cursed items (wizard mode exempt), cost 50 energy, and let
the takeoff hook veto. Dropping SHALL refuse cursed equipment and, among carried
items, cursed ones bearing `TR4_CURSE_NO_DROP`, take an optional quantity, cost 50
energy, and let the drop hook veto.

#### Scenario: Cursed takeoff refusal

- **WHEN** the player tries to take off a cursed item outside wizard mode
- **THEN** the removal is refused with the cursed-item message and no energy is
 spent

- **Anchors**: `src/cmd3.c:508-637`

### Requirement: Destroying

Destroying SHALL cost no energy; a quantity prefix (the command argument) skips the
confirmation, and worthless items skip it under the auto-destroy option;
`TR4_CURSE_NO_DROP` items cannot be destroyed; artifacts and randarts cannot be
destroyed — instead they receive the "special / terrible" sensing marks. Destroying
a blessed item SHALL anger Eru (`GOD_ERU`), deducting piety at ten times the item
kind's level; partially destroying a wand stack SHALL reduce its charges
proportionally; and creating an automatizer rule with the auto-pickup editor SHALL
offer a destroy rule alongside.

#### Scenario: Artifacts resist destruction

- **WHEN** the player attempts to destroy an artifact
- **THEN** the destruction fails, the item gains a sensing mark, and its category
 is announced

- **Discrepancy:** the comment above the partial-stack reduction speaks of "rods
 or wand", but the code tests only `TV_WAND` (`src/cmd3.c:782-790`).
- **Anchors**: `src/cmd3.c:643-811`

### Requirement: Inspecting And Inscribing

Inspecting SHALL output the item's full description (stating plainly when there is
nothing special). Inscribing SHALL keep inscriptions in the quark pool
(defaulting to the previous inscription as the base text); removing an inscription
SHALL clear it with a message; both SHALL trigger the pack combine and the window
refresh.

- **Anchors**: `src/cmd3.c:817-970`

### Requirement: Refueling

Refueling SHALL branch on the light source: torches merge (fuel plus five, capped at
5000, with an over-fuel message when already full); lanterns take oil (a flask adds
its pval, another lantern adds its fuel time, capped at 15000, with the full
message); non-fuel light sources are refused. The action costs 50 energy and
recalculates the torch radius.

#### Scenario: Torch merge cap

- **WHEN** a merge would push the fuel to the cap
- **THEN** the fuel is clamped to 5000 and the full message plays

- **Anchors**: `src/cmd3.c:977-1208`

### Requirement: Targeting And Panel Look

The target command SHALL enter lock-on mode, and the look command SHALL enter
inspect mode (the target command announcing success or failure; the look command
announcing a success). The panel look SHALL
allow roaming the map panes with the direction keys while reporting the relative
direction live; on exit the view recenters and a full refresh runs.

#### Scenario: Target selection

- **WHEN** the player picks a target with the target command, or aborts it
- **THEN** "Target Selected." or "Target Aborted." is reported accordingly

- **Anchors**: `src/cmd3.c:1214-1344`

### Requirement: Symbol Lookup And Research

Symbol lookup SHALL support four bulk filters (all / uniques only / non-uniques
only / name substring); the recall prompt orders the hits by player kills (k) or
level (p), or keeps the race-index order (y), listing
only monsters seen at least once (cheat omniscience lifts that), with the full
recall text toggleable at any time while browsing. Building research SHALL share the
mechanics but force omniscience (cheat_know is forced on for the scan and restored
afterward, and the inspected race's seen flag is faked to once), letting the player
pay to inspect any monster.

- **Anchors**: `src/cmd3.c:1624-1871` (lookup), `src/cmd3.c:1878-2092`
 (research)

### Requirement: Grid Mana Sensing

Sensing the grid mana SHALL cost 200 energy: the success chance derives from the
magic device skill (halved while confused, reduced by one tenth of the grid's mana,
with a floor), and on failure the input buffer flushes and a message plays; on
success the current grid mana is announced in coarse ranges scaled by the skill
(wizard mode additionally shows the exact value).

#### Scenario: Wizard sensing shows the raw value

- **WHEN** the grid mana sensing succeeds while wizard mode is on
- **THEN** the exact grid mana and the averaged value are both reported

- **Anchors**: `src/cmd3.c:2098-2144`

### Requirement: Portable Hole

The portable hole's weight SHALL dynamically equal one and a half times the total
weight of the random-town home's contents plus two, synchronized across every
portable hole in the shop stocks, on the floor, and in the player's pack. Using it
SHALL verify the tool slot is worn, then temporarily disguise the current grid as a
house store and enter the interaction; on exit the terrain is restored and the
weight recalculated.

#### Scenario: Weight coupling

- **WHEN** an item is stored in the home
- **THEN** every portable hole's weight updates to one and a half times plus two

- **Anchors**: `src/cmd3.c:2150-2281`

### Requirement: Command Line Interface

The CLI SHALL keep a command alias table (name to command code lookup, the
description inherited from the previous entry); input offers completion, and a hit
dispatches the command directly by its code; help SHALL build a temporary listing
to display. The HTML dump SHALL save the current screen as a web page or help file.

- **Anchors**: `src/cmd3.c:2287-2531`
