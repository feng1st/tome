# character-birth Specification

## Purpose

Character creation: `src/birth.c` carries the whole new-character flow - the boot
savefile-list screen, the full state wipe, the stat rolling and point-buy routes, the
birth history generation, the starting equipment, the random quest setup, the six-step
question flow (sex/race/subrace/class/speciality/god), the quick start, the background
text editor, and the dungeon random-town and wilderness seed initialization. The
attribute effect recalculation (PU_BONUS and friends) is specified in
specs/player-derive/spec.md; skill computation is specified in specs/skills/spec.md.

## Requirements

### Requirement: Savefile List Screen

The boot SHALL read the savefile index file (per-uid files in multi-user setups); the row
format is module@alive@savefile-name@description (`@`-separated; when an old format has no
module field, ToME is recorded by default). Only savefiles loadable by the current module
are listed (ruled by `module_savefile_loadable`). The menu SHALL offer: new character
(input a savefile name), load an unlisted savefile, pick an existing savefile (alive in
green/dead in red), and delete after a backspace confirmation. On write-back it SHALL
update the current character's row (including the full race/class name and the
alive/dead description) and keep the other rows.

- **Anchors**: `src/birth.c:3529-3716` (index read/write), `:3768-3920` (menu)

### Requirement: Full State Wipe

`player_wipe` SHALL reset, before every re-roll: the special-level cache; the player
struct (the power and corruption bitmap pointers are kept, everything else zeroed); the
four history lines; the special-level marker table; the random-town flags; all quest
states back to UNTAKEN; the rune spell table; pack and equipment; every artifact's
cur_num; the object word-list tried/aware/know/artifact; the monster word-list cur_num
and max_num (uniques to one, UNIQUE_4 to four) and the kill counts; satiation to
PY_FOOD_FULL-1; the alchemy known titles and artifact bitmaps; the six cheat options;
noscore and wizard; the innate spell count; the fate table; the black-breath and undead
mundane markers; the pet defaults (follow distance six, no door opening, no picking up);
the possession state; the bounty count; the extra spells and hp modifier; the monster
table; the Doppelganger; every dungeon's deepest-level record; the inscription and trap
known tables; the wilderness mode; the blood-of-life count; the loan; the ability modifier
table; the companion kill count.

The random artifact roster SHALL be regenerated: the short and full names drawn row by
row from the two rart word-lists, color rolled 1-15, the activation effect drawn from the
full table, and the cost randnor(0,250) with negatives clamped to zero.

- **Anchors**: `src/birth.c:821-1058` (wipe), `:644-676` (random artifact roster)

### Requirement: Stat Rolling

Stats SHALL be rolled with eighteen dice (d3/d4/d5, six rounds each) and accepted only
when the sum lies strictly between 42 and 57; each stat takes 5 plus three dice (range
8-17), then stacks the race plus subrace plus class modifiers - with maximize on through
`modify_stat_value` directly, with it off through `adjust_stat` with a random fold. Luck
SHALL be the race plus subrace base plus a -5..5 roll.

The stat decrease rule SHALL run from high to low (at 28 and above steps down ten, above
18 drops
to 18, above 3 steps down one); the increase rule runs from low to high (below 18 plus
one; with maximize, plus ten per point; the 18/70 band plus randint(15)+5; the 18/90 band
plus randint(6)+2; the 18/100 band plus one).

- **Anchors**: `src/birth.c:375-456` (rolling), `:308-365` (increase/decrease rules)

### Requirement: Life Lines And Physique

The experience multiplier SHALL be the sum of the three tables; the hit dice are the sum
of the three tables and equal the level-one hit points. The per-level hp table SHALL have
its first cell equal to the hit dice, and every later level adds randint(hit dice),
re-rolled until the last level falls between the lower bound (50 x (hit dice - 1) times
3/8, plus 50) and the upper bound (the same formula with 5/8). Age SHALL be the base plus
randint(spread); height and weight SHALL be rolled by randnor with the sex's mean and
standard deviation (no sex takes the average of both sexes), floored at one. The tactic
and movement modes default to index 4 (the "normal" entry).

- **Anchors**: `src/birth.c:462-534` (life lines), `:736-744` (age), `:682-730`
 (height/weight)

### Requirement: Birth History And Social Class

