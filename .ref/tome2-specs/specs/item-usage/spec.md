# item-usage Specification

## Purpose

Item usage: `src/cmd6.c` carries the four big use actions — eating, drinking, reading
and aiming — namely: whole corpses eaten and cut/cured, the full potion tables of both
families, the full scroll table (including parchments and maps), staff and wand
activation by name, rod charging and rod-tip assembly, the unified artifact activation
machine (the activation-effect resolution chain + the giant ACT table + artifact
exclusives), cursed armor and weapons, and spring drinking plus bottle filling. The
item selection framework and the Awareness experience are referenced with the item
selection system; mage staff embedded spells go through `rune_exec` into the rune
system.

## Requirements

### Requirement: Corpse Eating

Eating a corpse SHALL first check the remaining meat: for species that leave a
skeleton, a corpse at or below 3/5 (60%) of the race weight has no meat left; for all
other species the threshold is 7/20 (35%).

- Each nibble of a whole corpse subtracts 10 from both pval and weight; the corpse is
 not destroyed and can be eaten again; a head is nibbled the same way ("You feel
 rather sick.").
- Uncured rotten meat (timeout zero) announces raw meat ("Ugh! Raw meat!"); cured
 meat tastes good; plain meat cut from a corpse, when raw with weight above pval and
 without poison resistance, rolls poison by the remaining difference.
- `corpse_effect` SHALL fold the immediate effects over the monster's four-blow
 table (damage halved by the eaten share): poison/blindness/confusion/fear/
 paralysis each attach per resistance; acid and fire burns (acid immunity
 converts into temporary resistance); the six stat reductions (LOSE_ALL also
 knocks the pval back to 1); sanity damage; the four experience-drain tiers
 (hold life fully resists half the time; a resisted-but-failed hold loses a
 tenth of the drain; without hold life the full drain lands).
- **Quirk:** the fire-burn branch's condition is the negation of the acid
 branch's — a fire-immune character takes the hit, while a non-immune
 character gains the temporary fire resistance.
- Breath residue SHALL discharge with power equal to one fifth of the overweight
 portion (capped at one fifth of the race weight), and a whole corpse discharges it
 by chance while nibbled or while cut — the four elements capped at 1600, poison at
 800, nether at brpow/6 capped 550, chaos at 600, disenchant at 500, plasma at 150;
 a family whose breath does not discharge instead grants 10-19 turns of temporary
 resistance to it.
- When nothing harmful happened, eating SHALL grant the monster kind's good flags
 (the five resist families for 10-19 turns, nether resistance giving protection
 from evil of `rand_int(25)` plus three times the monster level, instant clearing of
 fear/stun/confusion) plus friendly summons for the RF6 summon flags
 (`S_MONSTERS` summons eight).
- Jelly-symbol (`j`) races with `RF3_IM_ACID` have a special guaranteed 8d8 acid
 hit ("acidic food"); the `ijkmS,` symbol races with `RF3_IM_POIS` poison
 the eater unless resistant or opposed.

#### Scenario: Skeleton species eaten bare

- **WHEN** a corpse of a skeleton-leaving species sits at or below 60% of the
 race weight
- **THEN** it has no meat left and cannot be eaten further

#### Scenario: Acidic jelly food

- **WHEN** a jelly-symbol (`j`) race with `RF3_IM_ACID` is eaten
- **THEN** a guaranteed 8d8 acid hit lands ("acidic food")

- **Anchors**: `src/cmd6.c:98-904` (corpse_effect), `src/cmd6.c:1317-1429` (eating
 dispatch)

### Requirement: Food And Meat Processing

Eating food SHALL pass the `HOOK_EAT` hook first, then consult the food table: the
mighty haunch adds 70 to hp_mod; poison/blindness/fear/confusion/hallucination/
paralysis each attach per resistance; the six poisonous foods (weakness, sickness,
stupidity and their kin) deal 6d6-10d10 and drain the matching stat; the eight cure and
restore kinds; the fortune cookie (draws from the chainswd/error/death/rumors
word-lists); rations, biscuit and jerky; the slime mold (2% chance of the grow-mold
power); bread (cures poison, heals wounds, feeds); alcohol (returns the bottle);
Athelas (removes the three states and breaks Black Breath).

- Eating a meat cut poisons by the remaining difference when raw.
- Curing `do_cmd_cure_meat` SHALL convert the soak time per potion (salt water 200,
 confusion 80, slow poison 20, cure poison 45 per bottle; poison halves the pval,
 death divides it by ten), scale a spoiling piece down by pval against weight
 twentieths, and cap the timeout at pval.
- Cutting `do_cmd_cut_corpse` SHALL leave the bone per the skeleton threshold and
 take meat as race weight plus a tenth minus the bone (capped at 100), run the
 cutting variant of `corpse_effect` (discharge always checked), then produce meat
 cuts (count = meat/10, decay timer 1000 plus d1000).
- Nutrition SHALL be: normal food the full amount, food-refusing races 1/40, and
 vampires do not eat at all (prompted to drink blood).

#### Scenario: Vampire refuses food

- **WHEN** a vampire tries to eat food
- **THEN** the character does not eat at all and is prompted to drink blood

#### Scenario: Slime mold surprise

- **WHEN** a slime mold is eaten and the 2% draw hits
- **THEN** the grow-mold power fires

- **Anchors**: `src/cmd6.c:923-1504` (eating), `src/cmd6.c:1510-1618` (cutting),
 `src/cmd6.c:1626-1789` (curing)

### Requirement: Potions

Quaffing SHALL pass the `HOOK_QUAFF` hook first. The traditional potion table SHALL
cover about forty-five svals:

- thirst quenchers; slowness; salt water (vomit down to starving, cure poison,
 paralysis 4); poison and blindness;
- booze (confusion and hallucination; a 1-in-13 blackout — one third lose all
 information, otherwise whole-map darkness plus teleport 100 plus darkness again);
- sleep; lose memories (a quarter of experience, hold life exempts); ruination
 (10d10 plus a 25% decay of each of the six stats); the six stat reductions;
- detonations (50d20 plus stun 75 plus 5000 cuts); death (5000 damage);
- infravision and see invisibility; slow poison, cure poison and boldness; speed;
 resist heat and cold; heroism and berserk strength;
- the healing ladder (light 2d8 and cut -10, serious 4d8 and the cuts halved
 less 50, critical 6d8, healing, star healing);
- life (experience restoration plus 5000 plus a full clear plus stat restoration
 plus Black Breath broken); restore mana full; restore experience;
- the six sustains and the six stat gains; augmentation;
- enlightenment (maps the level); star enlightenment (map plus dual stat gain plus
 six detections plus full identification plus self knowledge); self knowledge;
- experience (half plus ten, capped at 100000); the five-family resistance;
 curing; invulnerability 7+d7; new life (reroll); blood of life (extra life +1);
- mutation (random corruption); invisibility (30+d30, power 35); learning (skill
 points 4-10 plus the luck conversion).

Secondary potions SHALL: the mimic potion (random duration via Lua per pval2, level
folded by two thirds) and the four sanity cures (4d8/8d8/12d8/10d100).

After drinking SHALL: record Awareness and experience; alchemists keep the empty
bottle; food increases by pval; the potion is destroyed.

#### Scenario: Booze blackout

- **WHEN** booze is quaffed and the 1-in-13 blackout draw hits
- **THEN** one third of the time all information is lost, otherwise the whole
 map darkens, the player teleports 100, and darkness falls again

#### Scenario: Alchemist keeps the bottle

- **WHEN** an alchemist finishes drinking a potion
- **THEN** the empty bottle is kept (for other characters the potion is
 destroyed)

- **Anchors**: `src/cmd6.c:1804-2500` (quaff_potion), `src/cmd6.c:2506-2617` (entry)

### Requirement: Springs

A spring SHALL keep its remaining amount in special2 and its potion kind in special
(a kind not exceeding `SV_POTION_LAST` is a traditional potion, otherwise a secondary
potion minus the base offset).

- Drinking SHALL ask between drinking and filling: drinking converts the pval and
 goes through `quaff_potion`, the remainder decrements, an empty spring becomes a
 dry one, and a tasted potion records CAVE_IDNT.
- Filling SHALL trade an empty flask for an equal amount of potion (capped by the
 remainder); once the spring is identified, its product is known too.

#### Scenario: Spring runs dry

- **WHEN** the last potion is drunk from a spring
- **THEN** the empty spring becomes a dry one and the tasted potion kind
 records `CAVE_IDNT`

#### Scenario: Filling a flask

- **WHEN** a spring is filled from with an empty flask
- **THEN** the flask is traded for an equal amount of potion, capped by the
 remainder

- **Anchors**: `src/cmd6.c:2623-2692` (drinking), `src/cmd6.c:2698-2790` (filling)

### Requirement: Cursed Armor And Weapons

A cursed armor SHALL get a 50% exemption when the item is an artifact; on failure it
is exchanged for `EGO_BLASTED` — armor class zeroed, modifiers negative 2d5, all
flags cleared, and the curse recorded. A cursed weapon works the same way into
`EGO_SHATTERED` — to-hit and to-dam negative 2d5.

#### Scenario: Artifact armor exemption

- **WHEN** a cursed armor that is an artifact triggers the curse exchange
- **THEN** a 50% exemption roll decides whether it is spared

#### Scenario: Weapon shatters

- **WHEN** a cursed weapon fails its exemption roll
- **THEN** it is exchanged for `EGO_SHATTERED` with to-hit and to-dam at
 negative 2d5

- **Anchors**: `src/cmd6.c:2796-2922`

### Requirement: Scrolls

Reading SHALL be refused while blind, dark, or confused. The `HOOK_READ` hook can
take over (returning whether the scroll is spent and identified). The traditional
scroll table SHALL cover about forty kinds:

- mass resurrection (uniques all restored to max_num); the body-leaving scroll
 (after confirmation; without a corpse the body is discarded); reset recall;
 divination (turns over unknown fates and makes them known);
- darkness (3+d5 blindness plus a dark room); aggravate monster; curse armor and
 curse weapon;
- the three summonings (monster and undead hostile, mines friendly); trap creation;
 phase door; teleport; teleport level; word of recall (special levels confirm);
- identify and star identify (cancelling costs nothing); remove curse and star
 remove curse;
- the four enchantment directions plus the star enchants (armor d3+2, weapon
 d3+d3); recharging (60); light and Map; the detection scrolls (gold, item, trap,
 door, invis);
- satisfy hunger; the three blessing tiers; monster confusion (the glowing hand,
 one use); protection from evil; rune of protection; trap/door destruction;
- star destruction (destroys a radius-15 area; quest levels only tremble); dispel
 undead (60); genocide and mass genocide; acquisition and star acquisition;
- the fire/ice/chaos trio (large balls centered on the reader plus self-damage —
 note: the fire scroll's self-damage expression always takes 20 because of ternary
 precedence, an implementation quirk); the rumor scroll; the artifact scroll.

Parchments SHALL: with sval at or above 200 be a wilderness map (the book-N.txt
four-tuple reveals block by block), otherwise display the `book-N.txt` file and not
be spent (sval at or above 100 unlocks the matching grid inscription knowledge). Alchemists keep a
blank scroll after reading.

#### Scenario: Reading while blind

- **WHEN** a scroll is read while blind, in the dark, or confused
- **THEN** the reading is refused

#### Scenario: Parchment display

- **WHEN** a parchment with sval below 200 is read
- **THEN** the `book-N.txt` file is displayed and the parchment is not spent

- **Anchors**: `src/cmd6.c:2944-3732`

### Requirement: Staffs And Wands

Staffs and wands SHALL share the flow: disabled under an anti-magic field; a floor
stack refuses use; name lookup supported; entering stick mode (pval3 low 16 bits are
the level conversion, high 16 bits the cap, injected into the Lua globals).

- The failure rate SHALL come from Lua `spell_chance(pval2)` (EASY_USE one third);
 staffs get an extra bottom-rung rescue (the USE_DEVICE chance evens it out; wands
 have no such rescue — an implementation difference).
- An empty staff records `IDENT_EMPTY`; execution goes to Lua
 `activate_stick(pval2)`, which returns obviousness and whether a charge is spent —
 obvious means Awareness, spending removes one charge.
- A staff is split out of its stack as a single piece to keep using, with
 announcements; the remaining text follows the stack count. A wand grabs its sval
 before execution (for the activation to reference).

#### Scenario: Anti-magic blocks the stick

- **WHEN** a staff or wand is used inside an anti-magic field
- **THEN** the use is disabled

#### Scenario: Empty staff sense

- **WHEN** an empty staff is used
- **THEN** `IDENT_EMPTY` is recorded for it

- **Anchors**: `src/cmd6.c:3737-3748` (stick mode), `src/cmd6.c:3758-3937` (staff),
 `src/cmd6.c:3960-4110` (wand)

### Requirement: Rods And Rod Tips

The rod code lives in `src/cmd6.c` (not in `src/cmd5.c` as in other Angband
lineages). Zapping a rod tip (`TV_ROD`) SHALL go through assembly: the target is
limited to an empty rod base of sval `SV_ROD_NOTHING` (disabled under anti-magic);
the tip's mana cost is halved through the base's CHEAPNESS flag; an insufficient base
mana pool (pval2) rejects; on success the rod's pval is set to the tip's sval and the
tip is destroyed.

- Zapping a rod base SHALL: be disabled under anti-magic; require a direction per
 the tip sval threshold (svals below `SV_ROD_MIN_DIRECTION` aim freely, and the
 detect-trap/havoc/homecoming three tips also aim freely); unidentified rods always
 ask for a direction; FAST_CAST halves the energy cost;
- the failure rate is device skill minus the tip level (confusion halves it, the
 level taken is capped at 50, EASY_USE multiplies by ten, USE_DEVICE rescues);
- an insufficient mana pool rejects the charge, casting deducts the cost (CHEAPNESS
 halves it); the effect table covers about twenty-five kinds (homecoming trump,
 detect trap/door, identify, recall, light, mapping, full detection, probing,
 curing, healing 500, restoration of experience and stats, speed, teleport away,
 disarming, light bolt, sleep monster, slow monster, drain life 75, polymorph, the
 four bolt and ball families with fixed damage, havoc `call_chaos`); an unknown tip
 goes to `HOOK_ZAP`; a cancelled use (for example a cancelled identification)
 refunds the deducted mana.

#### Scenario: Tip onto a bare base

- **WHEN** a rod tip is zapped at an empty `SV_ROD_NOTHING` base with enough
 mana
- **THEN** the rod's pval is set to the tip's sval and the tip is destroyed

#### Scenario: Cancelled use refunds

- **WHEN** a rod charge is cancelled (for example a cancelled identification)
- **THEN** the deducted mana is refunded

- **Anchors**: `src/cmd6.c:4155-4236` (assembly), `src/cmd6.c:4242-4727` (zapping
 the rod)

### Requirement: Artifact Activation

Activation SHALL be limited to identified items carrying `TR3_ACTIVATE` (items with
the `TR5_ACTIVATE_NO_WIELD` flag may activate from the pack); the failure rate
matches the rod rule; a recharging item refuses (the mage staff checks both
timers, eggs instead toggle development — activating a paused egg (nonzero timeout)
resumes incubation and zeroes it, activating an incubating egg (timeout zero)
pauses it with -1); the `HOOK_ACTIVATE` hook can take over completely; afterwards
the `activation_aux` unified machine runs.

- The activation-effect resolution chain: a randart's pval2, a real artifact's
 name1, a random artifact's art_name xtra2, the ego's name2 and name2b, then the
 object kind's built-in activate; negative values go to the Lua activation table
 for execution and timeout, positive values go to the ACT table (about one hundred
 and ninety entries: artifact exclusives such as GILGALAD's eight-beam starlight (directions
 1-9 minus the center, 75 damage each), NUMENOR's
 full monster probe (blow table maxed, drops forced, spell knowledge maxed, nine
 flag groups all known), POWER's Ring of Power three-way choice (summon the dead /
 trade worlds — autosave then change level / doom fireball 600 or a charm bolt),
 the Stone of Lore risky identify (a perfect casting costs 20 mana, otherwise
 overdraw with paralysis and confusion; the 1d12 self-damage always lands), ANGUIREL's
 thirteen-way escape (teleport 10 / 222 / make stairs / leave level), the
 dimension door (paired FEAT_BETWEEN tiles interconnect — an illegal grid, an ICKY
 grid, out of range or a failed roll drops the landing point randomly), light
 drain (extinguishes CAVE_GLOW tiles within six and fires a light bolt scaled by
 count times level), the dragon horn's four-family big balls always in direction
 five, the whirlwind attack hitting all directions, DISP dispelling good or evil,
 the summon family (element/demon/undead/thunderlord each carry a one-in-three
 out-of-control draw)).
