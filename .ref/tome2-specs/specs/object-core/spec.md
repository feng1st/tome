# object-core Specification

## Purpose

Object core: `src/object1.c` carries the item flavor system, the visual reset, flag
aggregation (the `object_flags` family), item description generation (the full
`object_desc` grammar), item display and selection (the `get_item` interaction
machine), floor pickup (the `py_pickup_floor` family), equipment slot resolution,
intelligent weapon leveling (`TR4_LEVELS`) and item-set attach/detach. Carry, drop and
use actions are recorded under cmd3/cmd6 and in `src/object2.c`.

## Requirements

### Requirement: Flavor System

The flavor tables SHALL be provided per category: 62 ring adjective+color pairs, 34
amulet pairs, 35 staff woods, 39 wand/rod metals (the rod arrays start as a copy of
the wand arrays at startup), 20 mushrooms, 66 potion colors (the first four — Clear,
Light Brown, Icky Green, Strangely Phosphorescent — are fixed and are never shuffled),
and 55 scroll titles composed from 164 syllables.

- `flavor_init` SHALL shuffle every adjective/color group deterministically, using
 `seed_flavor` with the quick RNG (`Rand_quick`).
- Scroll titles SHALL be built from one- or two-syllable words up to the
 15-character limit, deduplicated on the first four characters; every title color
 stays `TERM_WHITE`.
- The subsequent whole-k_info scan SHALL map each kind through `object_flavor` onto a
 flavor color segment by tval in the 0x80-0xF0 range; a kind without a flavor is
 marked aware — except `TV_ROD_MAIN`.
- `object_easy_know` SHALL define `easy_know`: TRUE for books/flasks and bottles/
 eggs/skeletons/corpses/symbiotes (`TV_HYPNOS`)/spikes/junk; TRUE for food/potions/
 scrolls/rod tips (`TV_ROD`, `TV_ROD_MAIN`)/essences (`TV_BATERIE`) unless
 `TR3_NORM_ART`; for rings/amulets/lights only via `TR3_EASY_KNOW`.
- `get_table_name` SHALL produce a quoted random name: one third of the time from 2-4
 syllables, otherwise from 2-3 lines of `elvish.txt`; the first letter is capitalized
 and the result is truncated to 18 characters.

#### Scenario: Fixed potion colors survive the shuffle

- **WHEN** `flavor_init` shuffles the flavor groups
- **THEN** the first four potion colors (Clear, Light Brown, Icky Green,
 Strangely Phosphorescent) are never shuffled

#### Scenario: Flavorless kind is marked aware

- **WHEN** the whole-k_info scan meets a kind without a flavor
- **THEN** the kind is marked aware, except `TV_ROD_MAIN`

- **Anchors**: `src/object1.c:27-272` (tables), `src/object1.c:279-335` (flavor
 mapping), `src/object1.c:338-368` (random names), `src/object1.c:377-431`
 (easy_know), `src/object1.c:464-669` (flavor_init)

### Requirement: Visual Reset

`reset_visuals` SHALL reset `x_attr`/`x_char` back to `d_attr`/`d_char` for the four
families terrain/store/object/monster, and SHALL zero the overlay graphics for the
three overlay families monster-ego/race-modifier/trap. When `use_graphics` is on it
processes `graf.prf` and derives `graphics_mode` from `ANGBAND_SYS`/`ANGBAND_GRAF`
(ibm/iso/new/old/unknown); otherwise it processes `font.prf` with `GRAPHICS_NONE`.

- **Quirk:** the trap overlay loop is bounded by `max_rmp_idx` (10, the
 race-modifier count) instead of `max_t_idx` (176), so only the first ten trap
 overlays are zeroed.

#### Scenario: Graphics mode selection

- **WHEN** `reset_visuals` runs with `use_graphics` on
- **THEN** `graf.prf` is processed and `graphics_mode` derives from
 `ANGBAND_SYS`/`ANGBAND_GRAF`; with `use_graphics` off `font.prf` is processed
 with `GRAPHICS_NONE`

- **Anchors**: `src/object1.c:685-819`

### Requirement: Flag Aggregation

`object_flags` SHALL compose the flags in layers — the k_info base; a full
replacement by the `name1` artifact flag set (with the set overlay applied through
`apply_flags_set` when the artifact is registered in a set); a bitwise OR of the six
`art_flags` groups (the random artifact layer); and, when the object carries no
`art_name`, an extra tier selected by `xtra1` — `EGO_XTRA_SUSTAIN` takes one of six
sustains by `xtra2 % 6`, `EGO_XTRA_POWER` one of eleven resistances by `% 11`,
`EGO_XTRA_ABILITY` one of eight abilities by `% 8` (including `ESP_ALL`).

