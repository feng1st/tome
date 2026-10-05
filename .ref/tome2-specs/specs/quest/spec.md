# quest Specification

## Purpose

Quests and plots: `src/plots.c` compiles all 25 quest implementations across 23
`q_*.c` files by textual inclusion, grouped into plot lines — the random quests, the main line
(necro/one/sauron/morgoth/ultrag/ultrae), the Bree line, the Lorien line, the
Gondolin line, the Minas Anor line, the Khazad-dum line, and the others
(Narsil/Thrain). This capability carries the two shared facilities (quest
description display and the null hook) plus the behavior of every plot line. The
quest table structure is established at startup from the Lua module parameters and
`reinit_quests` — see specs/boot-loading/spec.md.

## Requirements

### Requirement: Quest Description Display

A quest SHALL support displaying its description: the description array is printed
line by line (at most ten lines, terminated by an empty line).

#### Scenario: Line-by-line display

- **WHEN** a quest with a three-line description is displayed
- **THEN** the three lines are output in order as yellow messages

- **Anchors**: `src/plots.c:419-427`

### Requirement: Null Hook

The quest system SHALL provide an always-false null hook function
(`quest_null_hook`), which plot implementations use as a "no operation" callback
placeholder.

#### Scenario: Placeholder

- **WHEN** a plot mounts the null hook
- **THEN** the hook returns false when triggered, without interrupting the hook chain

- **Anchors**: `src/plots.c:430-434`

### Requirement: Plot Line Grouping

The 25 quest implementations (23 included `q_*.c` files) SHALL be compiled into the
unit grouped by seven plot lines: the random quests, the six main-line quests, the six Bree quests, the three
Lorien quests, the four Gondolin quests, the two Minas Anor quests, the one
Khazad-dum quest plus two others; each line exports init hooks that `init_hooks`
loads. Each quest's behavior is specified in the requirements below.

#### Scenario: Compile inclusion

- **WHEN** the engine is built
- **THEN** all q_*.c files compile as part of the plots.c translation unit, with no
 object files of their own

- **Anchors**: `src/plots.c:436-473` (include list), `src/plots.h:1-47` (the init
 hooks exported by each line)

### Requirement: Main Line Chain

The main line SHALL progress through three tiers — the Necromancer (race 819), then
Sauron (860), then Morgoth (862): the presence of a higher-tier monster SHALL block
a lower-tier monster from entering a level (the new-monster entry hook intercepts by
race number). The Necromancer's death SHALL reveal him as Sauron's spirit, complete
that quest, and turn the plot pointer to the One Ring quest. Sauron's death SHALL
complete its quest, take the Morgoth quest, and display its description. Morgoth's
death SHALL decide victory (the endgame winner flag and the title refresh) and branch
the following plot lines on whether the One Ring quest's ring is destroyed —
destroyed enters the ultragood line, otherwise the ultrabad line; the two lines show
different congratulations, then the death hook is unloaded and the hook iteration
restarted.

#### Scenario: Chained interception

- **WHEN** Sauron tries to enter a level while the Necromancer is alive
- **THEN** the entry is intercepted; Morgoth's interception of Sauron works the same way

#### Scenario: Endgame branch

- **WHEN** Morgoth dies
- **THEN** the plot pointer turns to the ultragood or ultrabad line according to the
 One Ring state, and that line's quest initialization runs at once

- **Anchors**: `src/q_main.c:1-19` (interception), `src/q_main.c:146-176`
 (Necromancer), `src/q_main.c:92-144` (Sauron), `src/q_main.c:20-90` (Morgoth and
 the branch)

### Requirement: Nazgul Revival Clause

The Nazgul of Mordor SHALL not be permanently destroyable while Sauron exists: after
a Nazgul dies, its race availability SHALL reset to 1 with a "not totally destroyed"
message. When Sauron himself is destroyed while the One Ring quest is unfinished, his
race availability SHALL likewise reset to 1, with a message that the One Ring must be
destroyed or wielded first.

#### Scenario: Nazgul reset

