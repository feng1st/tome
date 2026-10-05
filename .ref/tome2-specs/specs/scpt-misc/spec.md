# scpt-misc Specification

## Purpose

Loose scripts: the standalone pieces under `lib/scpt/` — the bounty quest
(`bounty.lua`), the drunk who takes wine (`drunk.lua`), the god creeds (`gods.lua`),
the fireproofing quest (`fireprof.lua`), the script load order (`init.lua`), the
intro (`intro.lua`) and joke monsters (`joke.lua`), the mimic shape definitions
(`mimic.lua`), the skill activation mkeys (`mkeys.lua`), the birth object hook
(`player.lua`), the 'U' powers (`powers.lua`), the store lists (`stores.lua`), the
test scripts (`test.lua`), and the contextual help trigger definitions (`help.lua`).
The spell schools (`s_*.lua`) are covered by specs/school-magic/spec.md and
specs/school-spells/spec.md; the faith system that the god creeds hook into
(`follow_god`/`abandon_god` and the `HOOK_FOLLOW_GOD` handling) by specs/god/spec.md.
Language baseline: the Lua 4 dialect (`%var` upvalues, `arg` tables,
`getn`/`tinsert`/`tremove`/`call`, `TRUE`/`FALSE`).

## Requirements

### Requirement: Bounty Quest

`bounty.lua` SHALL register `BOUNTY_QUEST` through `add_quest` — a dynamic desc (in
the `TAKEN` state the hunted target name is printed through `print_hook`, built with
`monster_race_desc(bounty_quest_monster, 0)`), level -1, a `data` entry storing
`bounty_quest_monster = 0` into the savefile, and a `HOOK_BIRTH_OBJECTS` hook that
resets the quest to `UNTAKEN` at birth.

Two building actions SHALL be registered:

- Number 54: in the `UNTAKEN` state it turns the quest to `TAKEN`, picks the target with `get_new_bounty_monster(3 + (level x 3)/2)` and announces it; otherwise it repeats the target.
- Number 55: in the `TAKEN` state it collects a corpse through `get_item` (`USE_INVEN`, the tester matches `TV_CORPSE` with `pval2` equal to the target number) — after `inven_item_increase`/`inven_item_optimize` take the corpse away it prints the two-line thanks; when the `SKILL_LORE` mod is zero it is set to 900 with `dev` expanded, and `value` gains the mod; when the `SKILL_PRESERVATION` mod is zero its `value`/`mod` are both set to 800 with `dev` expanded and an extra line is announced; the quest returns to `UNTAKEN` and the target is cleared; otherwise it prints the no-quest line.

- **Anchors**: `lib/scpt/bounty.lua:3-24` (quest), `:26-40` (taking the quest), `:42-90` (turning in the corpse)

#### Scenario: Turning in a corpse

- **WHEN** building action 55 runs in the `TAKEN` state and the player selects a `TV_CORPSE` whose `pval2` equals the target number
- **THEN** the corpse is removed from the inventory, the skill rewards are applied, and the quest returns to `UNTAKEN` with the target cleared

### Requirement: Drunk Takes Wine

`drunk.lua` SHALL attach the `HOOK_GIVE` hook `drunk_takes_wine` — when the monster is
`"Singing, happy drunk"` and the item is `TV_FOOD` with sval 38 or 39, it prints
`'Hic!'` (yellow, via `cmsg_print`) and takes the item through
`inven_item_increase`/`inven_item_optimize`, returning true; otherwise it returns
false.

#### Scenario: Wine handover

- **WHEN** the player gives a `TV_FOOD` item of sval 38 or 39 to the `"Singing, happy drunk"`
- **THEN** `'Hic!'` is printed in yellow, the item is taken away, and the hook returns true

- **Anchors**: `lib/scpt/drunk.lua:3-21`

### Requirement: God Creeds

`gods.lua` SHALL attach two hooks:

- `HOOK_FOLLOW_GOD` (action `"ask"`): when following any god other than Melkor while any equipped slot from `INVEN_WIELD` upward holds an item with `name1 == 13` (`ART_POWER`, the One Ring), it prints `"The One Ring has corrupted you, and you are rejected."` and rejects the following.
- `HOOK_RECALC_SKILLS`: when a god is followed and `SKILL_ANTIMAGIC > 0`, it prints `"You no longer believe."` and calls `abandon_god(GOD_ALL)`.

#### Scenario: One Ring rejection

- **WHEN** the player is asked to follow a god other than Melkor while any equipped slot from `INVEN_WIELD` upward holds an item with `name1 == 13`
- **THEN** `"The One Ring has corrupted you, and you are rejected."` is printed and the following is rejected

- **Anchors**: `lib/scpt/gods.lua:1-26`

### Requirement: Fireproofing Quest

`fireprof.lua` SHALL embed the quest map (`#!map`, 47 x 22: permanent walls, dirt floor,
shallow and deep lava, random objects mixed with red mold 324 / Chimaera 341 / red
dragon bat 377 / hellhound 613, the quest exit `<`, start position 22:26).

Constants SHALL be `TOTAL_ITEM_POINTS` 12 with `BOOK_POINTS`/`STAFF_POINTS`/
`SCROLL_POINTS` 4/3/1.

`add_quest` registering `FIREPROOF_QUEST` SHALL: level 20, `data` storing
`item_points_remaining` and `essence` into the savefile, and a birth reset. The hooks:

- `HOOK_GEN_QUEST` SHALL — after the quest-level check, `load_map(MAP, 2, 2)`, set `level_flags2` to `DF2_NO_TELEPORT`, roll `essence = randint(18)`, forge a `TV_BATERIE` with that sval, mark it with `pval2` and the `"quest"` quark, drop it on a random grid in the top half of the map, then place `rand_range(10, 30)` traps on safe grids (not permanent and floor) at `rand_range(20, 40)` levels through `place_trap`.
- `HOOK_STAIR` SHALL — on the quest level while standing on `FEAT_LESS`, confirm the abandonment (confirming marks `FAILED` and allows the stair, refusing keeps the player on the level).
- `HOOK_GET` SHALL — a pickup whose `pval2` matches the quest essence turns the quest to `COMPLETED`.

#### Scenario: Essence pickup

- **WHEN** the player picks up the `TV_BATERIE` whose `pval2` matches the quest essence on the quest level
- **THEN** the quest turns `COMPLETED`

Building action number 56 SHALL:

- `UNTAKEN` -> `TAKEN`, reset the point count and print the instructions (returns `TRUE, FALSE, TRUE`).
- `COMPLETED` — collect the essence through `get_item` (matching `TV_BATERIE` on both sval and `pval2`), then loop `fireproof()` until the points run out (at zero the quest becomes `REWARDED`, otherwise `FINISHED`).
- `TAKEN` repeats the essence location; `FINISHED` continues with the remaining points; `FAILED`/`REWARDED` refuse a new quest.

`fireproof()` SHALL — offer `get_item` over `TV_BOOK`/`TV_SCROLL`/`TV_STAFF` without
`TR3_IGNORE_FIRE`; after `enough_points` passes, a partial stack is split off through
`obj_forge` (the `number` values adjusted, the `carry_it` flag set); `name2` is set to
149; the three pvals are saved and restored around `apply_magic(-1, FALSE, FALSE,
FALSE)` (so the book type cannot change); a split-off piece is added back with
`inven_carry`; the item is finished with `set_known` plus `set_aware`.

`enough_points(obj)` SHALL — for a multi-item stack ask the count with
`get_quantity`, multiply by the per-tval point cost, print the out-of-material line
and return false when short, otherwise deduct the points (reaching zero marks
`REWARDED`) and return true plus the stack count.

- **Anchors**: `lib/scpt/fireprof.lua:6-77` (map and constants), `:79-241` (quest registration and the three hooks), `:244-359` (action number 56), `:362-442` (fireproof), `:445-479` (enough_points)

### Requirement: Script Load Order