- `object_power` SHALL return the first granted power that is not -1, in the order
 base -> ego -> second ego -> artifact.
- `object_flags_known` SHALL produce the player-known face: it requires
 `object_known_p`; the base flags plus the oflags obvious group; artifact flags need
 `IDENT_MENTAL` (or the compile-time SPOIL switch) and are otherwise cleared;
 `art_flags` and the xtra tier likewise require MENTAL; the obvious group is always
 merged in; `TR2_RES_CHAOS` implies `TR2_RES_CONF`.
- The global `object_flags_no_set` can disable the set overlay.

#### Scenario: Unknown artifact flags are cleared

- **WHEN** `object_flags_known` runs without `IDENT_MENTAL` on an artifact
- **THEN** the artifact flag set, the `art_flags` groups, and the xtra tier are
 cleared from the known face

#### Scenario: Chaos resistance implies confusion resistance

- **WHEN** the known flag face carries `TR2_RES_CHAOS`
- **THEN** `TR2_RES_CONF` is merged in

- **Anchors**: `src/object1.c:832-1018` (flags and power), `src/object1.c:1025-1232`
 (known face)

### Requirement: Object Description Grammar

`object_desc` SHALL generate the description into a four-part buffer (`pref` controls
the count prefix, mode 0-3 the detail level).

Base-name grammar:

- `&` triggers the article section: no prefix / "no more" / a count / "The" (for a
 known artifact or randart) / "a"/"an" — "an" is chosen from the modifier string, the
 ego prefix name, or a leading vowel.
- `~` triggers the plural (count not one and pref >= 0; names ending in s/h add
 "es").
- `#` injects the modifier string (a `TV_ROD_MAIN` rod uses the known ego name as its
 modifier).

Per-tval templates:

- missile weapons/bows/melee weapons run show_weapon, armor runs show_armour, trapkits
 use "& # Trap Set~".
- ring/amulet/staff/wand/potion/mushroom take the flavor adjective; the flavor is
 dropped under `plain_descriptions` or `IDENT_STOREB`.
- The One Ring while unidentified gets the modifier "Plain Gold"; known artifact
 rings/amulets use the k name directly.
- `TV_ROD` becomes "& Rod Tip~" (with a special name for `SV_ROD_HOME`); `TV_ROD_MAIN`
 takes the rod tip's k name.
- scrolls are titled "#"; mimic cloaks and mimic potions get their name through Lua
 `get_mimic_info`.
- symbiotic/music/druid books use "& ...~ #"; essences "of #"; parchments "- #"; gold
 returns directly.
- corpse/egg/symbiote use the monster name (a unique corpse becomes "X's"); totems use
 `monster_desc` 0x188; randarts use `name_full`/`name_short`; the double rune uses
 "& Rune~ [#]".
- spellbooks with sval 255 append the embedded spell name; the default tval goes
 through the `HOOK_ITEM_NAME` hook, else "(nothing)".
- With `TR5_FULL_NAME` known, the whole group is replaced by the k name.

Additional sections:

- pseudo names (an inscription `%` suffix becomes a prefix name, `#` a suffix name).
- When known, the random artifact name quark / artifact name (unique corpses
 excepted) / ego name (before-types are already prefixed) is appended; spells inside
 `SPELL_CONTAIN`; the symbiote's remaining blood.

mode >= 1 SHALL add:

- for `TR4_LEVELS` intelligent weapons the (E: current exp or exp_need difference,
 L: elevel) pair; `TR4_ART_EXP` (Exp:); chests (empty / disarmed / trap name or
 "trapped").
- `TR3_SHOW_MODS` or strongly-bonused weapons show both bonuses; ammo with nonzero
 pval2 shows "(exploding)"; weapons show (NdM); bows show (xM, `XTRA_MIGHT` adding
 the pval).
- to_h/to_d bracketed notes (with `HIDE_TYPE` wording "to accuracy"/"to damage");
 armor shows [ac,to_a]; `TR1_MANA` and `TR2_LIFE` show percentages (folded by 5 under
 `munchkin_multipliers`, otherwise 10).

mode >= 2 SHALL add:

- staff/wand charges (N charges); `TV_ROD_MAIN` (timeout/pval2); `TV_ROD` (N Mana to
 cast); `FUEL_LITE` fuel time; the pval note (SPEED/BLOWS/CRIT%/STEALTH/SEARCH/INFRA/
 TUNNEL — with `HIDE_TYPE` only the number); `TR3_ACTIVATE` while charging (eggs
 "stopped"); the mage staff's second charge slot.