- **WHEN** a Nazgul is destroyed while Sauron still exists
- **THEN** that Nazgul's race availability resets to 1 and it can appear again

- **Anchors**: `src/q_main.c:116-133`

### Requirement: One Ring Acquisition

After the main quest is taken, the One Ring SHALL be planted into the world by
wonder: when Angband depth 99 generates, "Sauron, the Sorcerer" is placed on open
floor. When one of the named uniques dies and the One Ring does not yet exist in the
world — Sauron at 30%, or Ar-Pharazon the Golden / Shelob, Spider of Darkness / The
Watcher in the Water / Glaurung, Father of the Dragons / Feagwath, the Undead
Sorcerer at 10% each — a plain gold ring is marked as the One Ring (`ART_POWER`),
magic is applied, and it is stuffed into the backpack — when the backpack is full a
message plays and the last item is dropped to make room, followed by the compulsion
message about picking up the plain ring.

#### Scenario: Backpack full

- **WHEN** the One Ring drops while the backpack has no free slot
- **THEN** the last backpack item is dropped automatically and the compulsion message
 plays

- **Anchors**: `src/q_one.c:225-306` (planting), `src/q_one.c:319-344` (depth 99
 placement)

### Requirement: One Ring Use And Price

Wearing the One Ring SHALL pass a triple escalating confirmation. After wearing: the
four good gods (Eru/Manwe/Tulkas/Yavanna) all abandon the character; the five towns
are marked destroyed; the quest ends as "defeated" and the plot turns to the Sauron
line; the life count is zeroed. From then on the max-hp calculation SHALL multiply
the max-hp value by two-thirds, applied (lives + 1) times; death SHALL be postponed
by the Ring (while max hp is greater than 1, a message says the shadows sustain the
character) until max hp reaches 1, where the character dies with "being drawn to the
shadow world" as the cause.

#### Scenario: Wearing price

- **WHEN** the player confirms the triple prompt and wears the One Ring
- **THEN** the good gods abandon, the towns are destroyed, the quest turns defeated,
 and each later death extends life at the price of the max-hp reduction

- **Anchors**: `src/q_one.c:100-156` (wearing), `src/q_one.c:157-173` (life
 reduction), `src/q_one.c:174-192` (death postponement)

### Requirement: One Ring Destruction

The One Ring quest SHALL start through a movement-triggered dialog (the
Galadriel's mirror store tile, store tile special 23; requires the Necromancer to be
dead; an extra warning
is added according to the worshipped god). Dropping the One Ring on a
`FEAT_GREAT_FIRE` tile SHALL destroy the ring (its count zeroed), abandon Melkor,
complete the quest, and turn the plot to the Sauron line (Sauron's death becomes
permanent). When an unidentified One Ring is identified, an extra message SHALL be
added.

#### Scenario: Great Fire destruction

- **WHEN** the One Ring is dropped on a Great Fire tile
- **THEN** the ring is destroyed, Melkor abandons, the quest completes, and the
 Sauron line restarts with Sauron permanently destroyable

- **Anchors**: `src/q_one.c:4-68` (dialog), `src/q_one.c:69-99` (destruction),
 `src/q_one.c:193-224` (identification message)

### Requirement: Character Record Lines

The main line and the One Ring quest SHALL append ending lines to the character
dump: after Morgoth's quest completes, "You saved Arda and became a famed %s" or
"You became a new force of darkness and enslaved all free people" is written
according to the Ring's state; the One Ring writes a destroyed or a fell record
according to completion or defeat.

#### Scenario: Ending line

- **WHEN** a character dump is generated and Morgoth's quest is complete
- **THEN** the matching ending line is written according to the One Ring state

- **Anchors**: `src/q_main.c:70-80`, `src/q_one.c:307-318`

### Requirement: Thieves' Hideout