The social class SHALL start from a d4, walk the race history chart picking a path by a
d100 roll per segment, and accumulate the segment modifiers (bonus-50), clamped to 1-100.
The history text SHALL join the selected segment texts, trim the leading and trailing
spaces, and word-wrap at sixty columns into the four history lines.

- **Anchors**: `src/birth.c:540-638`

### Requirement: Starting Gold

The rolling route's starting gold SHALL be social class x 6 plus d100 plus 300, reduced
per stat (18/50 and above -300 each, 18/20 and above -200 each, above 18 (any 18/xx bonus)
-150 each,
otherwise (stat-8) x 10 each), floored at 100. The point-buy route instead folds the
unused points (see the point-buy requirement).

- **Anchors**: `src/birth.c:752-775`

### Requirement: Starting Equipment

`player_outfit` SHALL, in order: give the adventure-guide scroll (TV_PARCHMENT sval 20)
in any non-pristine town; trigger HOOK_BIRTH_OBJECTS; give three to seven rations and
three to seven torches (each with 500 x roll(3-7) fuel). Holders of the trap-setting power
SHALL pre-know three traps (TRAP_OF_DAGGER_I/POISON_NEEDLE/FIRE_BOLT, familiarity
randint(50)+50 and marked identified) plus five to fifteen normal shots (recorded as
store-bought). It SHALL then work through the race/subrace/class/speciality four outfit
tables item by item with `outfit_obj`: the count rolled by dd ds, the pval from the table,
IDENT_MENTAL set (store-bought), auto-aware and identified, into the pack.

- **Anchors**: `src/birth.c:1104-1194` (granting), `:1061-1096` (single item)

### Requirement: Random Quest Setup

Creation SHALL ask for the optional random quest count (when the module's `rand_quest`
allows and the level is neither an ironman room nor persistent; 0 to the cap minus one,
defaulting to an input of twenty, `*` random) and hook the monster draw as
`monster_quest`. `gen_random_quests` SHALL split level 98 into n equal segments and place
one quest per segment: the quest level converts from the segment center, the host dungeon
is the DF1_PRINCIPAL dungeon covering that level; the type is drawn from a 32-entry table
(the princess line three sets and the hero-sword line eight entries each); the quest
monster SHALL be drawn by depth+4+d6 and exclude - special/never generation flags,
breeders, joke monsters, pets, Nazgul, good alignment, self-destruct attack spells. Type
one (kill) takes only uniques (and only those not already taken by another quest); other
types refuse uniques; the monster level must exceed the depth (depth above 49 caps at 49,
forcing an over-leveled monster); a failed 5000 roll zeroes that slot's type (wizard mode
warns); a drawn unique is marked max_num=-1 exclusive.

The plot lines SHALL, when the module's `C_quest` allows, initialize seven slots (the
main line is QUEST_NECRO recorded TAKEN; BREE/LORIEN/GONDOLIN/MINAS/KHAZAD each get their
matching quest UNTAKEN; the remaining slots stay empty) and trigger
`quest_random_init_hook`.

#### Scenario: Quest monster draw exhausted

- **WHEN** none of the 5000 draws for a random-quest slot satisfies the exclusions
- **THEN** that slot's type is zeroed and wizard mode logs "Could not find quest monster"

- **Anchors**: `src/birth.c:2431-2524` (asking and plot lines), `:1197-1333` (generation)

### Requirement: Six-Step Question Flow

The questions SHALL fix, in order:

- sex (no real gameplay difference, `*` random);
- race (arrow-key browsing with instant description; a single option is taken directly;
 after selection a random name is generated from the syllable tables);
- subrace (filtered by the choice bitmap of the current race: no match takes zero
 directly, a single match is taken directly, several open a menu);
- class (the metaclass family first, then the class list; the allowed set is (race choice
 OR subrace pclass) minus the subrace mclass, used only for display coloring; under the
 RESTRICT_COMBINATIONS compile switch, an over-limit combination gets the no-score flag
 0x0020 and a Cheater banner recorded);
- speciality (a single speciality is taken directly);
- god - a god-refusing race records GOD_NONE directly; the selectable set is the union of
 the class and speciality gods bitmaps (empty means no god, a single one is taken
 directly, several open a menu with a no-god option); a god-favored subrace starts with
 200 piety, otherwise 100.

