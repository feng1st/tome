# game-loop Specification

## Purpose

The game main loop: `src/dungeon.c` carries the turn engine — `play_game` (game lifetime),
`dungeon` (level main loop), `process_player` (player energy and commands), `process_command`
(command dispatch), `process_world` (world evolution on every tenth game turn: regeneration,
digestion, piety, timer countdowns, equipment curses and recharging, floor-item decay and
hatching, word of recall), plus pseudo-identification and item sensing, pattern terrain,
and persistent floor spell effects. Player state pipelines (notice/update/redraw) and the
monster turn are documented in the capabilities behind `src/xtra1.c`/`src/xtra2.c`/
`src/melee2.c`; the savefile flows themselves are documented under save-load
(see specs/save-load/spec.md).

## Requirements

### Requirement: Game Lifetime

`play_game` SHALL drive the whole session: after the character-list screen it loads the
savefile (a broken file quits the game); skill indexes added since the save get their
default values; the complex RNG state table is seeded from the clock (on Unix the pid is
mixed in); option bits are expanded from `option_info`. A new game SHALL be admitted only
if the module answers `get_module_info("allow_birth")`, then runs HOOK_INIT, rolls the
object flavor and town layout seeds, runs `player_birth`, and starts astral characters on
level 98 of their dungeon and everyone else on the surface; undead races start just past the nightfall boundary
(the turn is set half a day in) instead of at dawn. It then loads `user.prf` plus the race/class/character preference files
and the character's automatizer script, initializes the vault word-list and the hooks,
repairs out-of-bounds player coordinates, generates a level when needed, and raises
HOOK_GAME_START. The main loop SHALL run each round: remember the old depth, greet an
astral character returning to town, enter the level loop, save persistent levels, apply
death fates (see below), forget light and view, `wipe_o_list`, adjudicate death, and
`generate_cave` when needed; on exit `close_game` runs.

#### Scenario: Undead start

- **WHEN** a new game begins and the character's race carries the undead flag
- **THEN** the turn is set half a day in (plus the dawn offset) while an astral
 character otherwise starts on level 98 and everyone else in town

- **Anchors**: `src/dungeon.c:5633-6148`

### Requirement: Level Main Loop

`dungeon` SHALL reset the command variables (cmd/new/repeat/arg/dir), the target, and the
health bar, and SHALL track max_plv and max_dlv (quest levels excluded). Mechanism:

- Quest levels (unless astral) must not spawn down stairs; town/wilderness levels ban
 stairs in both directions; with the connected-stairs option off both directions are
 banned.
- Leaving a level out of depth (below `mindepth` or above `maxdepth`) SHALL send the
 player back to the surface and reset the wilderness coordinates from the dungeon's
 entrance or exit coordinates.
- On entry the game SHALL place the connecting stair under the player (DF1_FLAT dungeons
 use the two-way road features).
- **Quirk:** the `DF2_NO_STAIR` branch condition is inverted relative to the flag's name —
 dungeons *without* the flag are the ones that get connecting stairs disabled. As-shipped
 behavior.
- Before the loop body the game SHALL run two full update passes in xtra mode and report
 the level feeling.
- Each loop pass SHALL: compact the monster/object lists when they near their caps, run
 `process_player`, refresh the four pipelines, run `process_monsters`, `process_world`,
 HOOK_END_TURN, `evolve_level` every ten turns in DF1_EVOLVE dungeons, and increment
 `turn`; death or departure exits the loop.
- In the death dungeon (DUNGEON_DEATH) the game SHALL wipe every staircase on the map,
 fix the monster generation level at 127, and set the object generation level to 0.

#### Scenario: Out-of-depth reset

- **WHEN** the player's depth leaves the current dungeon's min/max range
- **THEN** the game returns to the surface and places the player on the wilderness grid
 matching the dungeon's entrance or exit coordinates

- **Anchors**: `src/dungeon.c:5193-5591` (main loop), `:5241-5257` (stair bans and the inverted flag condition), `:5563-5590` (out-of-depth reset)

### Requirement: Player Energy And Command Pump

