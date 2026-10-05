# player-movement Specification

## Purpose

Player movement and interaction: `src/cmd1.c` carries all rulings of a one-step move -
enterable-grid judgment, random movement and ice sliding, wilderness boundary crossing,
friendly-monster swapping and attack dispatch, the energy rules for unknown/known
obstacles, pattern road walking order, detection-area entry and exit, stepping effects
(search/pickup/stores/traps/grid inscriptions), the running algorithm and pet commands,
possession leaving and embodiment; `src/cmd2.c` carries staircases and inter-level
movement, door and chest opening/closing, tunnel digging, trap disarming, door bashing and
spiking, the generic alter key, walk/run/stay/rest, phase-door steps and the immovable
specials, sacrificing, stealing, giving and chatting. The terrain passable-flag
definitions are specified in specs/terrain/spec.md; the pickup core `py_pickup_floor` is
specified with the object1.c material; shooting is specified in specs/player-ranged/spec.md.

## Requirements

### Requirement: Enterable Grid Judgment

`player_can_enter` SHALL rule:

- wraith form or half-ghost grants wall passing;
- while wearing the wall (CLASS_WALL), the player may only stay inside walls and is
 barred from open floor;
- in wild_mode (the small-scale wilderness map), a player carrying at least half the
 weight limit without feather fall may not enter deep water, and those without fire
 resistance/immunity/opposition or feather fall may not enter shallow or deep lava;
- tree squares permit: flying, wall passing, the tree-walking power, the Ent mimic form,
 or praying to Yavanna with grace of at least 9000;
- CAN_CLIMB permits climbing, CAN_FLY or CAN_LEVITATE permits flying, CAN_LEVITATE
 permits feather fall, CAN_PASS permits wall passing;
- NO_WALK bars entry outright;
- spider webs bar everything except spider races and spider mimicry.

#### Scenario: Web entry

- **WHEN** a character whose body race lacks `RF7_SPIDER` and without the Spider mimic
 form moves into a square with the `FF1_WEB` flag
- **THEN** `player_can_enter` refuses the entry

- **Anchors**: `src/cmd1.c:3200-3271`

### Requirement: One-Step Move

`move_player_aux` SHALL first rule the direction: a disembodied player uses the
input direction directly; a body_monster with the random-move flags re-rolls a
random direction by the two-flag combination chance (75%) or each single flag's
own chance; standing on ice
without feather fall or flight slides with a (70-level)% chance into a changed direction.

The target grid SHALL be ruled in order: a friendly monster on an enterable grid, while
the player is neither confused, stunned, nor hallucinating and the monster is visible -
the encounter wakes the friendly monster, then the wielded random artifact named
`'Stormbringer'` lands a probability strike (a 1000-side roll above 666), otherwise the
positions swap when the square underfoot is standable floor or the monster passes walls
(with a "You push past" message), otherwise the blocked way costs zero energy and the
move is cancelled; an enemy monster, or any monster visible or on an enterable grid,
gets `py_attack`; a chasm (FEAT_DARK_PIT) without feather fall is barred and stops
running; with easy disarm on, `do_cmd_disarm_aux` runs automatically; a detected trap
bars stepping while the player is neither confused, stunned, nor hallucinating (zero
energy); an unenterable grid - probability travel switches to passwall, the
feel message plays by known-or-not and the terrain is remembered (with easy tunnel on,
`do_cmd_tunnel_aux` runs automatically; in confusion/stun/hallucination a wall charge
still costs energy, otherwise zero); a pattern road sequence violation costs zero energy
and alerts.

The actual move SHALL: bar roots; run the HOOK_MOVED hook; redraw the old and new grids
with panel checking; announce detection-area entry and exit; update view/flow/monster
distances; describe the new grid's terrain (a down staircase adds the dungeon branch
name); run the spontaneous search (fires once the search skill has passed its half, else
by a 50-minus-search roll) and the continuous search; pick up; pre-stage the enter-store
command on a store door; announce altars; discover hidden traps and trigger `hit_trap`;
display the grid inscription and execute INSCRIP_EXEC_WALK.