The thieves quest SHALL build its level from the dedicated map (`thieves.map`) as a
permanent-wall cage with the player inside: on entry a blow-to-the-head message
plays, and all possessions are dropped onto a fixed tile in a loop (cursed equipment
excepted) until nothing is left to drop; the dungeon flag is set to the no-genocide
state. While active, the cage doors SHALL be checked every turn: a door opened or
broken triggers the alarm — every monster in the level is angered, the eight door
tiles burst open, and the breached door becomes floor. When the hostile count reaches zero,
the up staircase SHALL be revealed and the quest turns completed. The entry
feeling SHALL announce waking up in a cell, robbed of possessions. The final dialog
SHALL reward the hideout as a home and branch by the ratio of the fight and magic
skill values (fifty-fifty when equal or on a one-in-ten roll) to the trolls or the
wights quest, which then initializes at once.

#### Scenario: Entry disarm

- **WHEN** the thieves quest level generates
- **THEN** every non-cursed possession is dropped on the fixed tile inside the cage,
 while cursed equipment stays worn

- **Anchors**: `src/q_thief.c:4-63` (generation and disarm), `:64-115` (alarm and
 clearing), `:116-147` (branch), `:149-160` (entry feeling)

### Requirement: Hobbit Rescue

The rescue quest SHALL roll the maze dungeon's target depth at initialization
(26 to 34) and record the starting turn. Bree wilderness generation SHALL place
Melinda on open floor out of the player's sight where no store stands (the special
restriction temporarily lifted) while the quest is unfinished — through the
completed state, since her reward dialog is what finishes it — and after a ten-day
cooldown; the target depth SHALL place friendly Merton. Giving a town-portal scroll
to Merton SHALL make him read it and go home (the monster is deleted, the scroll
consumed) and turn the quest completed. Talking to Melinda SHALL advance by stage:
while untaken, the plea statement plays and the quest is taken; when completed, she
thanks and rewards a rod of recall (identified, origin marked as quest reward),
deletes Melinda, and completes the quest; after completion only thanks remain. While
the quest is unfinished, talking to Melinda SHALL replace the default speech with
the plea for help. The character dump SHALL gain the saved-hobbit line.

#### Scenario: Cooldown

- **WHEN** fewer than ten days have passed since the last record
- **THEN** Melinda does not appear and the town event is absent

- **Anchors**: `src/q_hobbit.c:169-195` (initialization and depth roll), `:4-35`
 (town placement), `:36-62` (in-level placement), `:63-92` (giving the scroll),
 `:93-160` (dialog progression and reward), `:161-168` (record)

### Requirement: Stone Troll Lair

The troll quest SHALL build from the dedicated map (`trolls.map`), place Tom the
stone troll on a marked tile (the special restriction temporarily lifted), and hook
the mystery blade Glamdring into his carry pile — forged on a broad sword base with
magic applied, the artifact restriction temporarily lifted; when the object list is
full (`o_pop` fails), the artifact counter is rolled back instead. While active, monster
deaths SHALL be watched: when Tom dies, the up staircase is revealed at (3,3) and
the quest completes; the first death of any other monster triggers the ambush — the
map's special marked tiles are re-floored to grass and one forest or stone troll
(each at fifty percent) is placed on each, and the ambush fires only once. The final
dialog SHALL give thanks and let the player keep the loot, and the plot turns to the
nazgul quest, which then initializes at once.

#### Scenario: One-time ambush

- **WHEN** another monster dies after the ambush has already fired
- **THEN** no further ambush is generated

- **Anchors**: `src/q_troll.c:4-95` (generation and the artifact carry), `:116-170`
 (death listening and ambush), `:96-115` (completion and turn)

### Requirement: Barrow-wight King

The barrow-wight quest SHALL build from the dedicated map (`wights.map`), place the
barrow-wight king on a marked tile (the special restriction temporarily lifted), and
hook a "of the Wight" ragged cloth armor into his carry pile (a fixed flag set:
intelligence and searching, resistance to blindness and confusion, sensitivity to
fire, indestructibility by the four elements, and seeing invisibility, permanently
cursed and already identified; the
pval is rolled at half — one half adds spell chance 6, the other adds mana 2). The
king's death SHALL reveal the up staircase under the player and complete the
quest; the final dialog SHALL give thanks and let the player keep the loot, and the
plot turns to the nazgul quest, which then initializes at once.