`process_player` SHALL accumulate energy from the `extract_energy` table according to speed
(clamped to 0-199); below 100 energy no turn is produced. With full energy it SHALL handle
rest completion (-1 stops at full HP and mana; -2 only after full recovery, the carried
symbiote healed, no bad states, and no drained stats) and interrupt checks (keypresses are
listened for while running, repeating, or resting). Every 100 energy SHALL run, in order:
the four pipelines refresh, wilderness grid known-marking, forced drop of the overflow pack
slot, then dispatch by state — paralyzed or stun at or above 100 consumes 100 energy, resting
decrements its counter, running calls `run_step`, a repeating command is replayed, and
otherwise `request_command` reads a key and `process_command` runs. After energy is spent
the pump SHALL handle hallucinated-map redraws, the triple shimmer (multi-hued monster,
object, and terrain panel redraws), monster-detection flag rotation (NICE turns hostile,
MARK/SHOW expire into forgetfulness), and full-map forgetting under DF1_FORGET. Death or
departure breaks the pump loop immediately.

#### Scenario: Paralysis burn

- **WHEN** the player is paralyzed (or stun has reached 100) with at least 100 energy
- **THEN** the pump consumes 100 energy without reading any command

#### Scenario: Rest completion

- **WHEN** the player rests with the full-recovery option and every completion
 condition holds (HP/mana full, the carried symbiote healed, no bad states, no
 drained stats)
- **THEN** resting is disturbed to a stop before the next command is read

- **Anchors**: `src/dungeon.c:4720-5183`

### Requirement: Command Dispatch

`process_command` SHALL first run HOOK_KEYPRESS (a hit swallows the key), then consult the
command table: all movement/door/mining/stair/store/skill-cast/prayer/pet/corpse-cutting/
stealing/item-use commands. The coarse gates are: while controlling a host (possession)
no active commands are allowed; the overland map forbids dungeon commands; the arena
absorbs every magical command (firing and throwing degrade to a hand-to-hand notice).
Wizard, debug, and Borg modes SHALL ask for confirmation once and mark the savefile
noscore bits 0x0002/0x0008/0x0010. Quit-and-save (KTRL-X), suicide, recording, the CLI,
knowledge screens, and the extended CMD_* command family each have their dispatch.
Unknown commands SHALL roll against the insanity percentage: on a hit a mad quote is drawn
from `error.txt`, otherwise the help-key hint is printed.

#### Scenario: Arena magic absorption

- **WHEN** the player casts a spell or reads a scroll inside the arena
- **THEN** the command is refused with "The arena absorbs all attempted magic!"

- **Anchors**: `src/dungeon.c:3576-4708`

### Requirement: World Tenth-Turn Processing

`process_world` SHALL run on every tenth game turn (one processing step per player turn at
normal speed; one overland step contains 132 of them): HOOK_PROCESS_WORLD and the player's
song magic fire (a song stops when mana runs out; both are skipped in the overland mode), the global Lua timer chain counts down,
class special maintenances run (mana path paving and recycling, mana wind explosion,
guided mana), a fate is rolled (with fate_option and level above 10, a 1-in-50000 chance of
a new fate), and the carried symbiote may turn hostile (a d1000 roll below monster level
minus (twice player level plus Symbiosis skill) makes the carried monster attack). Every 1000
game turns SHALL check time and system load: three gate warnings (closing_flag counts 0,
1, 2) precede a forced shutdown on the next failed check; timed autosaves trigger at their
configured frequency.

#### Scenario: Symbiote rebellion

- **WHEN** the processing step rolls a d1000 below the carried monster's level minus
 twice the player's level minus the Symbiosis skill
- **THEN** the carried monster announces the loss of symbiosis and makes a normal
 attack against the player

- **Anchors**: `src/dungeon.c:1268-1407`

### Requirement: Day Cycle And Loan Accounting

On the surface (outside the overland mode) the game SHALL switch day and night every half
day: at dawn the whole town gains light and lit grids are memorized per option, at sunset
plain-floor grids lose both light and memory; crossing a day boundary announces the
Middle-earth calendar date. Every STORE_TURNS processing step there is a 20% chance the
bounty list is replaced. An outstanding loan SHALL lose one unit of its deadline per step;
once overdue, every 5000 turns one twelfth of the loan is added as interest capped at the
gold maximum, and the punishment branch picks: in the wilderness (half the time) one random carried slot (pack or equipment; empty slots
and items already discounted to 100 are skipped, up to 200 tries) gets its discount raised
by 70 capped at 100; otherwise 5 + level/3 mercenaries and black
market spies are summoned around the player (each summon is a mercenary 80% of the time,
otherwise a black-market agent; their experience aligned to twice the player's
level).

