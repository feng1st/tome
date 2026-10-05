# monster-generation Specification

## Purpose

Monster generation and runtime registration: monster experience level-ups, monster
ego selection and synthesis, the monster life-table registration (deletion,
compaction, recycling), per-level drawing (dungeon rule filtering and difficulty
raising), the placement protocol (some twenty checks plus individualization), groups
and escorts, the summon family, breeding mutation, target visibility updates and lore
accumulation, and smart learning. The name and description protocol and habitat
filtering are specified in specs/monster-memory/spec.md; factions and control are
specified in specs/monster-faction/spec.md.

## Requirements

### Requirement: Monster Level-Up

A monster SHALL level up from experience (capped at level 150). Each level gained
rolls its growth items independently:

- 80% chance: max hp and current hp each gain one die side;
- 40% chance: speed and current speed each gain a random 1 to 2;
- 50% chance: armor gains one tenth of the monster's own base armor (at least 1);
- 30% chance: one random blow group gains one damage die.

A visible level-up is announced. A forced level set SHALL run through the same
experience channel and can only raise the level, never lower it.

- **Quirk:** the blow-group growth roll is `rand_int(3)` (indices 0-2) with a 20-try
 rescan — blow group index 3 (the fourth group) can never be picked for growth.

#### Scenario: Item-by-item growth

- **WHEN** a monster crosses the next level's experience threshold
- **THEN** every growth item rolls independently, and no item is guaranteed to
 improve

- **Anchors**: `src/monster2.c:20-69`, experience entry `src/monster2.c:71-101`

### Requirement: Ego Selection And Synthesis

A normal monster SHALL gain a monster ego by probability: the candidate must pass the
four filters of the ego word-list (required flags, excluded flags, symbol list,
exclusion list), with separate roll gates for depth overshoot and rarity; elf/dwarf
themed dungeons instead assign the themed ego directly (still subject to the
filters). Ego synthesis SHALL run on a copied race: the four blow dice and sides are
run through their modifier operators, hit dice / armor / sleepiness / speed / weight
/ experience / level are combined by their own operators, the casting frequency takes
the larger of the two, flags are removed before they are added, and symbol and color
are rewritten when not wildcard.

#### Scenario: Themed direct assignment

- **WHEN** an elf-themed dungeon generates a resident
- **THEN** the probability draw is skipped and the elf ego is applied directly

- **Anchors**: `src/monster2.c:126-225` (filtering and draw),
 `src/monster2.c:231-317` (synthesis)

### Requirement: Life-Table Registration

Monster deletion SHALL clean up: the race's on-level count, the breeding count, the
player's target / tracking / control references, other monsters' target references,
the floor-grid reference, carried objects (with the preserve option on, an
unidentified artifact is returned to generation), and the mind/special-race
structures. When the fated dungeon (`DUNGEON_DEATH`) empties, the game announces
"You overcome your fate, mortal!" and returns the player to the wilderness (dungeon
type switched, depth zeroed, level left). A full table SHALL recycle dead slots, and
if that is still not enough, announce the overflow and fail. Compaction SHALL relax
its criteria round by round (the level ceiling rises as `5 x round` and the distance
shield shrinks as `5 x (20 - round)`): quest monsters are exempt unless the round
count reaches the thousand-round emergency, uniques are nearly exempt (a 99% save),
all others get a 90% save roll; afterwards dead slots are filled from the tail and
`m_max` is compressed. A level-exit cleanup SHALL wipe the whole table and reset the
counters.

#### Scenario: Fated dungeon emptied

- **WHEN** the last monster of a fated dungeon is deleted
- **THEN** the defy-fate message plays and the player is returned to the wilderness

- **Anchors**: `src/monster2.c:403-525` (deletion), `src/monster2.c:549-712`
 (compaction), `src/monster2.c:720-770` (level-exit cleanup),
 `src/monster2.c:778-824` (recycling)

### Requirement: Per-Level Drawing