#### Scenario: King dies, door opens

- **WHEN** the barrow-wight king is destroyed
- **THEN** the player's tile turns into the up staircase

- **Anchors**: `src/q_wight.c:4-103` (generation and the cloth armor), `:104-127`
 (death completion), `:128-146` (completion and turn)

### Requirement: Nazgul Uvatha

The nazgul quest SHALL gate its acceptance: a character below level 30 is refused
with a come-back-later message. After acceptance, Bree generation SHALL place
Uvatha on open floor out of the player's sight (the special restriction temporarily
lifted); Uvatha's death completes the quest. The final dialog SHALL give thanks and
reward six athelas plants (identified, store-identification marked, origin marked as
quest reward), after which the plot pointer is cleared — the Bree line ends there.
The character dump SHALL gain the saved-Bree line.

#### Scenario: Gate refusal

- **WHEN** a player below level 30 tries to take the nazgul quest
- **THEN** acceptance is refused and the quest keeps its former state

- **Anchors**: `src/q_nazgul.c:73-86` (gate), `:4-34` (town placement), `:87-103`
 (death completion), `:35-64` (reward and line end), `:65-72` (record)

### Requirement: Farmer's Mushroom Garden

The mushroom quest SHALL roll the mushroom quota at initialization (7 to 14).
Wilderness generation at the fixed coordinates (21,33) SHALL lay out a 15 x 11 fungus
field, scatter marked mushrooms up to the uncollected quota (food kind, mark 1), and
place the farmer's three dogs — Grip, Wolf, Fang (the special restriction
temporarily lifted, hostile) — with barking messages. The farmer SHALL appear in
Bree only in daylight, from six o'clock to eighteen (at night the placement is
skipped), on open floor out of sight.
Delivery SHALL distinguish: if any dog has died, the quest turns "ended in defeat" —
the murderer is rebuked, the farmer vanishes, and all hooks are removed; delivering
marked mushrooms consumes and counts them, and at quota the reward is 15 to 20
curing mushrooms (with the free discount) plus the farmer's sling (fixed artifact
number 149, magic applied), the farmer vanishes and the quest completes, otherwise
the remaining count is reported. While the quest is unfinished, talking to the
farmer or his speaking SHALL first check dog survival: a dead dog turns the same
defeat path; while untaken the plea plays first (with the in-game help trigger
attached), while taken the remaining count is reported.

#### Scenario: Dog killed, quest lost

- **WHEN** any of the three dogs dies and the farmer is then interacted with
- **THEN** the quest turns defeated, the farmer rebukes and vanishes, and all hooks
 are removed

- **Anchors**: `src/q_shroom.c:266-291` (initialization), `:6-80` (field and farmer
 generation), `:81-98` (dog death message), `:99-183` (delivery and reward),
 `:184-265` (dialog and defeat)

### Requirement: Lorien Wolves

The wolves quest SHALL build from the dedicated map (`wolves.map`) with the
no-genocide generation flag; entering without talking to the mayor auto-takes the
quest. Placement SHALL roll 4d4 twice — once for the wolves (race 196), once for
the wargs (race 257) — and place that many of each, confined to non-permanent floor
tiles inside the map area; every monster is placed hostile (MSTATUS_ENEMY), with a
fifty percent chance of starting asleep. The monster-death listener
SHALL complete the quest when the hostile count (hostile and more hostile stances)
is not more than one. The final dialog SHALL reward the hut as a home and turn the
plot pointer to the spiders quest (without immediate initialization).

#### Scenario: Enter untalked

- **WHEN** the player enters the wolves level without taking the quest
- **THEN** the quest auto-turns taken and generation proceeds normally

- **Anchors**: `src/q_wolves.c:4-68` (generation and placement), `:70-100`
 (clearing completion), `:102-117` (completion and turn)