mode >= 3 SHALL add:

- feeling text, cursed, inscription (truncated after `#` or `%`), empty, tried,
 discount % off; the total length is truncated to 79.

#### Scenario: Unidentified One Ring

- **WHEN** the One Ring is described while unidentified
- **THEN** the modifier "Plain Gold" is used

#### Scenario: Vowel article

- **WHEN** the modifier string, the ego prefix name, or a leading vowel makes
 the name start with a vowel
- **THEN** the article section chooses "an" instead of "a"

- **Anchors**: `src/object1.c:1242-1347` (low-level string parts),
 `src/object1.c:1394-2606` (object_desc)

### Requirement: Store Description And Activation Description

`object_desc_store` SHALL temporarily set aware and `IDENT_KNOWN`, describe, then
restore both. `item_activation` SHALL require `TR3_ACTIVATE`; for `EGO_MSTAFF_SPELL`
the double rune builds its own timeout case from pval low 16 / pval3 low 16 / pval2
low 8 and the high halves ("runespell(...) every N turns"); eggs use "stop or
resume..."; instruments without an activation entry report aggravate for `SV_HORN`;
the `HOOK_ACTIVATE_DESC` hook then takes priority over the `activation_aux`
default.
`grab_tval_desc` SHALL output the category description from `tval_descs`.

#### Scenario: Store face is temporary

- **WHEN** `object_desc_store` generates a description
- **THEN** aware and `IDENT_KNOWN` are set for the call and restored afterwards

#### Scenario: Activation hook takes priority

- **WHEN** the `HOOK_ACTIVATE_DESC` hook is registered and
 `item_activation` builds a description for a non-special item
- **THEN** the hook's text replaces the `activation_aux` default

- **Anchors**: `src/object1.c:2613-2731`

### Requirement: Damage Estimation And Display

`display_weapon_damage` SHALL compute with a temporary equip swap (`object_copy` into
`INVEN_WIELD` plus `calc_bonuses`, restored afterwards): base average damage
(dd + dd*ds)*5 plus (to_d + player to_d + to_d_melee)*10, multiplied by the number of
attacks, listing multipliers per known (`IDENT_MENTAL`) flag — slay family 2 to 3,
kill family 5, brand family 3 plus 6 against the vulnerable double tier, everything
else "all/other monsters".

- `display_ammo_damage` SHALL be the ammo version — multiplied by `get_shooter_mult`
 plus `xtra_might` (boomerangs use `throw_mult`), and it adds the bow's to_d and
 to_d_ranged; exploding ammo appends the `gf_names` text.
- `describe_device` SHALL, for known staffs/wands, enter stick mode and output the
 spell description, level, failure chance and info through Lua.

#### Scenario: Known staff enters stick mode

- **WHEN** `describe_device` runs on a known staff or wand
- **THEN** the spell description, level, failure chance and info are output
 through Lua

#### Scenario: Exploding ammo note

- **WHEN** `display_ammo_damage` lists damage for exploding ammo
- **THEN** the `gf_names` text is appended

- **Anchors**: `src/object1.c:2739-2961`

### Requirement: Item Long Description

`object_out_desc` SHALL output the full description to a file or the screen (the
screen state saves the screen); without MENTAL and when not writing to a file, only
the obviously-group flags are used.

Known sections, in order:

- the k_text and a_text originals plus the set description;
- `TR4_LEVELS` sense bitmap listing realms via `flags_groups`; `TR4_ULTIMATE` as one
 of the three ultimate artifacts; COULD2H/MUST2H; activation text; granted power;
 light radius (LITE1/2/3 summed, capped at 5; `FUEL_LITE` gets the "has fuel"
 wording); `ART_ANCHOR` gets the temporal anchor special text; anti-magic field;
 `SPELL_CONTAIN`.

Attribute section:

- the pval family list (trapkit STEALTH is reworded "well-hidden"); MANA/LIFE
 percentages; the four brand families plus POIS; CHAOTIC/VAMPIRIC/IMPACT/VORPAL/
 WOUNDING; the eleven slay and kill tiers; INVIS; the six sustains; the five
 immunities (trapkit rewords the nine `TRAP2_*` special flags); the sixteen
 resistances (immune bits excluded); SENS_FIRE/WRAITH/breathing/levitation/flying/
 climbing/IMMOVABLE/SEE_INVIS; the fourteen ESP tiers or ALL; SLOW_DIGEST/REGEN/
 REFLECT/auras/NO_MAGIC/NO_TELE/XTRA_MIGHT/SHOTS; the three drain flags; BLESSED/
 AUTO_ID/TELEPORT/AGGRAVATE/NEVER_BLOW/BLACK_BREATH; the three curse tiers plus
 TY/DG/CLONE/CURSE_NO_DROP/AUTO_CURSE; the four staff flags; `RES_MORGUL`; the four
 IGNORE families.