#### Scenario: Overdue loan in town

- **WHEN** the loan is overdue and the turn crosses a 5000-turn boundary while the
 player stands in town
- **THEN** 5 + level/3 mercenaries and black-market agents (80%/20% per summon) are
 placed on empty grids around the player as enemies

- **Anchors**: `src/dungeon.c:1410-1589`

### Requirement: Damage Over Time And Environmental Death

Every processing step SHALL apply: poison drains 1 HP (invulnerability exempt); the
light-sensitive take 1 HP and lose regeneration on that grid when standing in daylight on
a lit surface grid or wielding a holy light source; drowning (damage rolled per level)
hits characters over half their weight limit in deep water without levitation, water
walking, water breathing, or magic breathing; standing in an impassable grid — tree walking
or the climbing ability/flag exempts, the semi-wraith state takes 1 + level/5 damage with
regeneration disabled (never reduced below 1 HP); cuts drain 1/2/3 HP by tier (above 200,
above 100, otherwise; invulnerability exempts); hunger below the starvation tier drains a tenth of the shortfall.

- **Anchors**: `src/dungeon.c:1610-1731`, `:1801-1809`, wall damage `:1675-1705`

### Requirement: Digestion And Regeneration

Digestion SHALL cost food every 100 game turns at twice the current speed's energy value,
plus: 30 for fast regeneration, a tenth of the timed regeneration power, half the
invisibility value, 40 for invulnerability, 30 for wraith form, five times the parameter
of a life-multiplier weapon; slow digestion subtracts 10; the minimum is 1. The gorged
state costs 100 food per processing step (every 10 game turns — the 100-turn gate does
not apply to it). Each food tier SHALL lower regeneration:
starvation stops it, the faint tier occasionally faints (paralysis of 1-5 turns that
ignores free action), the weak tier halves it. Regeneration SHALL be doubled by the
regeneration ability and by searching/resting, and increased by the timed regeneration
power; poison and cuts zero it; pattern terrain and no-regen grids zero it; a Yavanna
worshipper standing on grass adds 200 plus the wisdom-800 scaled amount. Mana regeneration
SHALL add a triple-intelligence modifier; with pets in tow it is reduced by a percentage
capped by total pet levels (perfect casting relaxes the divisor). HP regeneration SHALL
accumulate in 16.16 fixed point (percent x maxhp plus a base, then shifted right by 16);
the undead form does not regenerate.

#### Scenario: Fainting from hunger

- **WHEN** the food value sits below the faint tier and a d100 roll comes out below 10
- **THEN** the player faints into a 1-to-5-turn paralysis that free action does not
 prevent

- **Anchors**: `src/dungeon.c:1734-1866` (digestion and regen tiers), pet upkeep `:1868-1918`, fixed-point regen `:633-741`

### Requirement: God Piety Timers

Every 100 game turns a character whose god is Eru and who is not currently praying SHALL
gain piety from Eru scaled as a tenth of wisdom (characters that did nothing this turn, and
the overland mode, are excluded). Every 300 game turns characters inside a dungeon whose
did-nothing marker is clear (they acted this step) and who are not in the overland mode
SHALL lose piety: Manwe at 4 minus a
wisdom-3 scaling (worshippers add 1, elves additionally subtract a wisdom-2 scaling),
Melkor at 8 minus a wisdom-6 scaling (worshippers add 1, elves add 5 minus a wisdom-4
scaling), Tulkas worshippers at 4 minus a wisdom-3 scaling; every
400 turns Yavanna worshippers lose 5 minus a wisdom-3 scaling (Ents subtract a
further wisdom-2 scaling). All drains have a floor of 1. The did-nothing marker clears
every processing step.

- **Anchors**: `src/dungeon.c:1920-1979`

### Requirement: Global Timer Countdown