### Requirement: Mirkwood Spiders

The spiders quest SHALL build from the dedicated map (`spiders.map`) with no extra
placement; the monster-death listener SHALL complete the quest when the hostile
count is not more than one, and a Yavanna worshipper SHALL additionally gain 6000
piety with a smile message. The final dialog SHALL reward a potion (identified,
store-identification marked, origin marked as quest reward) — a `SV_POTION_AUGMENTATION`
— and the plot turns to the poison quest with immediate initialization.

#### Scenario: Forest guardianship

- **WHEN** the spiders quest is cleared and the player worships Yavanna
- **THEN** completion also grants 6000 Yavanna piety

- **Anchors**: `src/q_spider.c:4-34` (generation), `:35-70` (clearing and piety),
 `:71-100` (reward and turn)

### Requirement: Poisoned Waters

The poison quest SHALL pick one of four predefined wilderness coordinates at
initialization as the pollution point. Wilderness generation at the pollution point
SHALL pick open floor as the center: water within a 25-tile radius turns tainted
water at 80% probability, and mold-family monsters (the `create_molds_hook` allows
d_char `m`, `,`, `e` and rejects breeders) are placed within a 10-tile radius at 60%
probability, drawn at level 30 with an 80% chance of experience raising them to no
more than the player's level. Taking the quest SHALL put 99 curing potions in the
backpack (identified, inscribed "quest"). Dropping a curing potion at the pollution
point SHALL check the neutral-and-worse monster count: at ten or more the cleansing
is refused with a message, otherwise all tainted water in the level returns to
shallow water and the quest completes. The final dialog SHALL reward blue dragon
scale mail (carrying the `EGO_ELVENKIND` ego, magic applied), the plot pointer is
cleared — the Lorien line ends there. The character dump SHALL gain the
saved-the-mallorns line.

#### Scenario: Monsters block the cleansing

- **WHEN** neutral and hostile monsters total ten or more when a curing potion is
 dropped
- **THEN** the potion is not consumed and the water stays tainted

- **Anchors**: `src/q_poison.c:215-237` (initialization and site pick), `:12-22`
 (generation hook), `:24-99` (tainting and monster leveling), `:162-214` (potion
 cleansing), `:100-130` (reward and line end), `:131-138` (record)

### Requirement: Gondolin Dragon Lair

The dragon quest SHALL build from the dedicated map (`dragons.map`) with the
no-genocide generation flag; entering untalked auto-takes it. Generation SHALL first
place 35 mountain pillars (only on even-flag floor tiles, keeping the map
connected), then 25 dragons: each of the eight color families draws from its four
age tables — baby/young/mature/ancient — by percentile: a roll of zero picks
ancient, under 33 baby, under 66 young, otherwise mature, each hostile with a
one-third chance of being placed asleep. The hostile count not more than one completes the quest.
The final dialog SHALL reward a cave as a home, and the plot pointer turns to the
Eol quest (without immediate initialization).

#### Scenario: Pillar connectivity

- **WHEN** mountain pillar sites are chosen
- **THEN** only floor tiles whose flag word is even are allowed, keeping the whole
 map connected

- **Anchors**: `src/q_dragons.c:4-89` (pillars and dragons), `:91-121` (clearing),
 `:123-138` (completion and turn)

### Requirement: Dark Elf Eol

The Eol quest SHALL build a procedural cave (a 50 x 30 height-map fractal with random
roughness and cutoff parameters): scanning from the map's deep end, the first clean
tile places Eol (the special restriction temporarily lifted), clean tiles are
trapped at 18%, and the player is set at the scan end with an up staircase laid.
Eol's death SHALL complete the quest (with a lament message). The staircase listener
SHALL pass only this quest's staircase tiles: while Eol lives, escaping upward
requires confirmation, and confirming fails the quest; once Eol is dead, passage is
allowed. The quest-failed event SHALL end with "You fled" and clear the plot
pointer — the Gondolin line is severed there. The final dialog SHALL reward a dwarven
lantern (a light ego added, magic applied), and the plot turns to the Nirnaeth quest
with immediate initialization.

