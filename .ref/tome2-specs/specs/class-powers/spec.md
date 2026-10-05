# class-powers Specification

## Purpose

Class skill commands: `src/cmd7.c` carries the active command implementations of the skill
systems - mindcraft/mimicry/beast-master/alchemy (including the item-infusing and artifact
empower flows)/prayer toggling/power-mage random spells/archer ammo making/necromancy/rune
crafter/unbeliever anti-magic/summoner totems/blade charge/symbiosis/boulder making - plus
the shared get_magic_power menu and brand_ammo. Each skill's value source is specified in
specs/skills/spec.md; the spell effect routines (fire_ball and friends) are specified with
the spells1-2 material.

## Requirements

### Requirement: Bookless Power Menu

`get_magic_power` SHALL filter the available entries by the power table and the skill
level, select with a lowercase letter, show the description with uppercase, and list with
`*`; the failure rate folds live - base failure minus three times the skill excess minus
three times the stat-table fold, raised by five times the mana gap, floored by the stat
table, stun adding fifteen or twenty-five, capped at 95. `mindcraft_info`/`mimic_info`/
`necro_info`/`symbiotic_info` SHALL produce each form's live parameter footer (damage
dice/range/radius/duration banded by level; the mimic Mimic form takes the outer cloak's
pval2 plus the skill folded to 70).
**Discrepancy:** the Mimic footer folds the duration skill to 70, while the actual
transformation in `do_cmd_mimic_lore` folds it to 1000.

- **Anchors**: `src/cmd7.c:127-336` (menu), `:22-110` (mindcraft and mimic footers),
 `:5474-5501` (necro footer), `:7516-7535` (symbiote footer)

### Requirement: Mindcraft

Mindcasting SHALL be barred by both the anti-magic field and the anti-magic shell, and by
confusion; a failed use runs the backfire table with probability half the failure rate -
five percent total forgetting, ten percent images, thirty percent confusion, forty-five
percent stun, and the tail (ten percent) an uncontrolled mana storm (centered on the
caster, GF_MANA at radius 2+level/10,
mana minus level x max(1, level/10)).

The success side has twelve powers: Precognition (banded detection/mapping/telepathy),
Neural Blast (a level-times-two percent beam, otherwise a zero-radius ball), Minor
Displacement (below level 25
a plain short teleport; from level 25 a between gate - the failure rate is the level
squared folded to one half,
failure costs 100 energy with a random landing square), Major Displacement (29 adds
banish), Domination (30 switches to group charm), Fist of Force, Character Armour (a
shield plus banded five-element resistance), Psychometry (40 switches to full
identification), Mindwave (25 switches to group mindblast), Adrenaline Channeling (35
switches to greater heroism), Psychic Drain (when the drain ball connects the caster also
loses a random 1-150 energy), Telekinesis. Overdrawing mana SHALL faint-paralyze five
times the gap plus
one, with a 50% chance of WIS decay (15+1d10 points, a quarter of the rolls permanent).

- **Anchors**: `src/cmd7.c:343-760` (mindcraft flow, backfire `:424-465`, overdraft
 `:727-753`)

### Requirement: Mimicry

Mimic casting SHALL be barred by the dual ban and confusion; the Mimic form calls
`do_cmd_mimic_lore` - blind or no light bars it; already mimicking reverts; otherwise the
outer slot must hold a cloak with SV_MIMIC_CLOAK; the failure rate goes through the Lua
`get_mimic_info` as level x 3 minus the skill folded to 150 minus three times the
dexterity-table fold, and above 75 asks for confirmation; on failure a saving throw
(skill_sav) miss turns the player into an Abomination for 30 turns, success lasts the
cloak duration plus the skill folded to 1000; the additional-limb trio (legs/wall/arms,
exclusively switched; mimic_extra's high 16 bits are the timer, the low 16 bits the flags,
capped at 10000, the same flag stacking duration, PU_BODY refreshing) and the
invisibility form (10+level+d20+to_s, power fifty). The first call SHALL hang the
HOOK_FORBID_TRAVEL hook (additional limbs bar wilderness travel).

- **Anchors**: `src/cmd7.c:763-879` (lore and hook), `:885-1178` (command, overdraft
 damages dexterity)

### Requirement: Beast-Master And Prayer

`do_cmd_beastmaster` SHALL, while the pet count is below twice the level, roll 80 minus
level minus charisma minus to_s and, below twenty, grant one friendly beast (of a
depth-random half level). `do_cmd_pray` SHALL exhort the godless; with a god it toggles
the praying state with a full refresh (including PR_PIETY and the screen redraw) and
costs 100.