Every processing step SHALL decrement roughly fifty player timers by one and finalize each
through its corresponding set_* routine: water walking, perfect strikes, meditation, timed
projection (with GF and damage parameters), root spines (with armor and damage), water and
magic breathing, timed regeneration, disruption shield, parasite (with monster race),
reflection, probability travel, time resistance, slow-descent flight, thunderstorm (each
step strikes a random hostile monster in view — damage split in three equal parts of
electricity/light/sound), poison hands, fire aura, light, blindness, no-breeding, mimicry,
fixed counters, invisibility (with power), see-invisibility, telepathy, infrared, paralysis,
confusion, fear, haste, lightspeed, slowness, protection from good/evil/undead, invulnerability,
wraith form, hero/superhero, blessing, shield (with five parameters), the nine elemental
resistances, mental barrier, and rush. Mimic limbs (a counter in the high 16 bits of
`mimic_extra`) hitting zero removes the extra legs/arms/wall affinity and refreshes the
body. Temporarily drained stats restore one per timer; hallucination decrements per step.

#### Scenario: Thunderstorm strike

- **WHEN** the thunderstorm timer is active and a visible hostile monster exists
- **THEN** a random such monster takes three projections of a third of the rolled
 damage each, one of electricity, one of light, one of sound

- **Anchors**: `src/dungeon.c:2016-2510`

### Requirement: State Healing And Dungeon Damage

Poisoned/stunned/cut counters SHALL heal by the constitution-scaled rate plus one per
processing step; a cut above 1000 is a mortal wound and does not self-heal. Dungeon-level
damage SHALL, per the four frequency groups of `d_info` (when the turn modulo reaches
zero), roll dice and fire the matching GF projection at every grid of the map (projectile
source index -100): by default empty grids and monster grids are skipped, and with
DF1_DAMAGE_FEAT empty grids are hit too. The level flags DF2_WATER_BREATH/DF2_NO_BREATH
suffocate characters lacking the matching breathing mode for 3dlevel per step.

- **Quirk:** the no-flag "skip empty grids" test reads `(j != p_ptr->py) && (i != p_ptr->px)`
 where `i` is the frequency-group index (0-3), not the column loop variable `k` — so
 without `DF1_DAMAGE_FEAT`, empty grids on the player's row are hit too (and, whenever the
 player's x coordinate is 0-3, so are all empty grids of the matching group).

#### Scenario: Suffocating water dungeon

- **WHEN** the dungeon level carries DF2_WATER_BREATH and the player lacks water
 breathing
- **THEN** every processing step deals 3dlevel suffocation damage

- **Anchors**: `src/dungeon.c:2512-2542` (state healing), `:2544-2587` (dungeon damage), `:2800-2810` (suffocation)

### Requirement: Persistent Spell Effects