- A randart's timeout is one tenth of its cost. Unknown effects announce and pass
 through.

- `brand_bolts` SHALL take the first qualifying bolt stack and brand `EGO_FLAME`
 with a 25% chance per stack, plus enchantment.

#### Scenario: Recharging item refuses

- **WHEN** activation is attempted on an item that is still recharging
- **THEN** the activation is refused

#### Scenario: Egg incubation toggle

- **WHEN** an incubating egg (timeout zero) is activated
- **THEN** incubation pauses (timeout set to -1); activating a paused egg
 resumes it (timeout zeroed)

- **Anchors**: `src/cmd6.c:4916-5161` (activation entry), `src/cmd6.c:5165-7914`
 (activation_aux full table), `src/cmd6.c:4758-4849` (the Ring of Power),
 `src/cmd6.c:4857-4903` (bolt branding), `src/cmd6.c:7917-7948` (staff spell
 execution)

### Requirement: Name Lookup

All eating, drinking, reading, staff-using and rod-zapping SHALL support lookup by
full name: after the full object name is typed, the pack and the equipment are walked
item by item with `object_desc` comparisons, and a hit selects the item.

#### Scenario: Lookup by full name

- **WHEN** a full object name is typed for one of the use actions
- **THEN** the pack and the equipment are walked item by item with
 `object_desc` comparisons and a hit selects the item

- **Anchors**: `src/cmd6.c:26-50`