- **Quirk:** the fractal gradient parameter is actually computed by a bitwise XOR
 with `2^randint(4)`, yielding 3/0/1/6 — recorded as-is.

#### Scenario: Fleeing severs the line

- **WHEN** the player confirms the upward escape while Eol lives
- **THEN** the quest fails and the Gondolin line ends (the plot pointer cleared)

- **Anchors**: `src/q_eol.c:4-82` (procedural cave, monsters, traps), `:133-152`
 (death completion), `:153-182` (staircase listening), `:115-132` (failure and
 severance), `:83-114` (reward and turn)

### Requirement: Battle of Unnumbered Tears

The Nirnaeth quest SHALL build from the dedicated map (`nirnaeth.map`) and count all
monsters in the level at generation as the extermination base. While active, deaths
SHALL be counted one by one; stepping on this quest's staircase tile finds the way
out and completes the quest (leaving is allowed at any time). The wrap-up SHALL grade
the reward by kills: at two thirds of the base or more, 200000 gold pieces and
access to the royal jeweler store; below that, access only; afterwards the plot
pointer is cleared — the Gondolin line ends.

#### Scenario: Kill grading

- **WHEN** the quest wraps up with kills at two thirds of the base or more
- **THEN** the gold and the jeweler access are both granted; below the mark only the
 access

- **Anchors**: `src/q_nirna.c:4-43` (generation and counting), `:81-100` (staircase
 completion), `:44-80` (wrap-up grading and line end)

### Requirement: Fall of Gondolin

The defense SHALL be recruited through a global per-turn hook: while the quest is
untaken, the character is level 45 or more, not in a quest, and not astral-walking,
the Thunder Lord Liron appears and asks — accepting teleports to Gondolin (town 2,
coordinates 117,24) and takes the quest; refusing fails the quest, marks Gondolin
destroyed, and removes the recruitment. The quest level SHALL build from the
dedicated map (`maeglin.map`), with the marked tile set as the player start and the
up staircase, and recorded as Maeglin's target point. Maeglin's (race 825) AI
SHALL be taken over: within two tiles of the player he acts normally, otherwise he
is forced toward the target point; on arrival he is deleted — Gondolin falls, the
quest fails, and Gondolin is marked destroyed. Maeglin's death SHALL complete the
quest. The staircase listener SHALL pass only this quest's staircase: upward branches
by quest state — failed plays the ruins message, completed has Turgon grant a
150-point max-hp bonus and formally complete, in-progress asks for confirmation to
flee (quest fails, Gondolin destroyed); downward is always blocked. The character
dump SHALL write "You abandoned Gondolin" or "You saved Gondolin" by ending.

#### Scenario: Maeglin slips away

- **WHEN** Maeglin's hijacked AI reaches the target point
- **THEN** Maeglin vanishes, Gondolin falls, and the quest turns failed

- **Anchors**: `src/q_invas.c:87-145` (recruitment and teleport), `:4-47`
 (generation and target point), `:48-86` (AI takeover and the fall), `:158-177`
 (death completion), `:178-220` (staircase branch and the life blessing),
 `:221-233` (initialization), `:146-157` (record)

### Requirement: Haunted House of Minas Anor

The haunted quest SHALL build from the dedicated map (`haunted.map`) with the
no-genocide generation flag; entering untalked auto-takes it. Placement SHALL set 12
ghosts (race 477), a 4d4-rolled number of haunt monsters drawn randomly from a
22-race list, and 10 plus 4d4 traps. The hostile count not more than one completes
the quest. The final dialog SHALL reward a building as a home, and the plot pointer
turns to the between-gates quest (without immediate initialization).

#### Scenario: Three placements

- **WHEN** the haunted level generates
- **THEN** ghosts, haunt monsters, and traps roll their positions independently

- **Anchors**: `src/q_haunted.c:4-85` (generation and the three placements),
 `:87-117` (clearing), `:119-134` (completion and turn)

