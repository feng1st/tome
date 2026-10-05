# tables Specification

## Purpose

Global data tables: `src/tables.c` is the engine's static data collection — direction
arrays, the eighteen stat modifier tables, the blows table, the speed energy table, the
experience table, the option table (with defaults), the Chaos patrons and their reward
tables, the martial arts tables, the four bookless spell tables, gods and the tactic and
exploration tiers, the miscellaneous activation table, grid inscription table, artifact
forging flag groups, racial power table, initial quest table, monster power table, the tval
help text, the between-gate exits, the elven calendar, body part caps, and the GF message
table. Values defer to the source at the anchor.

## Requirements

### Requirement: Stat Modifier Tables

The stat modifier tables SHALL be indexed by stat_ind (38 tiers from 3 up to 18/220+):
the casting family — adj_mag_study (half the spell count), adj_mag_mana (half the mana),
adj_mag_fail (minimum failure rate), adj_mag_stat (spell level bonus); adj_chr_gold (the
store payment percentage, 130 down to 78); adj_int_dev (magic device bonus); adj_wis_sav
(saving throw bonus); adj_dex_dis and adj_int_dis (disarming bonus); adj_dex_ta (armor
class modifier with a 128 offset); adj_str_td, adj_dex_th, adj_str_th (damage and hit
modifiers with offsets); adj_str_wgt (weight limit in ten-pound units, reused by the
possession skill conversion); adj_str_hold (weapon weight limit); adj_str_dig (digging
base); adj_str_blow and adj_dex_blow (blows table indexes); adj_dex_safe (theft and fall
protection); adj_con_fix (poison/stun/cut healing rate); adj_con_mhp (per-level half
hit-point modifier with offset).

- **Anchors**: `src/tables.c:58-1018`

### Requirement: Core Mechanic Tables

The blows table blows_table SHALL be a 12x12 matrix (P = strength index times the class mul
divided by div, D = dexterity index), with the cap set by the class num (the header comment
lists six class parameters). The speed energy table extract_energy SHALL cover speeds
0-299: slow speeds start at one energy, normal speed ten, speed thirty past normal 38,
with an asymptotic cap of 49. The experience table player_exp SHALL list the
requirements for fifty levels (10 up to 5,000,000, converted by expfact). The arena monster
sequence arena_monsters SHALL list twenty-nine monster race indexes. The body part caps
max_body_part SHALL be weapons 3/torso 1/arms 3/fingers 6/head 2/legs 2. The sex table
SHALL be Female/Male/Neuter with the winner titles Queen/King/Ruler.

- **Anchors**: `src/tables.c:1022-1088` (blows), `:1124-1156` (energy), `:1164-1216` (experience), `:1091-1098` (arena), `:4696-4704` (body parts), `:1225-1240` (sexes)

### Requirement: Option Table

option_info SHALL register 105 options with variable pointer/default value/page
number/bit number/internal name/description: the interface page with nineteen entries
(rogue_like_commands default off, quick_messages on, use_old_target off, always_pickup
off, depth_in_feet off, stack-merge inscriptions on, list labels and weights on, bell off,
use_color on); the disturbance page (find_ignore_stairs off, find_ignore_doors on,
find_cut off, find_examine on, disturb_near on, panel/state/minor on, disturb_detect on,
last_words on, speak_unique on, auto_destroy on, wear_confirm on, confirm_stairs off,
easy_open on, easy_disarm on, easy_tunnel off); the game-play page (auto_haggle on,
auto_scum on, both stacking permissions on, view_perma_grids on, torch_grids off,
monster_lite on, dungeon_align and dungeon_stair on, flow sound and scent off,
plain_descriptions on, the two smart entries off, stupid_monsters off, small_levels and
empty_levels on); the efficiency page (all off except flush_failure and fresh_before on,
compress_savefile on); the ToME page (ingame_help on, exp_need off, old_colors off,
auto_more off, player_char_health on, linear_stats on); the birth page (maximize on,
preserve on, autoroll on, point_based off, permanent_levels off, ironman_rooms off,
take_notes and auto_notes on, fast_autoroller off, joke_monsters off, always_small_level
off, fate_option on); the testing page (testing_stack on, testing_carry on). The window
flag description table window_flag_desc SHALL list thirty-two entries (twelve positions
named, the rest empty).

- **Anchors**: `src/tables.c:1348-1701` (option table), `:1297-1331` (window flags)

### Requirement: Chaos Patrons

The sixteen Chaos patrons SHALL each carry a preferred stat (chaos_stats, Balo is -1) and a
twenty-entry reward table (chaos_rewards: a weighted sequence from REW_WRATH to
REW_AUGM_ABL, where an IGNORE entry is a blank draw); the patron name list runs from
Slortar to Khaine (with the four Warhammer gods as guests).

- **Anchors**: `src/tables.c:1704-1883`

### Requirement: Martial Arts And Bookless Magic