Each step in wild_mode (the small-scale wilderness map) SHALL reveal the surroundings and set did_nothing;
stepping out of the local block SHALL shift the wilderness coordinates by the boundary
direction and set leaving (the ambush mark is cleared).

#### Scenario: Ice sliding

- **WHEN** the player walks onto ice on foot without feather fall or flight
- **THEN** with a 70-minus-level percent chance the move turns into a random-direction
 move and the sliding message plays

- **Anchors**: `src/cmd1.c:3282-3880` (main flow), random direction `:3301-3339`,
 boundary crossing `:3359-3439`

### Requirement: Pattern Road Order

Pattern road walking SHALL rule by segment order: stepping onto the start square from a
square outside the road requires confirmation (confusion/stun/hallucination exempt); the
finish, old and corrupted segments may only be entered from in-road squares; the teleport
segment is free both ways; from the start square only in-road squares may be entered; the
finish, old and corrupted segments may only be left toward in-road squares (stepping off
the road is barred); the normal segments cycle 1->2->3->4->1 or
the move stops in place, with a message on a violation.

#### Scenario: Out-of-order segment

- **WHEN** the player standing on pattern segment 2 steps toward segment 4 while
 clear-headed
- **THEN** the straight-road order message plays and the step costs zero energy

- **Anchors**: `src/cmd1.c:3079-3196`, `src/cmd1.c:3652-3662` (violation handling)

### Requirement: Trap Triggering And Search

Stepping on a hidden trap SHALL first reveal it (pick_trap selects the concrete trap) and
then trigger `hit_trap` (the trap effect fires; a successful identification marks the trap
identified in `t_info` and announces its name).

`search` SHALL roll against the nine grids of the 3x3 neighborhood at a base rate of the
search skill
(divided by ten when blind, without light, or in confusion/hallucination): finding hidden
traps, seeing through secret doors (replacing the head with a normal door), and finding
chest traps (the whole chest is identified).

Object auras (touch_zap_player) SHALL, when the player stands close, roll
`damroll(1 + level/26, 1 + level/17)` for the fire and lightning auras; opposition and
resistance each divide the damage by three (rounded up), fire susceptibility doubles it,
and immunity negates it entirely (the monster memory records the aura flag when the aura
fires).

#### Scenario: Aura retaliation

- **WHEN** the player stands close to a monster with `RF2_AURA_FIRE` and lacks
 `immune_fire`
- **THEN** the level-folded roll damages the player, reduced by opposition/resistance and
 doubled by fire susceptibility

- **Anchors**: `src/cmd1.c:525-614` (search), `:636-658` (trigger), `:661-714` (auras)

### Requirement: Running Algorithm

Running SHALL maintain its path state with the direction ring table: after the start
checks known obstacles (tree squares excepted), run_init - from the side and diagonal-side
obstacles it derives the left/right openings and the open-area state, and corrects the
forward direction at diagonal corridors and obtuse corridor entries; each step runs
run_test - a visible monster or visible object stops the run; remembered terrain is
exempted by class (lava with invulnerability or fire immunity, deep water and ice with
feather fall or flight, open and broken doors by the find_ignore_doors option, staircases
and between gates by the find_ignore_stairs option, terrain with the
DONT_NOTICE_RUNNING flag, detected traps always stop); unknown and floor grids orient by
the one/two-opening logic (openings choose straight or corner-cut by the examine and cut
options); in the open-area state a side opening stops the run; a known obstacle ahead
stops the run. Each step costs 100 energy and moves with the pickup flag.

- **Anchors**: `src/cmd1.c:4112-4677` (state machine), `:4628-4677` (run_step)

### Requirement: Pet Commands

`do_cmd_pet` SHALL be disabled under confusion and prompt when no pet exists. The ten
commands: dismiss pets (per-pet confirm or all; RF7_NO_DEATH pets survive; the pet and
friendly tiers), dismiss companions (COMPANION tier), call to side (follow distance one),
follow (six), seek and destroy (255), open-flag toggle, pickup-flag toggle (when off,
pets drop their carried objects), give a target to all friends, and cancel one friend's
target.

- **Dead code:** the menu also lists "give target to a friend" (a point-selected target
 for one pet), but the command switch implements no handler for it, so choosing the
 entry does nothing.

