# cave-lighting Specification

## Purpose

Cave runtime: line-of-sight checks, the field of view (the octant slope
bit-vector algorithm), monster lighting, grid memory, the three-layer rendering,
the minimap, magic mapping, monster flow, terrain changes, and assorted helpers.
The terrain word-list and wilderness blocks are specified in
`specs/terrain/spec.md` and `specs/wilderness-terrain/spec.md`; the flag
semantics used by rendering are recorded in the grid entry of
`specs/core-data/spec.md`.

## Requirements

### Requirement: Distance and Line of Sight

Distance SHALL be computed by the approximation (long axis plus half the short
axis). Line of sight SHALL advance along the long axis with the integer-slope
algorithm (start and end grids exempt), short-circuiting first on orthogonal
straight lines cell by cell and on the knight-step special cases. Sight blockage
SHALL be judged by the raw grid feature's `FF1_NO_VISION` flag alone
(`cave_sight_bold`) — mimic terrain plays no role there. The separate
magic-mapping/clairvoyance wall classification (`is_wall`) SHALL take the mimic
feature first; grids below `FEAT_SECRET` (vanilla floors and doors) are not
walls, a glass wall is not counted as a wall (it carries WALL but no NO_VISION —
see-through), an illusion wall and a small tree are counted as walls (both carry
NO_VISION while not carrying WALL), and the rest follow the terrain `FF1_WALL`
flag. Sight need not be symmetric. Projectile reachability SHALL be
judged separately by step-by-step stepping (different criterion from sight; the
arrival check precedes the wall-passing check there).

#### Scenario: Illusion wall

- **WHEN** a sight line crosses an illusion wall grid
- **THEN** the sight line is blocked (`NO_VISION`), and magic mapping counts the
 grid as a wall (`is_wall` exception 2)

- **Anchors**: `src/cave.c:24-38` (distance), `:45-71` (is_wall — the
 mapping/clairvoyance wall test and its three exceptions), `:109-329` (line of
 sight), `src/defines.h:3714-3715` (cave_sight_bold — FF1_NO_VISION test),
 `lib/edit/f_info.txt:753-766` (glass and illusion wall flags), `:828-831`
 (small tree flags), `src/cave.c:5011-5040` (projectable)

### Requirement: Grid Flag Semantics

Each grid information bit SHALL have its own role: room and vault marks
(savefile), memory (savefile), glow, wall (blocks sight, maintained across terrain
changes), in view, visible and lit, the temporary reuse bit, player torch
lighting, monster lighting. Visible implies in view; each algorithm clears the
temporary bit after use.