- **Anchors**: `src/cmd7.c:1185-1221` (beast-master), `:4404-4426` (prayer)

### Requirement: Alchemy - Recipes And Learning

Alchemy SHALL be restricted to leather gloves and barred under confusion. The learning
rules SHALL: `alchemist_learn_object` records the object's know on great identification;
an artifact under great identification fills all six known_artifacts bitmaps (afterwards
the flag can be self-forged); ego learning spreads by the suffix to the same-name family;
`alchemist_gain_level` fires once per alchemy skill level - level 0 learns the potions of
Detonation, level 5 grants the twelve elemental egos, level 10 grants the digging ego
for ironman-rooms players, level 50 unlocks TR5_TEMPORARY knowledge, and every four levels
learns all recipes of a quarter of the level plus one or below (`alchemist_check_level` with
alchemist_gained guarantees exactly once per level). `alchemist_exists` looks up the
recipe table (tval 1 is an ego entry; the wand-staff pair allows the ego-extraction
special case).

- **Anchors**: `src/cmd7.c:3404-3464` (learning), `:3470-3611` (level chain),
 `:2396-2419` (existence)

### Requirement: Alchemy - Infusing And Extracting

Infusing (Power) SHALL filter through `item_tester_hook_empower` (the plain bases of each
tval, ego-izing from level 15, double ego and dragon scale mail from 25, artifact finalize
needing TR4_ART_EXP and not ULTIMATE, the AB_CREATE_ART power gate); the recipe is picked
(`alchemist_recipe_select`, a three-color list - green craftable/red missing
materials/white paging, `*` shows the recipe, bottle-to-potion tval conversion); an ego
infusion rolls its pval from 1 to max_pval minus one (the wand/ring/amulet/staff group keeps
the original pval set, random books keep their spells); the failure rate folds by the
ego cost (capped 50000, double ego accumulates) plus gold minus failure reduction (minus
one per twentieth of the object value, the Philosopher's stone to zero), and a failure
explodes 3d(object level minus skill) self-damage; ammo processes the whole stack by
material.

Extracting (Extract/Leech) SHALL filter through `item_tester_hook_extractable` (not an
artifact, not cursed, recharged wands and staves barred - TR4_RECHARGED); a charged wand
or staff discharges first, then extracts the ego; the extract produces essences by the recipe table
(one-third chance of a halved amount for insufficient skill), non-ego potions produce
empty bottles, random books keep their spells, ammo keeps pval2, instruments keep
pval2; a one-third chance learns the recipe (with skill above nine, the full description
comes with it); Leech loops the extraction over the whole stack. Recharging (Recharge)
costs one recipe essence plus one charge. The recipe book (B) is a two-level menu sorted
by known essences and known egos/objects (`alchemist_recipe_book`, --MORE-- paging).

- **Anchors**: `src/cmd7.c:3617-4398` (main flow), `:2688-2781` (material check and
 consumption), `:2786-2833` (recipe display), `:2849-3399` (menu and recipe book),
 `:4326-4383` (recharge)

### Requirement: Alchemy - Artifact Forging

`do_cmd_toggle_artifact` SHALL start the artifact: without the Philosopher's stone it
confirms permanently subtracting ten hp_mod and spends skill-folded magic essences; extra
stacks are cut to one; both egos are cleared, the inscription records Becoming, the base
object's flags merge wholesale into art_flags, and TR4_ART_EXP is set.

`do_cmd_create_artifact` SHALL run the six flag-group interaction (a-f:
parameters/miscellaneous/brands/resistances-immunities/ESP-curses/activations; g is
disabled; h
shows the recipe; i finishes) - flags colored in three states (yellow newly picked/white
pickable/green already set/red set but unknown/dark blue invisible/light dark beyond the
skill); unknown flags lock the pval (lockpval); the experience cost goes through
`get_flags_exp` (pval-dependent entries by a pval^2/4+pval factor, existing entries
subtract their old value, the Philosopher's stone charges a quarter); the activation group
goes through `select_an_activation` as a scrolling list (`?` shows the activation
details); the wrap-up validates experience and materials (`artifact_display_or_use` -
essences accumulated by recipe, objects validated by `check_artifact_items` for the pval
sign and the corpse-race flag six group); on pass, the flags are set, TR3_ACTIVATE and
xtra2 are added, the four-element IGNORE and SHOW_MODS are added, and a temporary
item gets a timeout of skill squared times three; the name (of/quote formatting)
goes into a quark, OBJ_FOUND_SELFMADE is recorded, and IDENT_MENTAL plus STOREB are set;
the keep-growing question - affirmative spends skill-folded essences and sets
TR4_ART_EXP, negative clears the experience; finally the whole pack is optimized in
reverse order (a historical note guarding against pointer shifts).

- **Anchors**: `src/cmd7.c:2613-2679` (starter), `:1957-2389` (forging main flow),
 `:1255-1327` (flag coloring), `:1344-1440` (experience folding), `:1451-1653` (object
 validation), `:1657-1788` (essence summary), `:1791-1896` (activation selection),
 `:1899-1954` (magic essence)

### Requirement: Power-Mage Random Spells

`do_cmd_powermage` SHALL honor the dual ban; the spell batch is picked (ten per batch,
uppercase to browse, `/` renames, `-` annotates); the failure rate goes through
`spell_chance_random` (the level plus 25 minus three times the thaumaturgy skill folded,
among the shared formulas); a failure burns mana and a turn (the mage staff eighty), and
the insane read their phantom-cloud text from sfail.txt; success dispatches by the
projection flags - BEAM/STOP need a direction (a locked target beams through), BLAST at
the caster, VIEWABLE through project_hack, METEOR_SHOWER through project_meteor, the rest
through the regular project. The spell_num/random_spells table generation is specified
with the spells2/xtra material.

- **Anchors**: `src/cmd7.c:4432-4467` (failure rate), `:4515-4755` (batch picking),
 `:4758-4901` (casting)

### Requirement: Archer And Boulder

`do_cmd_archer` SHALL be barred under confusion and blindness; ammo making (skill gates
0/10/20 unlock arrows and bolts in tiers) - making shots requires adjacent rubble (the
wall becomes mud), making arrows and bolts consumes one TV_JUNK/TV_SKELETON; the product
is fifteen to thirty items, the sval folds by m_bonus to the depth, apply_magic at full
quality with 20% force-might, a 90 discount, OBJ_FOUND_SELFMADE. `do_cmd_set_piercing`
toggles the piercing flag. `do_cmd_create_boulder` applies wall_to_mud to adjacent
granite/vein walls and yields two to five boulders (a 90 discount).

- **Anchors**: `src/cmd7.c:5216-5439` (archer), `:5444-5470` (piercing),
 `:7941-7984` (boulder)

### Requirement: Necromancy

Necro casting SHALL honor the dual ban plus the confusion ban and take CON as its stat;
failure backfires - ten percent self-undeath (necro_extra2 records the required kill
count, hp is refilled to the new maximum), thirty percent raises undead (the high tier from
level 30), otherwise 5dlevel self-damage. The success side has six powers: Horrify (banded
bolt/beam/ball/full-screen GF_STUN plus GF_TURN_ALL double fire), Raise Dead (a GF_RAISE
ball of radius one plus to_s folded to two plus level/10), Necromantic Teeth (a temporary
vampiric weapon - generated with a k_allow_special exception, TR5_TEMPORARY, lasting
d(100+four times level) plus 200 plus three times level), Absorb Soul, Vampirism (1 plus
to_s folded to two plus level/15 drain strikes), Death (confirmation - a GF_DEATH
bolt kills the enemy and turns the player undead, the required kill
count being the level plus a random half level folded).
**Discrepancy:** the confirmation message promises one DP left, but the code refills hp
to the undead-reduced maximum instead. Overdrawing damages CON.

- **Anchors**: `src/cmd7.c:5507-5811`

### Requirement: Rune Crafter

Rune combination SHALL: pick a TV_RUNE1 to set the spell type (GF), stack several
TV_RUNE2 sub-runes (exclusively deduplicated), and set the mana investment. The power
folds `rune_calc_power` (plus three, then 37 x integer square root folded to ten, one
third of it as the dice count, to_s folded to half as the multiplier); the failure rate
`spell_chance_rune` (the sub-runes POWER_SURGE/ARMAGEDDON/SPHERE/RAY each add
four/three/two/one times five plus power, folded by the dexterity table); the execution
`rune_exec` SHALL: widen the mana cap by the sub-runes' level/5 and truncate proportionally
when short; a failure plays the insanity phantom-cloud text; the effect follows the
sub-rune combination - POWER_SURGE the whole visible area, ARMAGEDDON a meteor rain
(radius power folded to eight, capped ten), SPHERE a ball at the caster, RAY a beam, ARROW
a targeted bolt, SELF a targeted bolt at the caster (setting unsafe); sub-runes lacking a
direction ask for one (a target with key five works). Persistence SHALL: memorize
(do_cmd_rune_add_mem into the rune_spells table capped at MAX_RUNES, renamable), delete
(do_cmd_rune_del shifts the rest forward), carve into a runestone (do_cmd_rune_carve
destroys the used rune stones into runestone pval/pval2/pval3, costing 400 energy), cast
from a stone (do_cmd_runestone at 75% cost), cast directly (do_cmd_rune at full cost);
`do_cmd_runecrafter` dispatches over five keys; `do_cmd_rune_cast` casting a memorized
form requires `test_runespell` to verify the rune is in hand.

- **Anchors**: `src/cmd7.c:5898-5964` (power and failure), `:5970-6144` (execution),
 `:6150-6265` (rune picking), `:6268-6309` (direct cast), `:6315-6505` (batch picking),
 `:6512-6571` (memorized cast), `:6577-6658` (stone cast), `:6664-6903` (memorize,
 carve, delete), `:6906-6986` (dispatch)

### Requirement: Unbeliever And Summoner

`do_cmd_unbeliever` SHALL: toggle the magic interruption (antimagic_extra toggles
CLASS_ANTIMAGIC, from skill level twenty, PU_BONUS); detect traps (from level 25
detect_traps, from 35 with touching off doors and traps).

The summoner SHALL: extract a totem (from an intact corpse or skeleton only - a unique
always makes a true totem, a normal one asks about a half-body totem; the failure rate is
magik(monster level minus the summoning skill); the product is TV_TOTEM with sval one or
two and the pval recording the monster race, costing 100 energy); summon (a true totem
goes through `summon_true` - a unique's totem is always consumed and its friendly chance
is the summoning skill x 70/(level+1); a normal totem is consumed only when the summoning
skill is zero or magik(level x 25/skill) fires, with the friendly chance the summoning
skill x 130/(level+1); placement tries 40 random squares around the player;
bypass_r_ptr_max_num lifts the race cap and the summoned monster gets MFLAG_NO_DROP; a
half-body totem is always a pet with MFLAG_PARTIAL); the dispatch honors the dual ban and
the confusion/blindness ban.

- **Anchors**: `src/cmd7.c:6989-7071` (unbeliever), `:7093-7185` (extract),
 `:7188-7300` (true totem), `:7303-7449` (summon and dispatch)

### Requirement: Symbiosis, Possession And Miscellany

`do_cmd_symbiotic` SHALL honor the dual ban plus the confusion ban and take INT as its
stat, with nine powers - Hypnotise (an adjacent never-moving pet or companion only,
turned into a
TV_HYPNOS object recording race/current hp/cap/experience/level, monster removed),
Release (the HYPNOS object is re-released as a pet restoring experience and hp, a level
mismatch errors), Charm Never-Moving (a GF_CHARM_UNMOVING bolt at power three times the
level), Life Share (the player's and pet's hp percentages are shared equally),
Minor/Major Powers (forwarded through use_symbiotic_power without cost), Heal Symbiote
(15 plus the skill folded to 35 percent of the max hp), Summon Never-Moving Pet
(SUMMON_MINE), Force Symbiosis (any monster is point-selected to borrow its power table).
Overdrawing damages CHR.

`do_cmd_possessor` SHALL honor the dual ban; the `R` form calls use_symbiotic_power (the
possession skill folded to 100 above the monster level gives the great tier); mana
running dry loses control and leaves the body (a failed cursed body bounces back with one
hp); the `I` form forwards leaving and embodiment. `do_cmd_blade` starts the charge (The
Rush, lasting 2 plus the level folded to two plus a roll of turns);
`use_ability_blade` announces the dodge-rate wording (dodge_chance minus five sixths of
the depth, in six tiers); **Dead code:** `brand_ammo` (no caller anywhere - Cubragol's
bolt branding goes through `brand_bolts` in `src/cmd6.c`) enchants the first qualifying
ammo stack with a 50% chance of a flame or frost ego; `summon_monster` is barred in
arenas and summons friendly reinforcements.

- **Anchors**: `src/cmd7.c:7541-7936` (symbiosis), `:5108-5197` (possession),
 `:7455-7511` (blade), `:4985-5075` (ammo branding), `:5081-5101` (reinforcement)
