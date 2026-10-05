# defines Specification

## Purpose

Master list of global constants: the macro constants declared in `src/defines.h`, grouped
by domain. Numeric detail defers to the source at the anchor; constants cited by other
capabilities' requirements are located here. The source header notes that some values are
hard-bound to the savefile format, array sizes, external file formats, or game balance, and
that changing them crashes the game or misreads data — a re-implementation must re-judge
those constants rather than copy them.

## Requirements

### Requirement: World And Interface Scale

The dungeon scale constants SHALL define: block and panel sizes (11x11 / 11x33), the
22x66 display, the 66x198 dungeon cap, and the default detection radius of 25; the sight
cap of 20 and the casting range of 18; the temporary array capacity of 16384. The time
constants SHALL define: a day of 11520 turns, sunrise at the sixth hour, and the starting
year 2890. The savefile version is 104.

- **Anchors**: `src/defines.h:60-131` (scale), `:426-434` (fuel and sight), `:355-363` (calendar), `:48` (savefile version)

### Requirement: Player Constants

The player constants SHALL define: the experience and gold caps, the level cap of 50, six
hunger tiers (bloated 15000 down to starving 100), the regeneration rates and bases
(PY_REGEN_NORMAL/WEAK/FAINT plus the HP and mana bases), a 23-slot pack
with the overflow slot, twenty-eight equipment slots (indexes 24-51, weapons through tool,
hard-coded in the savefile), the six stat indexes, three sexes, six body parts, the stack
cap of 99, and the food and gold extremes. The race flags PR1/PR2 SHALL cover about twenty
entries (experimental, black-breath resistance, never stunned, vampire, undead, no cuts,
little gain from food, cannot worship, elf, semi-wraith, antimagic field, and more), merged
by the four layers (race/subrace/class/spec) via PRACE_FLAG.

#### Scenario: Equipment slots are hard-coded

- **WHEN** an implementation builds the equipment list
- **THEN** the slot indexes are bound to the savefile format and must be re-judged against
 the savefile version

- **Anchors**: `src/defines.h:459-533` (player base), `:555-619` (race flags and school keys), `:467-483` (hunger and regen)

### Requirement: Feature Indexes

The terrain feature indexes SHALL cover: 0 empty, floor/fountain/rune/door/staircase, the
four quest entrance and exit types, shafts, traps, the door family (0x20-0x2F), the secret
door/rubble/vein wall families, the permanent and wall families, explosive runes, the
pattern family (0x41-0x49), stores, the four quest walls, the extra terrains (water, lava,
dirt, grass, tree, mountain, sand and more from 0x54 up), the between gate/altar family/
marker grids/dirty water/monster traps/brambles/two-way road, deep water/glass wall/illusion
wall, and town landscaping such as trees and mountains. The wilderness terrain indexes
SHALL be twelve kinds plus the edge, with an eighteen-segment terrain table.

#### Scenario: Door family range

- **WHEN** the game judges whether a feature is a door
- **THEN** the judgement uses the index range from the family's first to last member

- **Anchors**: `src/defines.h:713-866` (feature indexes), `:884-900` (wilderness terrain)

### Requirement: Object Kinds And Subkinds

The object kind tvals SHALL cover bones/flask/energy/iron spike/staff/chest/scroll/corpse/
egg/miscellany/tool/instrument/boomerang/three ammo kinds/bow/digging/three melee kinds/
boots/gloves/headgear/crown/shield/cloak/soft armor/hard armor/dragon armor/light source/
amulet/ring/trap kit/totem/staff/wand/device body/scroll/potion/second potion/oil/food/
gold/random artifact/the two rune families/five book kinds. The subkinds sval of each major
kind SHALL be registered entry by entry (three shot and ammo tiers, the multiplier encoding
of the five bows, the seven diggers, the rod/axe/polearm/sword families, the twelve colors
of shield/helm/boot/robe/glove/soft and hard armor and dragon armor, the full light source,
amulet, and ring tables, the full staff-rod-device tables, fifty-four scrolls,
sixty-four potions, the second potions, foods, energies, corpse parts, spellbooks).

#### Scenario: Bow multiplier encoding

- **WHEN** a bow's subkind is interpreted
- **THEN** the subkind number implies the ranged multiplier (sling x2 up to heavy crossbow x4)

- **Anchors**: `src/defines.h:1383-1458` (tval table), `:1460-2105` (per-kind sval tables)

### Requirement: Artifact And Ego Indexes