- **Anchors**: `src/cave.c:2892-2965` (long comment on each flag's semantics)

### Requirement: Field of View Algorithm

The field of view SHALL be driven by the precomputed octant tables: at startup,
for grids within radius twenty (161 grids), the corner slopes are collected,
deduplicated, and sorted (126 slopes), and a doubly linked parent chain plus slope
bit-masks are built and verified for completeness. At run time the update has
three steps: previously viewed grids have their visible state saved and their
flags cleared; the player grid goes first; each octant then advances along the
parent chain breadth-first — a grid is visible if and only if its slope bit-vector
intersects the set of unblocked slopes; a wall grid clears its own slope bits and
is only marked visible (visible by torch distance, monster lighting, or glow plus a
simplified neighbor-lighting test toward the player), while non-wall grids keep
propagating to children and are marked visible by torch distance or glow/monster
lighting. Going blind SHALL clear all visible lighting. Newly visible grids SHALL
be memorized and redrawn; previously visible grids that lose lighting are redrawn.

#### Scenario: Wall blocks propagation

- **WHEN** a wall appears across some slope channel
- **THEN** that slope is marked blocked and grids thereafter lit only through that
 slope are no longer visible

- **Anchors**: `src/cave.c:3444-3674` (octant table construction), `:3789-4121`
 (the three-step algorithm)

### Requirement: Monster Lighting

Light-carrying monsters SHALL light the grids of their 3x3 neighborhood that lie
inside the player's view (marking monster lighting and visibility); when the
monster is not visible, wall grids receive none of its light. A blind player SHALL
have all monster lighting cleared. When lighting is withdrawn, the affected
monsters' visibility and the grid displays SHALL be rechecked.

#### Scenario: Unseen light carrier

- **WHEN** a light-carrying monster is outside the player's sight
- **THEN** its light does not cross wall grids while the remaining neighbors are
 lit as usual

- **Anchors**: `src/cave.c:4192-4423`

### Requirement: Memory Registration

Grid memory SHALL have two tracks: terrain memory requires the grid to be
visible — plain floors are memorized per the options (memorize lit floors,
memorize torch-lit floors) or when carrying a trap, and all other terrain is
always memorized; object memory is independent of terrain (while the grid is
visible, all objects on it and a mimicking monster's disguise are marked seen).

#### Scenario: Floor memory options

- **WHEN** a lit floor grid is seen with both memory options off
- **THEN** that grid's terrain is not memorized

- **Anchors**: `src/cave.c:2122-2184`

### Requirement: Three-Layer Rendering

Grid rendering SHALL layer bottom-up: the terrain layer (drawn only when
memorized or visible; mimics replace terrain; building grids use the store
appearance; branch staircases are purple; detected traps use the trap color —
with a color-changing trap blending depth, dungeon, and terrain into the color,
and an unidentified trap on open floor using a dedicated symbol; persistent
effects and multicolored terrain shift color per frame; the lighting options apply
torch-yellow / darkness / blindness-gray / distance-dim tiers separately to floor
grids and to wall-and-door grids); the object layer (the first marked object is
shown on top, multicolored items cycle color, hallucination randomizes); the
monster layer (a mimicking monster shows as its carried disguise; visible monsters
follow the race appearance — multicolored ones cycle between the colors of a
breath table or take fully random colors, changers are random, transparent
symbols/colors step aside item by item, hallucination randomizes); the player
layer covers last (mimic race appearance, optional health-percentage character,
invisibility needs to be pierced). Under hallucination any single grid has a
one-in-256 chance of being randomized wholesale.

#### Scenario: Multicolored breathing color

- **WHEN** rendering a multicolored monster with a single breath type
- **THEN** its color cycles between the two colors of that breath; with several
 breaths each contributes its first color to the set

- **Anchors**: `src/cave.c:879-1569` (main rendering; monster color tables
 `:551-674`), `:1576-2020` (the no-graphics default version)

### Requirement: Minimap

The full-map overview SHALL be drawn compressed by priority: with the lighting
options temporarily disabled, every grid's render value is taken and placed onto
the scaled canvas by the terrain priority table (dark 2, floor 5, wall 10, door
17, staircase 25, and so on, with unlisted terrain at 20); the player always takes
the highest priority; the scale factor is multiplied by ten near one-to-one to
preserve detail; a frame is drawn around the canvas and a cursor marks the player.

- **Anchors**: `src/cave.c:2416-2517` (priority table and function), `:2533-2741`
 (compressed drawing), `:2749-2789` (interaction entry)

### Requirement: Magic Mapping and Clairvoyance

Magic mapping SHALL cover the current panel plus a random margin: non-wall grids
are memorized, and wall grids adjacent to them are memorized too. Clairvoyance
SHALL mark all floor objects (mimic disguises included) and set glow plus memory
over the whole map (floor memory per the options). Forgetting SHALL remove all
terrain memory and object marks.

- **Anchors**: `src/cave.c:4617-4674` (mapping), `:4694-4789` (the two clairvoyance
 tiers), `:4794-4834` (forgetting)

### Requirement: Monster Flow

The monster tracking flow SHALL flood breadth-first from the player as the source,
marking each grid's step cost (compile-time switch; active when MONSTER_FLOW is
built and sound flow is enabled); the cost stamp cycles for reuse once per 128
flow updates; walls and rubble do not conduct, and the flow depth is capped.
Forgetting SHALL zero the whole map.

- **Anchors**: `src/cave.c:4444-4476` (forgetting), `:4491-4606` (flood)

### Requirement: Terrain Change and Misc Helpers

Terrain changes SHALL sync the wall flag and, while the dungeon is active,
memorize and redraw; floor and fill laying SHALL each pick terrain at random from
their hundred-entry tables. Disturb SHALL cancel a repeated command, resting,
running (recomputing the torch), and optionally the searching state, and flush
input per the settings. The quest-level check SHALL return the player's current
quest first, then the principal dungeon's unfinished random quest level whose
quest type is not one of the hero-quest types in `randquest_hero[]`. The
effect table SHALL allocate persistent effects by slot. The valid-destroy check
SHALL exclude permanent grids and grids holding artifacts. Tracking SHALL maintain
the health bar and the monster recall and object windows.

- **Anchors**: `src/cave.c:4843-4900` (terrain change and laying), `:5145-5198`
 (disturb), `:5204-5236` (quest level), `:5242-5266` (effect table), `:348-375`
 (valid destroy), `:5096-5132` (tracking)