Every processing step SHALL process the grid effect table in two passes: a full map scan of
effect references on grids — an effect with lifetime fires its GF and damage as a
projection onto its grid, one without is cleared; EFF_WAVE wave effects expand their radius
by one ring per step (along the EFF_DIR1-9 sector or omnidirectionally, grids at exactly
the radius within line of sight receive the effect), EFF_STORM storm effects re-lay their
radius around the player's new center each step; the interior of a wave sheds each step and
storm grids are wiped before being re-laid (EFF_LAST effects persist). Afterwards each
effect slot has its lifetime decremented and its expansion executed. The grid the player
stands on SHALL additionally run terrain-level effects (the feature's four d_frequency
groups fire, a die count or size of -1 takes the player's level).

#### Scenario: Wave expansion

- **WHEN** a wave effect without EFF_LAST is processed
- **THEN** the grids closer than one ring inside its current radius are cleared first,
 after which the radius grows by one and the new ring within line of sight takes the
 effect

- **Anchors**: `src/dungeon.c:2589-2798` (per-step effect machinery), terrain effects `:1088-1131`

### Requirement: Black Breath

Black breath SHALL work on two tracks: a warning every 3000 turns (silent when equipment
carries TR4_BLACK_BREATH); and each step a 2% chance with the resistance flag, otherwise
5% — experience and max experience each drop by 1 + level/5, one random stat is drained,
and experience is recomputed. The anti-undead barrier against attack-transmitted black
breath is specified in specs/monster-melee/spec.md.

#### Scenario: Black breath drain

- **WHEN** the black-breath state is active, the race lacks the resistance flag, and the
 d100 roll lands below 5
- **THEN** experience and max experience each drop by 1 + level/5, one random stat is
 drained, and experience is recomputed

- **Anchors**: `src/dungeon.c:2812-2846` (warning), `:2900-2923` (drain)

### Requirement: Light Fuel

Consumable light sources SHALL lose one fuel unit per processing step: the equipment window
refreshes below 100 or at exact multiples of 100; while blind a floor of one unit keeps the
light alive; burning out disturbs and announces the light going out; below 100 the
"Growing faint" message plays and triggers a wilderness drop (`drop_from_wild`). The torch
radius is recomputed each step.

#### Scenario: Blind light conservation

- **WHEN** the fueled light source reaches timeout zero while the player is blind
- **THEN** the timeout is pushed back to one so the light survives until the player can
 see again

- **Anchors**: `src/dungeon.c:2849-2898`

### Requirement: Mana Life Experience Drains

Each processing step SHALL: run `drain_mana` (30% chance of double drain; hitting bottom
disturbs); charge the partial-summon upkeep as a basis-point fraction of mana with the
remainder preserved; run `drain_life` for the value plus up to 1% of max HP; and with 10%
chance run `exp_drain`, removing one experience point and recomputing.

- **Anchors**: `src/dungeon.c:2925-3000`

### Requirement: Equipment Curses And Recharging

Each processing step SHALL walk the equipment slots: a TY curse triggers `activate_ty_curse`
1% of the time, a DG curse triggers `activate_dg_curse` 1-in-50 and marks itself cursed,
an auto-curse marks itself cursed 1-in-15; the teleport flag fires 1% — a cursed item
teleports the player 40 unconditionally (anti-teleport excepted), an uncursed one gives up
in the overland mode or when the inscription contains a period, otherwise asks and
teleports 50. Ordinary timeouts decrement per step and announce the recharge on reaching
zero per the double-bang inscription convention; the second spell counter of mage staffs of
spells decrements. Temporary items (TR5_TEMPORARY) are destroyed as a whole stack when
their timeout reaches zero. Rods (TV_ROD_MAIN) regain one or two units per step according
to the charging flag, capped at the second parameter. Random artifacts with the
activate-no-wield flag decrement their timeout. Decaying items (TR3_DECAY) lose parameter
value at double rate in hot dungeons, a quarter rate in cold ones (a 1-in-2 gate then a 50%
roll), normal rate otherwise; their timeout counts down one per step while it is positive
and below the item's weight; at parameter zero they become
a skeleton. Eggs, once their timeout is zero, lose parameter per step; on hatching a pet
spawns within five grids of the player (upgraded to companion when it can be imprinted) and
the egg is removed.

#### Scenario: Cursed teleport

- **WHEN** a cursed item with the teleport flag rolls its 1% chance and the player lacks
 anti-teleport
- **THEN** the player is teleported 40 grids with no confirmation asked

- **Anchors**: `src/dungeon.c:3002-3100` (equipment), `:3101-3241` (pack)

### Requirement: Floor Object Evolution

Each processing step SHALL walk floor objects: temporary items are destroyed, rods recharge
silently, decaying items decay under the same rules (becoming a skeleton on the floor at
zero), and eggs hatch into hostile monsters near the egg's location.

#### Scenario: Hatching floor egg

- **WHEN** a floor egg's parameter counts down to zero
- **THEN** a hostile monster of the egg's race hatches on a free grid within five grids
 of the egg and the egg item is removed

- **Anchors**: `src/dungeon.c:3249-3342`

### Requirement: Word Of Recall

The recall countdown SHALL decrement per processing step; its gate chain is: a HOOK_RECALL
veto cancels it; DF2_NO_RECALL_OUT refuses; the death dungeon refuses (fated to fight);
astral characters refuse. At a count of one the game autosaves per option and force-saves
the persistent level; at zero — quest levels are cancelled by a strong force; inside a
dungeon the dungeon is remembered into recall_dungeon and the player is sent to the
surface; on the surface the player is sent to recall_dungeon at its deepest reached level
(floor 1) with the old coordinates reset. Recall activation disturbs the player.

#### Scenario: Recall from depth

- **WHEN** the recall counter reaches zero while the player stands inside a dungeon
- **THEN** the current dungeon is remembered into recall_dungeon and the player is
 yanked upwards to the surface

- **Anchors**: `src/dungeon.c:3345-3453`

### Requirement: Pseudo Identification

Item sensing SHALL run on two tracks: combat tvals are driven by the combat skill (the fast
formula 9000 / (skill squared + 40)), magic tvals by the magic skill (the slow formula
12000 / (skill + 5)), the daemon book qualifies for both; no sensing while confused; a
skill above 10 uses the heavy judgement table (ten SENSE_* tiers by curse/ego/modifier/base
price), otherwise the light table (three tiers); pack items are skipped 4 times out of 5;
already-sensed or fully-known items are not repeated; a hit sets IDENT_SENSE and the sense
field and announces the feeling. Psychometry SHALL force sensing of a single object through
the heavy tables (magic table first, combat table second) and states outright when nothing
new is found.

#### Scenario: Heavy pseudo-id hit

- **WHEN** the combat skill exceeds 10, the fast-formula roll hits, and an unsensed
 pack item qualifies
- **THEN** the item is judged by the heavy table, marked with IDENT_SENSE and the
 sense field, and a "You feel ..." message names the feeling

- **Anchors**: `src/dungeon.c:38-293` (judgement tables and tval domains), `:308-462` (sensing main flow), `:831-899` (psychometry)

### Requirement: Pattern Terrain

Standing on pattern terrain SHALL: the plain sections deal 1d3 damage per step unless
invulnerable; the end
section heals fully (clears six states — poison, hallucination, stun, cut, blindness, fear,
restores all six stats and drained experience, heals 1000 HP) and degrades into the old
section (inert; a source comment muses about making the healing one-time only, which the
degradation already is); the teleport section opens the level-jump dialog (with
confirmation any level 0-99 may be named, or after confirmation the player teleports
randomly 200 grids in place, with an autosave before the jump); the corrupted section costs
200 HP unless invulnerable. The pattern area forbids regeneration throughout. The Straight
Road test covers the range from FEAT_PATTERN_START through XTRA2.

#### Scenario: Pattern end healing

- **WHEN** the player steps onto the pattern end section
- **THEN** the six states clear, all six stats and drained experience restore, 1000 HP
 heal, the grid degrades to the inert old section, and a message announces the weaker
 road section

- **Anchors**: `src/dungeon.c:468-580`

### Requirement: Resurrection Adjudication

Death adjudication SHALL weigh escapes from death in order: a HOOK_DIE hook intercept
spares the character; an Eru worshipper currently praying with piety above 100000 is
raised by miracle with a 70% chance (piety is set to -200000); a positive Blood of Life
count consumes one charge to resurrect; wizard mode or the cheat_live option after
confirmation spares the character (the savefile gets noscore mark 0x0001; a nonzero social
class is zeroed together with the age, which is then set to 1; otherwise the age simply
increments). A spared death SHALL restore the winner status, add one to
the life count, refill HP, sanity, and mana, clear the eight states (blindness, confusion,
poison, fear, paralysis, hallucination, stun, cut), clear black breath and the
undead form, refill food to just below the gorged threshold, cancel recall, and record the cause of death as
"Cheating death". True death or a save-and-quit breaks the loop for shutdown.

- **Discrepancy:** the code comment says "Increase age", but a character with a nonzero
 social class has age reset to 0 and then incremented — the age becomes 1 rather than
 growing by one.

#### Scenario: Blood of Life save

- **WHEN** the character dies with a positive Blood of Life count
- **THEN** one charge is consumed, the death is cheated, and the full spared-death
 restoration runs

- **Anchors**: `src/dungeon.c:229-240` (Eru miracle), `:6010-6133` (adjudication chain)

### Requirement: Undead Death Points

The undead form (`necro_extra` carrying CLASS_UNDEAD) SHALL lose exactly one hit point per
processing step (ignoring invulnerability and wraith form); at negative HP the character
dies of "being undead too long", losing winner status, with optional dying words and a
death screenshot before shutdown. Below the warning line the game rings a bell per option.

#### Scenario: Undead expiry

- **WHEN** the undead form's death points drop below zero on a processing step
- **THEN** the character dies of "being undead too long", loses winner status, and
 receives the optional last-words prompt and screenshot offer before shutdown

- **Anchors**: `src/dungeon.c:2049-2123`