The artifact indexes SHALL be registered piece by piece (light source/amulet/ring/dragon
armor/hard armor/soft armor/shield/helm and crown/cloak/gloves/boots/sword/polearm/hafted/
bow/staff/boomerang/instrument/the nine digging kinds, including the One Ring at index 13,
the Hammer of Morgoth, Anduril and more); the ego item indexes SHALL register about fifty
entries; the random artifact activation effects SHALL register about two hundred entries
(including each artifact's dedicated activation).

- **Anchors**: `src/defines.h:902-1106` (artifact indexes), `:1121-1174` (ego indexes), `:1176-1381` (activations)

### Requirement: Five Object Flag Groups

The object flags SHALL be five groups: group one holds parameter-dependent entries (the six
stats/mana/casting/stealth/search/infravision/digging/speed/attack count) plus the slay and
brand families; group two the sustain and resistance-immunity family (six sustains,
invisibility, life multiplier, four immunities, fear of fire, reflection, free action,
hold life, fourteen resistances); group three the aura/curse family (fire/electric aura,
auto-curse, decay, no-teleport/no-magic/wraith, heavy curse, easy know/hidden type/visible
modifier, instant artifact, levitation, light radius, see invisible, ordinary artifact, slow
digestion, regeneration, extra might/shots, the four elemental immunities, activation,
experience drain, teleport, aggravate, blessing, the three curse tiers); group four the ToME
extensions (no-attack, prophecy, black breath, charging, flight, curse of Morgoth, dual
wielding, object leveling, cloning, special generation, mountain climbing, the four device
parameters, fountain, the four antimagic tiers, easy use, hell immunity, charged, ultimate,
auto-identification, light radius tiers two and three, fuel, experience accumulation, no-drop,
no-recharge); group five temporary items/drain mana and HP/slay demon and undead/critical
blows/multi-hued/wounding/full name/luck/immovable/spell container/maze resistance/activate
no-wield/water and magic breathing/cast while held.

#### Scenario: Parameter-dependent masks

- **WHEN** the game judges whether a flag scales with the parameter value
- **THEN** the parameter masks of groups one and five decide

- **Anchors**: `src/defines.h:2762-2982` (five groups and masks), the fourteen ESP bits `:2952-2966`

### Requirement: Nine Monster Flag Groups

The monster flags SHALL be nine groups: group one general (unique/quest monster/sex/transparent
symbol/forced depth/full HP/sleep/friend escort/no-attack no-move/two random-move tiers/
nine drop entries); group two abilities (stupid and smart/can speak/reflect/invisible/cold
blood/empty mind and weird mind/death orb/regenerate/shapechange/multi-hued/powerful/frightening/
fire and electric aura/opens and bashes doors/passes and destroys walls/bumps and tramples/
picks up and destroys objects/eight reserved mind slots); group three race resistances (orc,
troll, dragon, demon, undead, evil, animal, thunder lord, good, nonliving/afraid of light,
afraid of stone/weak to fire, weak to cold/five immunities/teleport/nether, water, plasma
resistance/four state immunities); group four innate and breaths (shriek/breed/summon animal/
arrow/the four arrow tiers/twenty-two breaths/nether, plasma, poison, change-shape orbs);
group five spells (ten orbs/drain mana/mind blast/brain smash/the four wound tiers/eleven
bolts/missile/scare, blind, confuse, slow, hold); group six special spells (haste/doom/
self-heal/summon animal/blink/teleport the four ways/darkness/lay trap/forget/raise dead/
twenty summons); group seven movement and ecology (swims, flies/friendly pets/mortal/spider/
Nazgul/curse of Morgoth/possessor/never moving/untargetable/harassing/special/neutral/drops
artifacts, drops random artifacts/player controlled/cannot be stolen from/spirit/melee
immune); group eight habitats (dungeon/town/shore and sea/waste/forest/volcano/mountain/
grass/no-cut/otherworld/comical/Angband/all wilderness); group nine drops and ecology (drops
corpse and skeleton/carries light/mimic/carries eggs/can be carried/acid-prone, electric-prone,
poison-prone/eats trees/dragon escort/Doppelganger/depth only/special/no generation).

#### Scenario: Smart spell masks

- **WHEN** a cornered monster picks a spell
- **THEN** the smart masks of groups four, five, and six restrict it to controllable and
 summon spells

- **Anchors**: `src/defines.h:3181-3462` (nine groups), `:3468-3512` (smart and summon masks)

### Requirement: Effect Types And Projection Flags

The projection effect types GF SHALL register one hundred eleven effect id slots (ids 0
through 110, of which ninety-three carry named GF_ constants: fifteen elements
and their variants, nine old control effects, six banishments, arrow, nuclear, holy fire,
hellfire, disintegration, charm, mind, telekinesis, gate closing, identification, reforging
and more), with the smart masks and the bolt masks attached. The projection flags SHALL
cover seventeen entries: jump/beam/pierce/stop at monster/hit grid, object, and monster/hidden
visual/in view/meteor shower/burst/panel/everyone/through walls/mana path/drain mana/in place.
The summon types SHALL register about thirty-eight kinds.

- **Anchors**: `src/defines.h:2583-2678` (GF table), `:2149-2165` (projection flags), `:2542-2579` (summon types)

### Requirement: Grid Flags And Miscellany

Fifteen grid flag bits; eleven monster extra-flag bits (quest/possession/newborn/placid/
marked/no-drop and the permanent mask among them); seven object identification bits; ten
pseudo-identification tiers; trap flags (chest/door/floor/color change/the four level tiers);
three stat-drain tiers; the player update/redraw/window flag groups; two notice bits;
fifty-three building action ids (fifty-one named, two reserved slots); eight quest states with twenty-six quest indexes and seven
plot lines; seventy-eight hooks of C and Lua types; six god bits with the praying macros;
skill indexes up to 59 (fifty-eight named constants, index 38 reserved) plus the 50000 cap with thousand steps and the exclusive marker;
twenty-three music constants (ids 0-22); six fate kinds; the two rune families; dungeon indexes with four
rule modes; inscription execution flags; four shield options; sixteen colors; six graphics
modes; sixty-five sounds; four road directions; a power table of POWER_MAX_INIT (62) slots; three
spellbinder triggers; two victory states; eleven abilities; three store states with eleven
flags.

- **Anchors**: `src/defines.h:2118-2134` (grid flags), `:2746-2759` (monster extra flags), `:2733-2739` (identification bits), `:4016-4069` (building actions), `:4075-4090` (quest states), `:4450-4531` (hooks), `:4573-4646` (skills), `:4664-4675` (gods), `:4307-4320` (store flags), `:3865-3983` (colors, graphics, sounds)