The hand-to-hand table ma_blows SHALL hold seventeen strikes (from the minimum 2d4 to the
dragon fist 20d10, with knee strike/slow foot/stun carrying effects and power); the bear
form table bear_blows SHALL hold eight strikes (claw up to double strike, including
MA_WOUND and MA_FULL_SLOW). The four bookless ability tables SHALL: mindcraft twelve
powers (Precognition through Telekinetic Wave), undead six (Horrify/Raise Dead/
Necromantic Teeth/Absorb Soul/Vampirism/Death — Death costs 100 mana and kills the
opponent and the caster together, turning the caster undead), mimicry five (Mimic/Invisibility/leg-wall-arm mimicry), symbiosis
nine (Hypnotise through Force Symbiosis). Each entry carries its gain level/mana cost/
failure rate and description.

- **Anchors**: `src/tables.c:2491-2531` (martial arts), `:2534-2732` (the four spell tables)

### Requirement: Gods And Tactics Exploration

The god table deity_info_init SHALL hold six gods (Nobody/Eru/Manwe/Tulkas/Melkor/Yavanna)
each with ten description lines (the birth screen shows the first four); the divine favor
wording table deity_niceness has ten tiers and standings eleven tiers (cursed through
championed). The tactics table tactic_info SHALL have nine tiers (coward through berserker,
symmetric trade-offs over the hit/damage/armor/stealth/disarming/saving modifiers); the
exploration table move_info SHALL have nine tiers (slug-like through running, over the
speed/searching/stealth/perception four modifiers).

- **Anchors**: `src/tables.c:2739-2870` (gods), `:2875-2965` (tactics and exploration)

### Requirement: Miscellaneous Activations And Grid Inscriptions

The miscellaneous activation table activation_info SHALL hold 51 entries
(name/cost/ACT tier: death and ruination free, destruction 1000, summon pet 1010, the cure
family 500 to 5000, genocide 5000, mass genocide 10000, acquirement 30000 and more). The
grid inscription table inscription_info SHALL hold eight entries: the empty entry, light
("ure nimir"), darkness ("lomi gimli"), storm ("dulgi bawiba"), guarding ("pedo mellon a
minno" — triggered only when a monster steps), summon dwarf ("Baruk Khazad! Khazad
aimenu!" — triggered only when inscribed), rift ("dunna hrassa" — monster steps only), black
fire ("burz ghash ronk"); each carries its execution flag bit combination and mana cost.
The pseudo-identification feeling table sense_desc SHALL hold eleven wording tiers.

- **Anchors**: `src/tables.c:2892-2948` (activations), `:2970-3020` (inscriptions), `:3025-3038` (feelings)

### Requirement: Artifact Forging Flag Groups

flags_groups SHALL hold twelve groups (Fire/Cold/Acid/Lightning/Poison/Air/Earth/Mind/
Shield/Chaos/Magic/Antimagic), each with a color/price tier/the five flag groups and an ESP
bit — used by alchemy forging and weapon upgrading to draw from a group.

- **Anchors**: `src/tables.c:3051-3173`

### Requirement: Racial Powers And Monster Powers

The racial power table powers_type_init SHALL hold 62 entries (the full POWER_MAX_INIT
allocation; name/description/gain
text/loss text/level/cost/stat/success rate in an eight-tuple, with the summon family and
class powers taking the zero-value special branch). The monster power table monster_powers
SHALL hold ninety-six slots (RF4/RF5/RF6 flags mapped to name/cost/great tier, empty slots
marked "(none)"), for symbiosis and possession to borrow (referenced by cmd5.c).

- **Anchors**: `src/tables.c:3176-3612` (racial powers), `:4267-4367` (monster powers)

### Requirement: Initial Quest Table And Assorted Tables

The initial quest table quest_init_tome SHALL hold twenty-six entries: the main line's four
parts (Dol Guldur at depth 70 starting TAKEN/Sauron 99/Morgoth 100/One Ring 99), the two
ultra endings (Apotheosis good and evil at 150 each), and the other plot lines with
their quests (Bree four, Lorien three including the wolf pack, Gondolin four, Minas
two, Khazad one, Other four, and the Random Quest dynamic-description slot with no plot
line) — each with
name/ten description lines/danger level/plot line pointer/HOOK_TYPE_C and an init hook
function. The assorted tables SHALL: the GF message table gf_names (ninety-two GF rows plus the
-1 terminator, WINDS_MANA with an empty message), the tval help text table tval_descs (about
forty-seven player-facing descriptions), the between-gate exit table between_exits with two
entries (pointing at each other's corresp and wilderness landing), the elven calendar
month_day/month_name nine segments (Yestare one day/Tuile fifty-four/Laire seventy-two/
Yavie fifty-four/Enderi three days/Quelle fifty-four/Hrive seventy-two/Coire fifty-four/
Mettare one day, totalling 365), the random artifact name pool artifact_names_list
(hundreds of Middle-earth words), and hexsym with the hexadecimal characters.

- **Anchors**: `src/tables.c:3617-4263` (quest table), `:4709-4804` (GF), `:4370-4625` (tval), `:4638-4654` (between), `:4659-4691` (calendar), `:1886-2488` (name pool)
