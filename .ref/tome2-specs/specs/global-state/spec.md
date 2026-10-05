# global-state Specification

## Purpose

Global state: the engine-level global variables and default values defined in
`src/variable.c` — run markers, commands and energy, depth and turn, the five option
groups, random seeds, the cave/object/monster tables with the light and view scratch
arrays, all word-list arrays with their capacity caps, the player structure and the
per-level HP table, directory paths, function hook pointers, module and plot/fate/bounty
state, the persistent effect table and global timers. Variable semantics live in the
requirements of the capabilities that use them; this capability registers only the
inventory and the initial values.

## Requirements

### Requirement: Version And Run Markers

The version triples (one set for the executable, one for the savefile), the savefile
metadata (system, time, past lives, save count), and the seven command-line argument flags
(wizard/sound/graphics/dual keysets/big tiles/fiddle mode) SHALL be engine-level globals.
So SHALL the four character-state markers (generated/loaded/in a level/saved), the two
screen-sticky markers, and the three random seeds (object flavor/town layout/persistent
dungeon).

- **Anchors**: `src/variable.c:38-69` (version and arguments), `:75-85` (character state and seeds)

### Requirement: Command And Depth State

The current command variables (command, argument, repeat, direction, display), the energy
spent this turn, the four auto-stair markers, the running and resting counters, the current
level dimensions and depth plus the old depth, the breeding counter, the object and monster
generation levels, the turn counter, the win/lose markers, panic save and accumulated
cheating, the coin type and chest lock, the shimmer and repair optimization markers, the
object/monster counts and caps, and the possession and symbiosis miscellany SHALL be
globals. The destined-death dungeon cleanup logic keys off the monster count reaching zero.

- **Anchors**: `src/variable.c:87-179`

### Requirement: Five Option Groups

The software options SHALL be five groups of booleans: interface (dual keysets/quick
messages/pickup confirmation/old targeting/metric units/stack merging/label weights/color
display), disturbance (running past stairs and doors and corners, the six trivial
disturbance probes for monster movement and proximity and panel and detection and state,
critical HP warning, dying words, unique quotes, small and empty levels, the old monster
AI, auto-destroy, staircase and cursed-wear confirmation, pet disturbance), game-play
(auto-haggling, level scumming, stacking permissions, list expansion, the two map-memory
options, monsters carrying light, dungeon alignment and stair connectivity, sound scent and
flow, follow and target, smart learning and cheating), efficiency (running light reduction,
reduced town view, no-interrupt/no-shimmer/no-special-color suppressions, the three flushes
for disturbance/failure and before-messages, savefile compression, player highlight, torch
yellow/distant light reduction and the two wall-ground effect options), plus testing
(stack test/carry test) and the six cheat flags (peek objects/peek monsters/peek dungeon/
peek miscellany/omniscience/escape death). Also the hitpoint warning tier, the delay
factor, and the three autosave parameters.

- **Anchors**: `src/variable.c:220-365`

### Requirement: Depth And Tracking

The level feeling and rating, the on-level artifact markers, the closing marker, the eight
panel parameters, the inside/outside wall features and the hundred-entry floor fill table,
the three target parameters (monster index/column/row), the health bar tracking, the two
monster-memory tracking indexes, and the object tracking pointer SHALL be globals.

- **Anchors**: `src/variable.c:368-418`

### Requirement: The Three Tables And Light Arrays

The two-dimensional cave table, the object list, the monster list, and the kept-monster
list SHALL be heap-allocated arrays; the player light grid list (cap 1536), the view grid
list (same cap), and the temporary grid list (16384) each carry a count. Object indexes
start at 1, and so do monster indexes.

- **Anchors**: `src/variable.c:456-475` (light/view/temp), `:710-725` (three tables)

### Requirement: Word-List Arrays And Capacities

The seventeen word-list families SHALL each hold a header plus data pointers plus a name
pool and text pool: alchemy (with recipes and optional flags), vault, terrain, object,
artifact, item set, title, randart part (with the thirty-row generation table), trap,
monster, monster title, dungeon, ability, skill, player race/subrace/class (with meta
class), wilderness block, and store/action/owner; the dungeon table is declared twice
(the same object in practice). The capacity caps SHALL be carried by the max_* globals
driven by misc.txt (monster/object/artifact/title/trap/wilderness/store/action/owner/the
four player tables/skill/ability/dungeon/item set/alchemy/vault/terrain), plus the
wilderness dimensions, the allocation table size, and the valid marker.

#### Scenario: Capacity driving

- **WHEN** misc.txt changes a word-list capacity
- **THEN** the matching max_* global holds the new value before loading, and the word-list
 parser rejects out-of-capacity entries against it

- **Anchors**: `src/variable.c:821-1002` (word-list arrays), `:1214-1330` (capacity caps)

### Requirement: Player And Global Tables

The player structure is a static entity accessed through a pointer; the five table pointers
(sex/race/subrace/class/spec) are set at birth; the per-level HP table SHALL be saved
separately so savefile rollback cannot farm hit points; the alchemy known titles/artifact
bitmaps and the gained count; the four global skill arrays (the generic skills); seven plot
slots, the ninety-nine-level random quest table, two hundred fate slots, twenty-four bounty
pairs, the random spell and rune spell tables, one hundred twenty-eight persistent effect
slots, the global timer chain, the CLI alias table, the corruption table capacity, the
module name and version triple, the level cap and the death dungeon (default 28), and the
god table (default six gods).

#### Scenario: Per-level HP table

- **WHEN** a level-up rolls a hit-point gain
- **THEN** the result is stored in the per-level HP table; experience draining and
 restoration operate only on the current value and never touch this table

- **Anchors**: `src/variable.c:790-828` (player and HP table), `:1352-1628` (bounty/fate/effect/module/god)

### Requirement: Directory Paths

The engine SHALL maintain the main directory plus eighteen subdirectory paths (high
scores/bones/core scripts/level definitions/compiled data/word-lists/miscellaneous
text/help/previews/modules/patches/notes/savefiles/scripts/default preferences/user
preferences/extra resources/recordings) for the loaders to fetch files by category.

- **Anchors**: `src/variable.c:1004-1138`

### Requirement: Function Pointers And Hook Slots

The object selection filters (show-all switch, category restriction, callback hook), the
generic sort compare and swap, the two monster draw hooks, and the object draw hook SHALL
be swappable function-pointer globals; the six action-letter ignore table, the four macro
tables, the word-list pools, the eight message ring-buffer pieces, the option and window
bitmask arrays, the terminal pointer array and name table, the color table, and the sound
name table SHALL be in place.

- **Anchors**: `src/variable.c:1149-1207` (hook pointers), `:479-578` (macro/word-list pools/messages/options/terminal), `:600-702` (colors and sounds)