Monster drawing SHALL pass two filter stages — the general hook and the per-grid
secondary hook both rebuild the allocation table — followed by a cumulative
probability draw at the target level. The draw gates SHALL include: unique stock and
cap, forced depth may not overshoot the depth, depth-only races must match the depth
exactly, module flag filtering (core ToME excludes foreign-race and Cthulhu
monsters), the funny-monster switch, and dungeon rules (a rule is picked randomly
from the per-hundred allocation table; AND/NAND require all flags and the symbol to
match, OR/NOR any one; the N forms negate; the summon bypass can skip this). The
level SHALL get two small-probability "nastify" raises. After the draw, a 60% roll
draws a second time and keeps the stronger monster, and a 10% roll draws a third
time.

- **Discrepancy:** the source comment claims the second draw happens "(50%)", but the
 code rolls `p < 60`.

#### Scenario: Keeping the stronger

- **WHEN** the second draw lands on a monster weaker than the first draw
- **THEN** the first, stronger monster is kept

- **Anchors**: `src/monster2.c:832-860` (hook recompute), `src/monster2.c:866-975`
 (dungeon rules), `src/monster2.c:1002-1173` (draw and raises)

### Requirement: Placement Protocol

Placement SHALL run a chain of about twenty checks in order: no placement in the
shrunken wilderness, in bounds, empty grid, floor grids need special permission,
glyph of warding / explosive rune / BETWEEN gate / altar / pattern tiles refuse
placement, valid race index, entry hook passes, uniques take no ego, walkable
terrain, special generation needs permission, spirits limited to the Void, the
never-generate flag, unique uniqueness (including the -1 special cap and the bypass
flag), the savefile-level unique marker, the on-level count, and the forced depth.
An over-depth placement SHALL raise the level rating (uniques count the depth
difference double). Individualization SHALL include: the stance defaults and then
floats by the race's pet/neutral flags; forced sleep rolls double the sleepiness plus a `randint(10 x sleepiness)` roll;
carried object generation (object level takes the mean
of dungeon depth and monster level; drop flags roll 60% / 90% / one to four groups
of d2 cumulatively; a mimic monster carries exactly one item; gold and items split
half-half; a force-drop-randart monster forcibly seeks a randart-capable base item
within a thousand tries and enchants it through the creation flow); force-maxhp
takes the hit dice maximum; the four blow groups and attributes are copied;
non-uniques get a small speed nudge; the dungeon level modifier flags (follow
player / half / single / double) drive a forced level set; a random initial energy;
the force-sleep marker for a gentle entry; visibility update; and the breeding and
on-level counters increment.

#### Scenario: Force-drop randart

- **WHEN** a monster with the force-drop-randart flag is placed
- **THEN** within a thousand tries a randart-capable base item is forcibly drawn and
 generated through the random artifact flow

- **Anchors**: `src/monster2.c:2133-2340` (check chain), `src/monster2.c:2342-2386`
 (rating and registration), `src/monster2.c:2388-2444` (initialization and sleep),
 `src/monster2.c:2446-2590` (carried objects), `src/monster2.c:2593-2652` (copying
 and level set), `src/monster2.c:2654-2703` (finishing)

### Requirement: Groups And Escorts

A group-flagged monster SHALL form a group with a base size of 1d13, adjusted by the
depth difference (stronger monsters get smaller groups, weaker ones larger, the
adjustment capped at twelve) and a total cap of 32, placed breadth-first outward
around the first placement. An escort-flagged monster SHALL recruit up to fifty
escort placements: escorts must share the dungeon flags and the symbol, be of level
no higher than the leader, and not be unique; they scatter within three grids and
pass the secondary terrain filter; when the escort itself has the group flag or the
leader has the multi-escort flag, the escorts also come in groups.

#### Scenario: Escort filtering

- **WHEN** escorts are recruited for a unique leader
- **THEN** the candidates are limited to lower-level non-unique monsters of the same
 symbol

- **Anchors**: `src/monster2.c:2714-2798` (groups), `src/monster2.c:2853-2939`
 (escorts), filtering `src/monster2.c:2809-2832`

### Requirement: Summon Family

A summon SHALL pick a grid within twenty tries, spiraling outward from the summon
source (glyph of warding, BETWEEN gate, and pattern tiles refuse; stepping onto a
BETWEEN gate fails the whole summon), filtered through the summon filter of roughly
thirty-five types (symbol families, flag families, name-substring families, Lua
callbacks) before the draw, at a level of the mean of dungeon depth and summoner
level plus five. Hostile summons default to group-capable (the Dawn and the blue
horrors excepted); friendly summons are fixed to pet stance and can disable
grouping. A preset spell level can force a level set after the summon.