`lib/scpt/init.lua` SHALL load, in `tome_dofile` order — `player`, `help`, `stores`,
`powers`, `mimic`, `corrupt`, `mkeys`, `spells` (the school compilation), `gods`; the
quest group `bounty`, `god`, `fireprof`, `library`; the joke group `drunk`, `joke`;
`dg_test.lua` through `tome_dofile_anywhere` (with `test_exist = FALSE`, so a missing
file is harmless); and finally `intro.lua` as the custom intro.

- **Anchors**: `lib/scpt/init.lua:5-46`

### Requirement: Intro Animation And Joke Monster

`intro.lua` SHALL — `drop_text_left`/`drop_text_right` slide text in character by
character (a `TERM_XTRA_DELAY` every four steps; a non-blocking `inkey_scan` key probe
interrupts and returns true); `tome_intro` goes through `screen_save`/`Term_clear` and
presents two scenes (the four question lines and the credits list ending with
`present` / `T.o.M.E.`); any interruption `screen_load`s and exits; the function is
attached to `HOOK_INIT`.

#### Scenario: Interrupting the intro

- **WHEN** a key is pressed while a slide-in line is still animating
- **THEN** the non-blocking probe returns true, `tome_intro` restores the screen with `screen_load` and exits

`joke.lua` SHALL — `gen_joke_place_monster` finds a spot within 1000 tries with
`randint(cur_hgt - 4) + 2` / `randint(cur_wid - 4) + 2` and calls `place_monster_one`
(`MSTATUS_ENEMY`); `gen_joke_monsters` is gated by the `joke_monsters` switch and on
dungeon 20, level 72, admits `"Neil, the Sorceror"` (the `m_allow_special` entry is
toggled on and off around the placement); attached to `HOOK_LEVEL_END_GEN`.

- **Anchors**: `lib/scpt/intro.lua:1-65` (slide-in functions), `:67-103` (tome_intro), `:105` (hook attachment), `lib/scpt/joke.lua:2-29`

### Requirement: Mimic Shape Definitions

`mimic.lua` SHALL define twelve shapes through `add_mimic_shape`.

The eight nature-realm shapes (the random pool):

- `Mouse` (level 1, rarity 10; speed and to-hit scale with the mimic level, damage cut to one fifth, a stealth bonus, STR-5/DEX+3/CON+1; level >= 30 grants `POWER_INVISIBILITY`).
- `Eagle` (level 10, rarity 30; `ffall`, stat adjustments, level 20 grants flight and see-invisible, 25 `free_act`, 30 lightning resistance, 40 the electricity aura).
- `Wolf` (level 20, rarity 40; STR/DEX bonuses, speed 10+level/5, `free_act` and fear resistance, levels 10/15/30/35 adding cold resistance / see-invisible / dark resistance / confusion resistance).
- `Spider` (level 25, rarity 50; INT and WIS bonuses while STR and CON wither, poison/fear/dark resistances, level 40 climbing, level >= 25 grants `POWER_WEB`).
- `Elder Ent` (level 40, rarity 60, `limit`; slowed, AC +10+level, poison and cold resistances / regen / see-invisible / fire sensitivity, the six stats adjusted with four of them scaling with level, `PWR_GROW_TREE`).
- `Vapour` (level 15, rarity 10; AC +40+level while to-hit drops by 40, cold immunity, stealth, fire sensitivity; it also sets `player.levitate`).

- **Quirk:** `player.levitate` is a script-invented field with no engine effect — no such field exists in the `player_type` binding.

- `Serpent` (level 30, rarity 25; speed and AC, poison resistance, level 25 `free_act`).
- `Mumak` (level 40, rarity 40; slowed while AC and damage gain heavily, STR/CON scale with level, levels 10/25/30/35 adding fear / confusion / `free_act` / nexus resistance).

The four extra-realm shapes (rarity 101, so never in the random pool; `limit`):

- `Bear` (for the Beornings; slowness traded for AC, four stats at +level/11, levels 10/20/30/35 progression, and the `calc` un-hides `SKILL_BEAR`).
- `Balrog` (six stat adjustments, fire/electric/acid immunity plus poison/dark/chaos resistance, `hold_life`, `ffall`, regen, the fire aura; the `calc` returns 1 blow).
- `Maia` (six large stat bonuses, four immunities plus poison/light/dark/chaos resistance; returns 2 blows).
- `Fire Elem.` (large STR/DEX bonuses with WIS withering, fire immunity, poison resistance, the fire aura, permanent light; returns 0 blows; the source notes it was poison-immune in the 3.0.0 version).