- **Anchors**: `src/cmd1.c:4722-5162`

### Requirement: Possession Leaving And Embodiment

Leaving the body SHALL be blocked by cursed equipment. When abandoning the body, a corpse
is left with probability 25% + possession skill folded to 25 + preservation skill (the
object records the monster race and current hp, the weight scales by monster weight, a
unique corpse is recorded as name1=201), otherwise the corpse rots; after leaving,
disembodied is set and the player manifests as a Lost soul. Embodiment SHALL be limited to
intact corpses on the floor (SV_CORPSE_CORPSE): the body is recorded as the corpse's race,
health carries over from the corpse's hp, and the corpse is removed while the wraith and
leaving states are cleared.

- **Anchors**: `src/cmd1.c:5167-5277`

### Requirement: Grid Inscriptions

Engraving SHALL take up to forty characters and match against the known inscription
library (unknown inscriptions are rejected), write onto the square underfoot, then execute
immediately by INSCRIP_EXEC_ENGRAVE, costing 300 energy. The inscription execution SHALL
first check the square's mana (silently doing nothing when short) and subtract it
permanently, then apply by class: light/dark lights or darkens the room; storm projects 10d10
lightning at radius three; one friendly `Dwarven Warrior` is summoned at a square
scattered up to three grids away;
chasm turns the square into an abyss (flying monsters skim over, non-unique monsters fall,
artifacts are spared while the rest are destroyed); black fire is a radius-three,
200-point hellfire. The guarding inscription is currently an empty implementation.

- **Anchors**: `src/cmd1.c:5441-5475` (engraving), `:5280-5435` (execution)

### Requirement: Staircases And Level Movement

Going up SHALL: a normal staircase or two-way road takes one level off by the confirm
options (DF2_ASK_LEAVE asks before leaving permanently, confirm_stairs the regular
confirmation); a shaft takes 1d3+1 (the result clamped at the surface, depth 0); a quest
exit records leaving_quest, then jumps to the quest pointed at by c_ptr->special and
clears the depth; probability travel (on non-FLAT levels, disabled by DF2_NO_EASY_MOVE)
refuses at the dungeon's shallowest level (mindepth).

Going down SHALL pass the between gates first (FEAT_BETWEEN swaps to the paired gate by
the low/high eight bits of special and costs 100 energy; FEAT_BETWEEN2 jumps through the
between_exits table to the recorded dungeon depth and wilderness coordinates); the Astral
level 98 bars going down; a TRAP_OF_SINKING trap square allows a deliberate jump down; a
normal staircase adds one level, a shaft adds 1d3 levels (the up direction's 1d3+1 is not
mirrored here; each intermediate level is checked for quest levels and the bottom cap);
when c_ptr->special points at a branch dungeon, the
min_plev threshold gates entry (HOOK_ENTER_DUNGEON may veto); arriving from the entrance
coordinates records the shallowest level, from the exit coordinates the deepest, otherwise
the shallowest. Staircase movement currently costs no energy; going up pre-stages a return
down staircase and going down a return up staircase (jumping excepted). The savefile
option autosave_l fires before the level change.

#### Scenario: Shaft descent

- **WHEN** the player takes a down shaft in mid-dungeon away from the quest levels and
 the bottom
- **THEN** the depth grows by 1-3 levels and a return up shaft is staged

- **Anchors**: `src/cmd2.c:86-256` (up), `:262-310` (between gates), `:315-548` (down)

### Requirement: Doors And Chests

Opening a door SHALL: the possessed form needs RF2_OPEN_DOOR; easy-open auto-aims at a
unique target. Unlocking a locked door SHALL roll on the disarm skill as base (divided by
ten when blind, without light, or in confusion/hallucination) minus four times the lock
level, floored at two; success opens, grants one experience point and triggers the door
trap; a jammed door (HEAD+8 and above) reports being stuck; a plain door opens directly.
Opening a chest requires the open-door flag just the same.