- When not in terse mode, the output additionally carries `describe_device`, the
 weapon/ammo damage estimation, and the breakage rate (`breakage_chance`; artifacts
 never break); the totem/corpse monster lore; the unidentified / not fully
 identified hints.
- The source history section covers the nine `OBJ_FOUND_*` tiers;
 `object_out_desc_where_found` words it by the wilderness/town/dungeon hierarchy.

#### Scenario: Unmental view on screen

- **WHEN** `object_out_desc` shows an item on screen without `IDENT_MENTAL`
- **THEN** only the obviously-group flags are listed

#### Scenario: Artifacts never break

- **WHEN** the breakage rate section runs for an artifact
- **THEN** the breakage chance is never applied (artifacts never break)

- **Anchors**: `src/object1.c:2970-4014`

### Requirement: Equipment Slot Resolution

The slot helpers SHALL provide `index_to_label`/`label_to_inven`/`label_to_equip`
letter conversions (an empty slot is rejected); `get_slot` SHALL walk the contiguous
same-slot segments of `body_parts` for the first free slot (no segment returns -1;
when all are taken it falls back to the first cell).

- `wield_slot_ideal` SHALL ask the `HOOK_WIELD_SLOT` hook first, then map by tval
 (digger/tool to `INVEN_TOOL`, melee to `INVEN_WIELD`, the bow family to
 `INVEN_BOW`, ring/amulet/light/body armor/cloak/shield/helm/gloves/boots/symbiote
 each to its own slot).
- The three ammo families pair with the bow by sval range (shot < 10, arrow 10-19,
 bolt >= 20) and merge into the quiver slot when stackable with the quiver content
 (`MAX_STACK_SIZE` check).
- `mention_use`/`describe_use` SHALL carry the slot wording table with the
 heavy-weapon fold (when `adj_str_hold` is below one tenth of the weight:
 "Just lifting"/"Just holding") and "Playing" for instruments.

#### Scenario: All slots taken

- **WHEN** `get_slot` finds every cell of the matching segment taken
- **THEN** it falls back to the first cell of the segment

#### Scenario: Ammo pairing by sval

- **WHEN** an arrow is resolved against a bow whose sval lies in 10-19
- **THEN** it pairs with that bow (shots need a bow sval below 10, bolts one at
 20 or above) and merges into the quiver slot when stackable with the quiver
 content

- **Anchors**: `src/object1.c:4022-4468`

### Requirement: Inventory Display And Selection Machine

`item_tester_okay` SHALL filter by `item_tester_full`/tval/hook and always reject
gold. `show_inven_aux`/`show_equip_aux` SHALL produce the right-aligned column layout
(labels/weights/graph switches each reserve their place).

- The inventory list cuts off at the last used slot, so trailing empty slots are not
 shown; the equipment lists the slots that actually exist in `body_parts` (the
 symbiote slot needs `SKILL_SYMBIOTIC`; when a two-handed weapon occupies the arms,
 the arm slot gets a red "(two handed)"; an empty weapon slot shows the current
 fighting style name).
- Unselectable entries are greyed with their labels; the letter color comes from
 `get_item_letter_color` (unknown slate / ego L_BLUE / artifact yellow / set green /
 ULTIMATE with MENTAL adds purple).
- `toggle_inven_equip` SHALL swap the eight PW_INVEN/PW_EQUIP windows.

The selector `get_item_floor` SHALL:

- with a repeat stack, take the item directly and verify it;
- open the domain per the USE_* bits and shrink i1/i2, e1/e2 to valid endpoints;
- rotate the three states of `command_wrk` (`/` switches pack/equipment, `-`
 switches to the floor; a single floor item is taken directly);
- treat `*` as the item-list toggle; digits go through `get_tag` (inscriptions `@n`
 and `@<command>n`); uppercase letters verify first; `@` goes to
 `get_item_extra_hook`; `$` switches to `automatizer_create`; Enter takes the
 default unique item;
- throughout, `get_item_allow` intercepts and asks confirmation per the `!<command>`
 or `!*` inscriptions;
- once chosen: `object_track`, clear the item_tester, `repeat_push`.