#### Scenario: Hostile and friendly stances

- **WHEN** the same summon type runs once through the hostile entry and once through
 the friendly entry
- **THEN** the monster kind is identical and only the stance differs, enemy versus
 pet

- **Anchors**: `src/monster2.c:3113-3401` (type filter table),
 `src/monster2.c:3429-3514` (hostile summons), `src/monster2.c:3518-3591`
 (friendly summons)

### Requirement: Breeding And Mutation

Breeding SHALL look for an empty adjacent grid within eighteen tries; there is a 3%
probability (clones excepted) of mutating into a stronger race of the same symbol at
no lower than the dungeon depth; with the global no-breed switch on, the attempt
fails with the "tries to breed but it fails" message. Clone offspring carry the clone
mark. Teleport exchange SHALL support swapping coordinates between a monster and the
player (the player as a negative index), refreshing both sides' visibility.

#### Scenario: Breed switch

- **WHEN** breeding is forbidden by the global switch
- **THEN** the breeding attempt fails and the effort message plays

- **Anchors**: `src/monster2.c:3722-3788` (breeding and mutation),
 `src/monster2.c:3597-3695` (exchange)

### Requirement: Visibility Update And Lore

Each monster's visibility SHALL be decided by combining: line of sight within the
panel without blindness, infravision range for non-cold-blooded races, lit
visibility (and either not invisible or the player can see invisibility), telepathy
matched class-by-class against the fourteen ESP flags (empty mind blocks all, alien
mind catches 10% at random, normal mind passes all), and the detection marker.
Turning from unseen to visible counts a first sighting and disturbs per the option.
Telepathy contact memorizes the smart/stupid flags; infravision and lit-visibility
failures memorize cold-blooded and invisible respectively. A terror monster turning
visible SHALL trigger a sanity blast: power is level plus ten (groups halve
it, uniques double it), then the exemption ladder judges in order — confusing
visions, intelligence/wisdom drain, brain smash (paralysis plus chained drain),
permanent drain, and total amnesia; undead and vampire characters get extra saves;
under hallucination the text becomes comic. Drops SHALL record the maximum counts of
gold/item drops and permanently memorize the good and great flags; detection SHALL
memorize the first three flag words.

#### Scenario: Infravision versus cold blood

- **WHEN** a cold-blooded monster stands within infravision range
- **THEN** infravision does not apply, only the lit path can make it visible, and
 its cold-blooded nature is memorized

- **Anchors**: `src/monster2.c:1725-2002` (visibility), `src/monster2.c:1529-1669`
 (sanity blast), `src/monster2.c:1505-1525` (drop lore), `src/monster2.c:1472-1489`
 (detection)

### Requirement: Smart Learning

A smart monster SHALL observe and memorize the player's resistances, immunities, and
ineffective-against states when casting specific effects at the player (the four
elements in three tiers of resistance / temporary opposition / immunity, every other
family recorded singly, with free action, no-mana, and reflection as single flags).
With the learning option off, for stupid monsters, and for non-smart monsters on
half their turns, learning is skipped.

#### Scenario: Stupid monsters do not learn

- **WHEN** a stupid monster observes the player's fire resistance
- **THEN** no player resistance is recorded

- **Anchors**: `src/monster2.c:3933-4042`

### Requirement: Pain Messages And Placement

Pain messages SHALL use seven tiers by the hp percentage after damage, with wording
in four families by symbol (jelly/fungus, canine, small noisy, and the rest); zero
damage announces that the monster is unharmed. Whole-level monster allocation SHALL
search within ten thousand tries for an empty grid far enough from the player.
A placement that occupies the player's grid is expressed as a negative index and
records the coordinates.

#### Scenario: Percentage wording

- **WHEN** a monster's remaining hp after damage is below one tenth
- **THEN** the near-death wording of its symbol family plays

- **Anchors**: `src/monster2.c:3821-3926` (pain), `src/monster2.c:3047-3099`
 (allocation), `src/monster2.c:4048-4059` (player grid)