Opening a chest SHALL unlock the same way (minus the chest pval); on opening, `chest_trap`
runs first (the pval is the trap type, fired through player_activate_trap_type and
identifiable) then `chest_death` - the object count is the sval remainder against the
large-chest base times two (pval zero is an empty chest), the generation level is the
absolute pval plus ten, and a small chest drops gold with 75% chance; after opening the
chest is marked empty and the whole chest is identified.

Closing a door SHALL replace the open state with a plain door and trigger the door trap; a
broken door refuses closing. The possessed form needs the open-door flag for both opening
and closing.

- **Anchors**: `src/cmd2.c:656-758` (chest contents and chest traps), `:768-842` (open
 chest), `:1016-1248` (open door), `:1261-1405` (close door)

### Requirement: Tunnel Digging

Digging SHALL first verify a TV_DIGGING tool in the tool slot (sand walls excepted), the
target remembered, not open floor, and TUNNELABLE; PERMANENT refuses digging.

The per-material thresholds SHALL be: tree chopping ten (success restores grass),
granite forty, quartz veins twenty (gold included), lava ten, sand walls and soft veins
five (gold included), rubble zero (restores dungeon floor with a 10% object chance),
secret doors thirty (the same door trap fires and mimicry is cleared), doors thirty.

Success is the digging skill exceeding the threshold plus the matching roll span
(400/1600/800/250/200/1200). While digging there is a 2% chance of a progress message
(below `skill_req` reports hopelessness and stops, below the `skill_req_1pct` one-percent
threshold - about 1.4 times `skill_req`, set to 8 for sandwalls - reports slow progress);
digging through a gold vein yields gold.

- **Anchors**: `src/cmd2.c:1411-1445` (legality), `:1491-1765` (classification),
 `:1777-1849` (entry, doors and stores refuse digging)

### Requirement: Disarming Traps

Disarming SHALL divide the skill by ten when blind, without light, or in
confusion/hallucination. A chest trap SHALL roll the disarm skill minus three times the
trap difficulty; success grants three times the difficulty as experience and inverts the
pval sign to mark the trap disarmed; on failure, with skill above five and a roll above
five, a retry is allowed, otherwise the chest trap fires. Floor traps work the same way
(difficulty not times three); success clears the trap and steps onto the square, failure
also steps on (and triggers). A monster trap (FEAT_MON_TRAP) has no failure state and is
dismantled straight away with the terrain restored. Easy disarm auto-aims at a unique
target.

- **Anchors**: `src/cmd2.c:1982-2048` (chest traps), `:2061-2155` (floor traps),
 `:2161-2167` (monster traps), `:2173-2274` (entry)

### Requirement: Door Bashing And Spikes

Bashing a door SHALL roll the strength table's bash power minus ten times the lock level
against the percentile (floored at one): a broken door is half smashed, half opened, the
door trap fires and the player steps in; with the door unbroken, an agility-table plus
level roll holds the door steady for another try, and a fumble leaves the player off
balance and paralyzed for 2-3 turns. Bashing an altar SHALL refuse (the anger-the-gods
question); bashing a fountain SHALL rule breakage by the bash power minus fifty (floored
at one) against a 200 roll - on
breaking, a radius-two 6d8 water ball bursts and the fountain becomes deep water. Spiking
SHALL add the 0x08 jam bit to a closed-door square and increment the door level (capped at
TAIL), consuming one spike.

- **Anchors**: `src/cmd2.c:2286-2383` (bashing), `:20-80` (altar and fountain),
 `:2623-2689` (spikes)

### Requirement: Generic Alter Key

`do_cmd_alter` SHALL always cost 100 and dispatch by the target grid: a monster is
attacked, a door opened, TUNNELABLE dug, a trap disarmed; an empty square reports the
swing into nothing.

- **Anchors**: `src/cmd2.c:2497-2580`

### Requirement: Walk, Run, Stay And Rest

Walking SHALL switch to the phase-door step (do_cmd_unwalk) for the immovable physique;
each step in wild_mode (the small-scale wilderness map) costs 100 times (MAX_HGT + MAX_WID)
/ 2 in energy, and the wilderness block level minus twice the player level gives the
ambush probability (change_wild_mode switches to the large map, generate_encounter is set,
the player lands centered).