#### Scenario: Gold never selectable

- **WHEN** `item_tester_okay` evaluates a gold item
- **THEN** it is always rejected

#### Scenario: Single floor item taken directly

- **WHEN** the player presses `-` with exactly one floor item underfoot
- **THEN** the selector switches to the floor and the item is taken directly

- **Anchors**: `src/object1.c:4473-4498` (tester), `src/object1.c:4548-4955` (the two
 lists), `src/object1.c:4963-4995` (window toggle), `src/object1.c:5004-5215`
 (verify/allow/tag/scan_floor), `src/object1.c:5324-6136` (the selector)

### Requirement: Floor Pickup

The pickup helpers SHALL: `wear_ammo` merge ammo into the quiver slot (cursed adds
`IDENT_SENSE` with `SENSE_CURSED`); `pickup_ammo` auto-merge whatever underfoot
matches the quiver content; `can_carry_heavy` ask with the weight-penalty increment
under `prompt_pickup_heavy`.

- `object_pickup` SHALL: under `auto_id` fully identify first; the `HOOK_GET` hook
 can veto; an object matching the quiver content goes through `wear_ammo`, anything
 else through `inven_carry`; on success it announces "You have %s (%c)" and calls
 `delete_object_idx`.
- `py_pickup_floor` SHALL: return immediately on a monster-trap square; run
 `pickup_ammo` first; under `auto_id` identify fully then `squeltch_grid` (avoiding
 a second step-on); pick up gold immediately (pval into `au`, `PR_GOLD`); with
 pickup disabled announce "You see %s." or "You see a pile of %d items."; for a
 single item, ask when `carry_query_flag` or overweight (no space reports no room,
 `TV_HYPNOS` needs `SKILL_SYMBIOTIC`); for multiple items go through `get_item`
 floor selection (filtered by `item_tester_hook_getable`); finally `object_pickup`.

#### Scenario: Trap square short-circuit

- **WHEN** the player steps on a monster-trap square
- **THEN** `py_pickup_floor` returns immediately

#### Scenario: Gold picked up at once

- **WHEN** the player steps on gold
- **THEN** the gold is picked up immediately with the pval added to `au` and a
 `PR_GOLD` redraw

- **Anchors**: `src/object1.c:6141-6159` (getable hook), `src/object1.c:6154-6303`
 (ammo and weight), `src/object1.c:6306-6368` (object_pickup),
 `src/object1.c:6371-6562` (py_pickup_floor)

### Requirement: Intelligent Weapon Leveling

The leveling helpers SHALL: `gain_flag_group` randomly pick an unowned realm whose
price fits the `pval2` budget (1000 attempts), deduct the price and set the bit in
`pval3`; `get_flag` randomly take one unheld bit from the five flag groups of a given
realm; `gain_flag_group_flag` spend 20000 attempts to move a random flag of owned
realms into `art_flags`/`esp` and announce it.

- `object_gain_level` SHALL apply only to the five melee weapon kinds — with 33%
 chance add d2 to-hit and 1 damage; with 33% chance add 1 to-hit plus 1 to `pval2`
 and open a new realm per `NEW_GROUP_CHANCE`; otherwise open a realm plus a flag and
 grow the pval (iterating at 20-2*pval, capped at 5).

#### Scenario: Non-weapon refuses to level

- **WHEN** `object_gain_level` runs on an item outside the five melee weapon
 kinds
- **THEN** no leveling is applied

#### Scenario: Hit-and-damage branch

- **WHEN** an intelligent weapon gains a level and the first 33% branch hits
- **THEN** d2 to-hit and 1 damage are added

- **Anchors**: `src/object1.c:6565-6761`

### Requirement: Item Set Attach And Detach

The set helpers SHALL: `wield_set` record the member as present and add to `num_use`
(when the set is complete it announces "item set completed."; beyond full it logs an
ERROR); `takeoff_set` perform the inverse (when `num_use` drops to one below the
full count — the set breaking from complete — it announces "not complete
anymore."); `apply_set` apply each present member layer by
layer (j < num_use) through apply_flags for each threshold tier's bonus;
`apply_flags_set` merge each layer's flags into the output group with the same
structure — `object_flags` and `calc_bonuses` take their set portion from here.

#### Scenario: Set completes

- **WHEN** `wield_set` adds the last missing member of a set
- **THEN** "item set completed." is announced

#### Scenario: Set breaks apart

- **WHEN** `takeoff_set` drops `num_use` to one below the full count
- **THEN** "not complete anymore." is announced

- **Anchors**: `src/object1.c:6767-6860`