### Requirement: Between Gates Link

The between quest SHALL gate acceptance at level 45. After acceptance it SHALL
advance through a movement listener: stepping into Turgon's tower in Gondolin (store
tile special 27) turns the quest completed with an instruction to return; in the
wilderness at vertical axis 19 or less a one-time ambush triggers (a data flag
marks it as already triggered) — the player is forced into the dedicated level
(`between.map`,
no-genocide generation flag, the entry energy drain zeroed to prevent instant death).
Inside, when the neutral-and-worse monster count is below two, an up staircase
SHALL open under the player as the way out. The final dialog SHALL reward the Golden
Horn of the Thunder Lords (forged with the special-item restriction temporarily
lifted, magic applied, free discount) and grant free passage through the between
gates; the plot pointer is cleared — the Minas line ends there. The character dump
SHALL gain the connected-the-two-cities line.

#### Scenario: One-time ambush

- **WHEN** the player moves in the northern wilderness and the ambush has not fired
 before
- **THEN** the player is pulled into the ambush level, and it never triggers again

- **Anchors**: `src/q_betwen.c:178-190` (initialization and gate), `:4-51`
 (movement listening and ambush), `:52-87` (level generation), `:126-154` (exit
 staircase), `:88-125` (reward and line end), `:155-163` (record)

### Requirement: Khazad-dum Balrogs

The balrog quest SHALL build from the dedicated map (`evil.map`) with the
no-genocide generation flag; entering untalked auto-takes it; six balrogs (race 996)
are placed. When the hostile count is not more than one, the quest SHALL complete
directly (skipping the mayor's reward dialog; a source note marks it as still to be
written) and clear the plot pointer — the Khazad-dum line ends there. The final
dialog SHALL reward a cave as a home and end the plot line the same way.

#### Scenario: Direct completion

- **WHEN** the six balrogs are cleared to a hostile count of one or less
- **THEN** the quest completes directly, the plot line ends, and no mayor reward
 dialog exists

- **Anchors**: `src/q_evil.c:4-54` (generation and placement), `:56-88` (clearing
 and direct completion), `:90-105` (final dialog)

### Requirement: Sword Reforging

The Narsil quest SHALL trigger on identification: identifying the broken sword
(`ART_NARSIL`) while untaken turns the quest taken, shows its description, and swaps
in the movement listener. Carrying Narsil into Aragorn's castle (store tile special
14) SHALL trigger the reforging dialog: the first inventory slot holding the broken
sword (the scan covers equipment and backpack) is reforged in place into Anduril
(`ART_ANDURIL`, forged on a long sword base with magic applied
and identified) and the quest completes. The character dump SHALL gain the reforging
line. This is a side quest and does not advance the plot pointer.

#### Scenario: In-place reforging

- **WHEN** the broken sword sits in an inventory slot and the player enters the castle
- **THEN** that slot's item is replaced in place by Anduril

- **Anchors**: `src/q_narsil.c:109-118` (initialization and acceptance), `:67-108`
 (identification trigger), `:4-58` (reforging), `:59-66` (record)

### Requirement: Thrain Rescue

The Thrain quest SHALL roll its target depth at initialization within Dol Guldur's
range (min depth + 1 to max depth - 1). Entering that level hears the tortured cries
for help and takes the quest. Room generation SHALL embed a dedicated secret chamber
into the normal room generation flow through a building hook (`thrain.map`: take the
size, allocate the space, fill random floor, wall the outer ring): the marked tile
places neutral Thrain ("Thrain, the King Under the Mountain", the special
restriction temporarily lifted), and the two Dol Guldur nazgul — "Dwar, Dog Lord of
Waw" and "Hoarmurath of Dir" — carry the quest flag; one chamber per level at most.
The chamber SHALL hide behind illusion walls (illusion number 61): stepping on an
illusion tile reveals every illusion wall in the level. The two nazgul SHALL be
counted: on the second death the illusion walls all reveal and the quest completes —
Thrain's dying words reveal the Necromancer as Sauron (bridging into the main line),
he is deleted, the glass walls around him are flattened to floor, and the
random-artifact dragon helm inscribed "of Thrain" drops at his position. Level
transitions SHALL reset that level's generation and kill counters.