Running SHALL be disabled under confusion; the count comes from the command argument or
1000. Staying SHALL cost 100 and run the spontaneous search, the continuous search and
pickup, pre-staging the enter-store command on a store door. Rest SHALL be refused at
between gates and in the undead form; the duration comes from the command argument
(0-9999, anything above clamped to 9999), `*` stops at full hp and mana, `&` at full
recovery; resting clears the
searching state.

#### Scenario: Rest refused on a between gate

- **WHEN** the player rests while standing on a FEAT_BETWEEN gate
- **THEN** the "too dangerous" message plays and no turn is spent

- **Anchors**: `src/cmd2.c:2692-2767` (walk and ambush), `:2770-2808` (run),
 `:2816-2865` (stay), `:2870-2957` (rest)

### Requirement: Phase-Door Steps And Immovable Specials

The immovable walk SHALL become a phase-door step: a monster at the target is attacked; an
enterable boundary crosses the block; an unstandable target teleports randomly ten
squares; quest exits and staircases fall back to normal movement; in wild_mode (the
small-scale wilderness map) the direction deflects with 15% chance; on open floor it is
`teleport_player_directed(10, dir)`; wilderness energy costs five times the half-screen
sum.

The immovable specials SHALL maintain immov_cntr: past a count of one, a confirmation
subtracts half the count from mana or health (refused when neither suffices); the menu
offers the fixed-point teleport (point selection), the telekinetic grab (a direction,
distance fifteen times the level, with auto pickup), and rise fifty feet / sink fifty feet
(arena quest levels and NO_EASY_MOVE refuse, capped at the level bounds); any action adds
101 minus twice the level to the count and costs 100 energy.

- **Anchors**: `src/cmd2.c:4317-4494` (phase step), `:4497-4713` (special actions)

### Requirement: Sacrificing

Standing on an altar SHALL: a godless player hears the altar god's sermon and may convert
(grace is recorded as -200); a believer sacrifices to the own god - Melkor allows
self-sacrifice of ten hp while max and current hp both exceed ten (choosing piety
subtracts ten from hp_mod and grants `wisdom_scale(6)` times 300 piety, the scale floored
at one; choosing strength increments melkor_sacrifice), or
sacrifices objects: an intact corpse records twice the monster race level as piety, a
spellbook without udun spells records twice the summed spell levels (levels_in_book) as
piety; other gods go
through HOOK_SACRIFICE_GOD. A non-altar square shows the god's message.

#### Scenario: Self-sacrifice for piety

- **WHEN** a Melkor worshipper with more than ten current and maximum hp stands on his
 god's altar and answers both confirmations
- **THEN** ten hp are lost, hp_mod drops by ten, and piety rises by `wisdom_scale(6)`
 times 300

- **Anchors**: `src/cmd2.c:4716-4834`

### Requirement: Stealing, Giving And Chatting

Stealing SHALL target an adjacent monster with carried objects (RF7_NO_THEFT refuses);
the object menu holds at most 23 items. The difficulty score is 40 minus the dexterity
stat, plus the object weight divided by (the stealing skill folded to 19 plus one), plus
(the stealing skill folded to 29 plus one), minus ten for a sleeping target, plus the
monster level; the theft fails when rand_int(difficulty) exceeds 1 + the stealing skill
folded to 25 - the monster wakes, gains five speed and becomes really *ANGRY*. On success
the object is unlinked and taken (gold is banked, the rest goes to the pack); races with
PR1_EASE_STEAL gain experience of half the object weight plus ten times the monster level
(half of that maximum guaranteed), and a confirmed phase door may follow. Giving SHALL go
through HOOK_GIVE (refused when nobody answers) and costs 100; chatting SHALL go through
HOOK_CHAT (refused when nobody answers) and costs no energy.

#### Scenario: Failed theft

- **WHEN** the player steals from an awake monster and `rand_int(chance)` exceeds
 `1 + get_skill_scale(SKILL_STEALING, 25)`
- **THEN** the attempt costs 100 energy, the monster wakes and gains five speed, and the
 "really *ANGRY*" message plays

- **Anchors**: `src/cmd2.c:4842-4984` (carried-object list), `:4990-5182` (stealing),
 `:5188-5266` (giving and chatting)