- **Anchors**: `lib/scpt/mimic.lua:4-261` (the eight nature shapes), `:266-296` (Bear), `:299-361` (Balrog and Maia), `:364-385` (Fire Elem.)

### Requirement: Skill Activation Mkeys And The Instant-Death GF

`mkeys.lua` SHALL — register `GF_INSTA_DEATH` through `add_spell_type` (dark color;
`angry` always returns true; the monster channel connects only on the 5% roll and
against non-unique, non-undead, living (not `RF3_NONLIVING`) targets; a connected target has its level
cut to one third and takes damage 32535 with the death words `" faints."` /
`" is sucked out of life."`).

Three mkeys SHALL be registered:

- 100 death touch — with mana above 40 it deducts 40 and calls `set_project` for a single-target `GF_INSTA_DEATH` projection lasting a random 11-40 rounds, `energy_use` 100; otherwise it prints the not-enough-mana line.

#### Scenario: Death touch with enough mana

- **WHEN** mkey 100 fires while the mana is above 40
- **THEN** 40 mana is deducted and a single-target `GF_INSTA_DEATH` projection lasting a random 11-40 rounds is set up at an `energy_use` of 100

- 101 Geomancy — gated by the anti-magic field, requires a wielded `TV_MSTAFF`, then `get_school_spell("cast", "is_ok_spell", 62)` picks a spell from book 62 and `cast_school_spell` casts it.
- 102 the polearm reach attack — requires `TV_POLEARM` with sval `HALBERD`/`PIKE`/`HEAVY_LANCE`/`LANCE`; after `get_rep_dir` it targets at double displacement, sets `max_blows` from `SKILL_POLEARM` scaled by `num_blow/2` with a floor of 1, adds 200 energy, and projects `GF_ATTACK` — at skill >= 40 a pure `BEAM` that pierces, otherwise `BEAM`+`STOP`.

- **Anchors**: `lib/scpt/mkeys.lua:3-17` (GF), `:20-32` (death touch), `:36-61` (Geomancy), `:64-95` (reach attack)

### Requirement: Birth Object Hook

`lib/scpt/player.lua` SHALL attach `__birth_hook_objects` to `HOOK_BIRTH_OBJECTS` —
per class it gifts a random-book spell book (`TV_BOOK` sval 255, `pval` holding the
`find_spell` number, `IDENT_MENTAL|IDENT_KNOWN`, added with `inven_carry` and
`end_object`): Ranger receives Phase Door, Geomancer Geyser, Priest(Eru) See the
Music, Priest(Manwe) Manwe's Blessing, Druid Charm Animal, Dark-Priest Curse, Paladin
Divine Aim; Mimic receives a `TV_CLOAK` sval 100 whose `pval2` is the Mouse mimic
shape; the Vampire subrace starts with the three Vampire corruptions granted in a row
(`TEETH`/`STRENGTH`/`VAMPIRE`).

#### Scenario: Mimic birth gift

- **WHEN** the birth object hook fires for a Mimic character
- **THEN** a `TV_CLOAK` sval 100 is added to the inventory whose `pval2` is the Mouse mimic shape

- **Anchors**: `lib/scpt/player.lua:4-76`

### Requirement: 'U' Power Registration

`lib/scpt/powers.lua` SHALL register three powers — `POWER_INVISIBILITY` (for the
Mouse mimic shape; level 30, cost 10 DEX, difficulty 20; casting runs
`set_invis(20 + randint(30), 30)`), `POWER_WEB` (for the Spider mimic shape; level 25,
cost 30 DEX, difficulty 20; `grow_things(16, 1 + level/10)` lays the webs),
`POWER_COR_SPACE_TIME` (the start/stop switch of the Anti-teleportation corruption;
level 1, cost 10 WIS, difficulty 10; it reverses the
`corrupt_anti_teleport_stopped` flag and raises `PU_BONUS`, and that flag is
initialized at module level and stored in the savefile with `add_loadsave`).