#### Scenario: God-refusing race

- **WHEN** the chosen race refuses gods
- **THEN** `GOD_NONE` is recorded directly and no god menu opens

Throughout, Q quits, S restarts, `=` opens the birth options menu, `?` opens help
(race/subrace/class/god go through `ingame_help` context help).

- **Anchors**: `src/birth.c:1641-2362`

### Requirement: Birth Options And Starting Place

The question tail SHALL fix four birth options (maximize/preserve/special_lvls copied
from the global switches, astral by PR2_ASTRAL); with the persistent levels option on, a
dungeon seed is rolled, otherwise it is zeroed; the return host dungeon comes from the
module's `base_dungeon` with its shallowest level recorded as reached; the Astral subrace
moves to the module's `astral_dungeon` and is placed by the module-given wilderness
coordinates.

- **Anchors**: `src/birth.c:2378-2426`

### Requirement: Point-Buy Stat Allocation

Point-buy SHALL buy the six stats from a 48-point pool, each stat from 10 to 18, with the
cost table {0,1,2,4,7,11,16,22,30}; overspending rings the bell, steps the stat back one
and recomputes. With maximize off, the race and class modifiers merge through
`modify_stat_value`; gold is 100 times
the unused points plus 100, capped at 600. Before allocation SHALL come the life
lines/physique/birth-history rolls and the HOOK_BIRTH trigger plus the chaos patron draw;
keys 2/8 switch stat, 4/6 subtract/add, ESC accepts.

- **Anchors**: `src/birth.c:2536-2729`

### Requirement: Rolling Route And Auto-Roller

The non-point-buy route SHALL roll: with the auto-roller on, per-stat minimums are
entered (the cap is computed by adjust_stat(17, modifier, TRUE) and shown; the old-style
display adds two parts, the linear display folds everything above 18 times ten), and
`get_stats` loops until all six stats meet the minimums or one million rounds cap out;
every 25 rounds (or during the first hundred) the hit percentage and round count refresh;
any key stops the loop and takes the current values; with the auto-roller off, a single
roll. Each round SHALL carry the life lines/physique/birth-history/gold rolls, the
HOOK_BIRTH trigger and the chaos patron draw. The interaction SHALL support r or space to
re-roll, p to restore the previous round (the current round steps aside into a holding
slot), h to toggle the background and miscellaneous page, ESC to accept; before
accepting, the values are recorded into the quick-start savefile.

- **Anchors**: `src/birth.c:2734-3074`

### Requirement: Skill Initialization And Background Editing

After the questions SHALL come `compute_skills` then `init_skill` for every skill row,
with valued rows marking their parent chain dev level by level (only the relevant
branches expand). The quick start SHALL call `load_prev_data` directly, restoring the
previous round's values and re-rolling the life lines. The regular route SHALL enter the
background editor: four lines of sixty columns cursor editing (arrows move, characters
overwrite, lines wrap when full, Enter accepts, ESC restores the old text), then `get_name`
names the character.

- **Anchors**: `src/birth.c:3084-3253`

### Requirement: Wrap-Up And World Initialization

`player_birth` SHALL first validate the history chart table (recursive existence checking
and roll-100 terminal segments; a violation exits directly). After a re-roll is accepted
it SHALL: zero the skill points, record skill_last_level=1, run `recalc_skills`, zero the
ability table's acquired then run apply_level_abilities(1), execute `follow_god` by the
pgod swap, run `select_default_melee`, note the birth, write the separator banner into the
message area, and grant the starting equipment.

World initialization SHALL: give each DF1_RANDOM_TOWNS dungeon zero to several random
towns at a decreasing TOWN_CHANCE-minus-ten probability per try (the level drawn between
mindepth and maxdepth-1 without repetition; the town is recorded TOWN_REAL, unexplored,
with a rolled seed; the store count is hard-coded to eight); clear destroyed on all towns
and run `create_stores_stock` plus a per-store `store_init`; roll the seeds for the whole
wilderness grid, clear the entrances, mark unexplored; finally `select_bounties` picks
the bounty monsters.

- **Anchors**: `src/birth.c:3378-3524` (main flow), `:3259-3350` (table validation),
 `:3355-3370` (town initialization)