#### Scenario: Illusion chamber

- **WHEN** the player steps on a chamber wall tile illusored as plain terrain
- **THEN** every illusion wall in the level reveals and the chamber becomes visible

- **Anchors**: `src/q_thrain.c:226-246` (initialization and depth roll),
 `:177-188` (level-entry acceptance), `:91-176` (chamber generation and quest
 flags), `:189-219` (illusion reveal), `:4-89` (nazgul counting and dying words)

### Requirement: Random Quest Placement

Shallow levels of the main dungeons (main flag on) SHALL host random quests: each
level can hold one target race and one kill count (type), in two classes — the
sword-recovery class (kill counts drawn from the fixed list
20/13/15/16/9/17/18/8) and the princess class; the target race number is stored in
the level's random quest slot. The entry feeling SHALL announce by class: sword
class, a man "wrapped in a dark cloak" pleads for help ("A horrible %s stole my
sword"); princess class, a shout of "Leave me alone, stupid %s". Level generation SHALL place by class: the sword class places
kill-count many target monsters at random tiles when the level finishes generating
(with the quest flag, the special restriction temporarily lifted); the princess
class embeds the numbered room map (`qrand<N>.map`) through a building hook
(two-pass: take the size and allocate the space, then place for real, with the
quest-flagged target monster on the marked tile) and raises the level's feeling
rating. A new or rebuilt level SHALL reset the kill counter and the room-built flag.

#### Scenario: Class split

- **WHEN** a random quest level generates
- **THEN** the sword class spreads the target monsters over the whole level, while
 the princess class concentrates them inside the embedded room

- **Anchors**: `src/q_rand.c:1-18` (class list and test), `:251-268` (entry
 feeling), `:269-298` (sword-class placement), `:299-387` (princess-class room),
 `:245-250` (counter reset on new or rebuilt level), `:432-442` (hook assembly)

### Requirement: Random Quest Clearing And Reward

Target monster deaths SHALL count only when the quest flag is set; reaching the kill
count branches the reward by class. Sword class: an adventurer appears and gives
thanks — while a companion slot is open the player may invite them (companion
stance, experience scaled by depth; the landing tile avoids glyphs of warding,
between gates, and pattern tiles, and after twenty failed finds the adventurer flees
in fright), while a refusal or a full roster has them touch the character's forehead
and grant skill points (through the Lua `do_get_new_skill`). Princess class: the
princess (race 969) thanks and goes home — the glass walls around her are flattened
and a down staircase opens; three quality items are offered and the player picks one
(origin marked as reward), and each unchosen traditional artifact SHALL have its
generation right returned (the randart mark reset, the no-artifact-generation mark
cleared, the normal artifact counts zeroed). Both classes record completion into the
level's random quest slot completion bits.

#### Scenario: Unchosen artifact returned

- **WHEN** the player declines a traditional artifact in the pick-one-of-three
- **THEN** that artifact's generated count is zeroed and it can appear again later

- **Anchors**: `src/q_rand.c:216-244` (counting gate), `:139-214` (sword-class
 reward and joining), `:97-137` (princess-class reward), `:20-95` (pick-one and
 artifact return)

### Requirement: Random Quest Record

The character dump SHALL sum random quest completions by class: the princess class
and the sword class are counted separately, the amounts rendered as English number
words (2 to 10) or as digits, with a "never completed" sentence at zero; the section
is written only when random-quest levels exist.

#### Scenario: Class sums

- **WHEN** a character dump is generated and random-quest levels exist
- **THEN** the two class completion counts each get a line, with the number word
 chosen by the amount

- **Quirk:** the sword line's number word (and its over-ten digit) misuses the
 princess counter index — a source defect recorded as-is.
- **Anchors**: `src/q_rand.c:388-431` (record)