- **Anchors**: `lib/scpt/powers.lua:4-34` (invisibility and web), `:38-61` (space/time control)

### Requirement: Store Buying And Stocking

`lib/scpt/stores.lua` SHALL provide `store_buy_list` for nine stores — General
Store/Armoury/Weaponsmith carry tval lists; Temple is a function (Druid books, random
books limited to `SKILL_SPIRITUALITY`, scrolls, the two potion families, hafted
weapons, the four blessed weapon families); Alchemy is a list; Magic shop is a
function (random books limited to `SKILL_MAGIC`, all other books bought, plus the
symbiotic-book/amulet/ring/staff/wand/rod/rod-main/scroll/two-potion/`MSTAFF`/
`RANDART` list); Black Market always buys; Book Store carries the five book tvals
(`TV_BOOK`, `TV_SYMBIOTIC_BOOK`, `TV_MUSIC_BOOK`, `TV_DAEMON_BOOK`, `TV_DRUID_BOOK`);
Pet Shop takes only eggs. `out_sticks` is a debug output.

`HOOK_STORE_STOCK` SHALL — Magic shop and Temple each roll a one-in-five chance
(`magik(20)`) to forge a random spell book through `obj_forge` (`TV_BOOK` 255, with
`get_random_spell` under `SKILL_MAGIC` or `SKILL_SPIRITUALITY` at level 20); it returns `(TRUE, obj_forge)`.

#### Scenario: Magic shop stocks a random book

- **WHEN** `HOOK_STORE_STOCK` fires for the Magic shop, `magik(20)` succeeds, and `get_random_spell` finds a `SKILL_MAGIC` spell at level 20
- **THEN** the forged `TV_BOOK` 255 is returned as `(TRUE, obj_forge)`

- **Anchors**: `lib/scpt/stores.lua:1-107` (nine stores), `:110-123` (debug), `:126-147` (stocking hook)

### Requirement: Test Scripts

`lib/scpt/test.lua` SHALL provide:

- `zog_magic` registered through `add_magic` as a test spell group (the fail path prints its text and calls `take_hit("stupidity", 5)`; stat `A_CON`; `get_level` is the sum of `Magic` and `Spirituality` each scaled to 25; Zog1 is a mana ball of 2000 damage at radius ten, Zog2 uber-izes a weapon to `dd`/`ds` 255 plus `to_d`/`to_h` 1000, Zog3 summons 1d2 friendly Novices).
- `MKEY_SHINY_TEST` 1000 attached through `add_mkey` calling `execute_magic` with an extra 100 energy cost.
- `POWER_TEST`, which prints `Zogzog !`.
- The two zsock read/write test functions (against `192.168.0.200:2262`).
- The `dungeon2` generator test (a `possible_walls` wall-edge table, room and corridor floor layouts, with the feature-placement loop currently written `for nb = 1, 0`).

- **Dead code:** the `for nb = 1, 0` loop body never runs, so no extra features are placed; the player is placed at the center and the generator returns true.

- `input_list_example` demonstrating `display_list` keyboard browsing.

- **Anchors**: `lib/scpt/test.lua:8-101` (spell group), `:104-114` (mkey), `:121-134` (power), `:139-165` (socket tests), `:168-325` (generator test), `:329-364` (list example)

### Requirement: Contextual Help Trigger Definitions

`lib/scpt/help.lua` SHALL register nineteen triggers through `ingame_help` —
seventeen in the hook form and two in the callback form:

- Hook form, seventeen: under `HOOK_MOVE` five (`FEAT_BETWEEN` jumpgates, `FEAT_FOUNTAIN` fountains, an item on the grid, the altar stretch, `FEAT_MORE` stairs); under `HOOK_END_TURN` two (the leaving-Bree (21,34) wilderness notice with the non-astral condition — the source carries a hardcoded-coordinates warning — and the always-true welcome text with the parchment pointers); under `HOOK_PLAYER_LEVEL` two (level-up skill points, and the level >= 20 character-dump brag); under `HOOK_IDENTIFY` one (full identify of a `TR5_SPELL_CONTAIN` spell container); under `HOOK_GET` six (`TV_BATERIE` essences, `TV_RUNE1|2` runes, `TV_ROD_MAIN` rod bases, `TV_ROD` rod tips, `TV_TRAPKIT` trap kits, `TV_WAND|STAFF` staves and wands); under `HOOK_RECALC_SKILLS` one (the multiple-melee-types notice, gated on `game.started` and `get_melee_skills() > 1`).
- Callback form, two: `monster_chat` (the talking notice) and `select_context` (`no_test` bypasses the option gate; a six-category mapping table — race with 22 names, subrace 9, class 33, god 5, skill 55, ability 11 — each entry pairing a document name with an anchor; a miss falls back to `help.hlp` presented through `ingame_help_doc`).

The trigger texts themselves are help text; only the structure is recorded here.

#### Scenario: Jumpgate notice

- **WHEN** the player steps onto a `FEAT_BETWEEN` grid while the ingame help option is enabled
- **THEN** the jumpgate help entry plays its text

- **Anchors**: `lib/scpt/help.lua:9-127` (hook-form triggers), `:129-296` (the select_context mapping table), `:298-411` (the object-pickup group and tail entries)

### Requirement: Library Quest

`library.lua` SHALL embed the quest map (`#!map`, 65 x 38 bookshelf maze, cobblestone
road `O`, the exit `<`, one Master lich (658) preset on the map face, start position
4:4).

The book-building machinery SHALL — `school_book[61]` with three slots using -1 as the
sentinel; the `bookable_spells` list of forty-five spells;
`book_slots_left`/`book_contains_spell`/`add_spell`/`remove_spell` (the last shifts
the following slots up after a removal); `print_spell` renders one line through the
255 random-book channel (name / school string / level / cost / fail rate / info);
`print_spells` is the book screen (the slot notice has three variants — red when no
slots remain, blue otherwise; the
current entry green, chosen white, available orange); `fill_book` SHALL clear the
book and loop under `screen_save` — ESC confirms the book when full and otherwise
exits at once, `n`/`p` page through, `I` describes, directions 2/8 move, 6 adds a
spell and 4 removes one, with the scroll position clamped.

`add_quest` registering `LIBRARY_QUEST` SHALL — level 35, the three slots stored into
the savefile, a birth reset. The hooks:

- `HOOK_GEN_QUEST` SHALL — after `load_map`, set `DF2_NO_GENO` and place monsters: Lich (518) 4d2 in the top-left region, Monastic Lich (611) 1d2 plus two runs of 1d2-1 across the bottom-right / bottom-left / top-right regions, and the four golem kinds 256/261/367 twice each with 464 once in the middle region.
- `HOOK_STAIR` uses the same confirm-abandonment shape as the fireproofing quest.
- `HOOK_MONSTER_DEATH` SHALL — scan `m_list` counting living enemy monsters
 (`status <= MSTATUS_ENEMY`), starting the counter at -1; when the count reaches 0
 the quest turns `COMPLETED` and prints `"The library is safe now."`.

Building action number 61 SHALL — `UNTAKEN` -> `TAKEN` with printed instructions
(returns `TRUE, FALSE, TRUE`); `COMPLETED` enters `fill_book`, and with all three
spells set the quest turns `REWARDED` and a `TV_BOOK` sval 61 is forged (`art_name` a
quark of the player name, `found = OBJ_FOUND_REWARD`, aware and known, added with
`inven_carry`); `TAKEN` repeats the request; `FAILED`/`REWARDED` refuse a new quest.

#### Scenario: Clearing the library

- **WHEN** the last living enemy monster on the quest level dies
- **THEN** the count reaches 0, the quest turns `COMPLETED`, and `"The library is safe now."` is printed

- **Anchors**: `lib/scpt/library.lua:8-80` (map and placement helper), `:83-166` (book slot functions), `:169-310` (book screen and fill_book), `:313-471` (quest and the three hooks), `:474-513` (action number 61)
