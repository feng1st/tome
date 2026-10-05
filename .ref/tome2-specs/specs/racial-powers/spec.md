# racial-powers Specification

## Purpose

Racial and extended powers: `src/powers.c` carries the unified casting pipeline for
player special abilities - the power-table selection menu (`select_power`), the
success-rate and cost adjudication (`power_chance`), and the execution of the roughly
sixty `PWR_*` power cases (`power_activate`). The power table data
(`powers_type_init`, `src/tables.c`) is registered in specs/tables/spec.md; the
ability sources (which player owns which power, set at birth time in
`src/birth.c:833-840`) are registered in specs/character-birth/spec.md.

## Requirements

### Requirement: Success Rate And Cost Adjudication

`power_chance` SHALL adjudicate every power use through one shared gate sequence:
zero-cost powers always succeed, unlearned or confused players are refused without
spending energy, the price is paid in mana or in hit points, and the success roll
compares a stat roll against the entry's difficulty.

- Zero-cost entries (`cost == 0`) return TRUE immediately.
- If `csp < cost` the cost converts to hit points (the hp channel).
- If `lev < x_ptr->level` the power is refused with
 "You need to attain level %d to use this power." and `energy_use = 0`.
- If `p_ptr->confused` the power is refused with
 "You are too confused to use this power." and `energy_use = 0`.
- On the hp channel with `chp < cost` the player must confirm with
 "Really use the power in your weakened state? " or the power is refused
 (`energy_use = 0`).
- Stun adds `p_ptr->stun` to `diff`; otherwise a player above the entry's level
 lowers `diff` by `(lev - level) / 3`, capped at 10. `diff` never drops below 5.
- The price paid is `cost/2 + randint(cost/2)` (hit points or mana) and
 `energy_use = 100`.
- The success roll is `randint(p_ptr->stat_cur[x_ptr->stat]) >= diff/2 + randint(diff/2)`;
 on failure "You've failed to concentrate hard enough." plays (input flushed when
 `flush_failure`).
- The entry into `power_activate` first breaks both the `invuln` and the
 `disrupt_shield` shields.

#### Scenario: Confused player refused

- **WHEN** a confused player calls a power
- **THEN** it is refused with "You are too confused to use this power." and
 `energy_use = 0`

#### Scenario: Weakened hp channel confirms

- **WHEN** the cost converts to hit points and `chp < cost`
- **THEN** "Really use the power in your weakened state? " must be confirmed or
 the power is refused (`energy_use = 0`)

- **Anchors**: `src/powers.c:18-99` (adjudication), `src/powers.c:118-128` (shield removal)

### Requirement: The Power Menu

`select_power` SHALL collect every power enabled in the `p_ptr->powers` bitmap and
offer it in a paged letter menu of twenty entries per page, and `do_cmd_power` SHALL
support the repeat stack plus direct selection through `command_arg`, refusing
powers the player does not own.

- The menu header row is "Name / Level / Mana / Fail"; the line format is
 `a-NNN) name level mana stat@diff`.
- `*` toggles the detail display on and off; `+`/`-` scroll between pages.
- `do_cmd_power` pulls a prior choice from the repeat stack (`repeat_pull`), or asks
 the menu when `command_arg` is unset, or picks entry `command_arg - 1` directly.
- A power not present in the bitmap is refused with
 "You do not have access to this power." (the choice is still pushed onto the
 repeat stack).

#### Scenario: Unowned power refused

- **WHEN** `do_cmd_power` selects a power absent from the `p_ptr->powers`
 bitmap
- **THEN** "You do not have access to this power." prints and the choice is
 still pushed onto the repeat stack

#### Scenario: Direct selection by count

- **WHEN** `command_arg` is set when `do_cmd_power` runs
- **THEN** entry `command_arg - 1` is picked directly without the menu

- **Anchors**: `src/powers.c:1264-1380` (menu), `src/powers.c:1383-1412` (entry point)

### Requirement: Power Cases

`power_activate` SHALL dispatch on the `PWR_*` case number; each case below records
its as-is behavior.

- `PWR_BALROG` / `PWR_BEAR`: shapechange via `set_mimic` (durations `lev/2` and
 `150 + lev*10` respectively).
- `PWR_COMPANION`: promotes the selected pet to `MSTATUS_COMPANION`
 (limited by `can_create_companion`).
- `PWR_MERCHANT`: a three-way choice, each option carrying its own inline
 level/cost/stat/diff adjudication:
 - Appraise writes `object_value` into the item's inscription;
 - Warp sends the item into a chest - the roll is `randint(10) > level` where
 `level` is the item's kind level (cost below 20 counts as 0, cost below 100
 counts as 1), so the cheaper the item the more likely it vanishes; on survival
 the chest's `pval2` increments and the chest's `pval` drops by
 `level / ((sval % SV_CHEST_MIN_LARGE) * 2)`;
 - Identify goes through `ident_spell`.
- `PWR_LAY_TRAP`: forwards to `do_cmd_set_trap`.
- `PWR_MAGIC_MAP`: `map_area` (with "You sense the world around you.").
- `PWR_PASSWALL`: points at the passwall capability (asks for a direction, then
 `passwall(dir, TRUE)`).
- `PWR_COOK_FOOD`: creates food kind 21 and drops it on the floor.
- `PWR_UNFEAR` / `PWR_BERSERK`: both remove fear; the latter adds `shero`
 (`10 + randint(plev)`) and heals 30 hp.
- `PWR_EXPL_RUNE`: `explosive_rune`.
- `PWR_STM`: `wall_to_mud` on the aimed wall.
- `PWR_ROHAN`: a two-way choice - below level 10 a bolt, otherwise a confusion ball;
 at level 30 `light_speed + 3`.
- `PWR_POIS_DART`: a poisoned dart.
- `PWR_DETECT_TD`: detects traps, doors and stairs.
- `PWR_MAGIC_MISSILE`: beam/bolt of `3 + (level-1)/5` dice d4.
- `PWR_THUNDER`: a three-way choice:
 - Thunder strike: dual ELEC and SOUND beams at `2*level` each, plus an extra
 `energy -= 100`;
 - Ride the straight road: targeted teleport - validates an empty grid, no
 `CAVE_ICKY`, known grid (`CAVE_MARK`), and distance no more than `plev*20 + 2`;
 a failed exit costs another 100 energy and scatters via `teleport_player(10)`;
 - Go back in town: sets `word_recall = 1` (refused while already in town);
 - the second and third options are gated by `DF2_NO_TELEPORT`.
- `PWR_GROW_TREE`: `grow_trees` with `level/8`.
- `PWR_DEATHMOLD`: forwards to `do_cmd_immovable_special`.
- `PWR_BR_COLD` / `PWR_BR_CHAOS` / `PWR_BR_ELEM`: breath balls of `2*level` with radius
 `1 + level/20` or `level/15 + 1`.
- `PWR_SUMMON_MONSTER`: goes through the beast-taming path.
- `PWR_WRECK_WORLD`: restarts the world (autosave, then `leaving = TRUE`).
- `PWR_VAMPIRISM`: drains blood from adjacent grids
 (`plev + randint(plev) * max(1, lev/10)`); a successful `drain_life` heals
 and adds 100 nutrition per point, capped at 5000, with no feeding while Gorged.
- **Discrepancy:** the source comment promises "150/hp drained" of nutrition but
 the code multiplies by 100 (`MIN(5000, 100 * dummy)`).
- `PWR_SCARE`: `fear_monster`.
- `PWR_REST_LIFE`: `restore_level`.
- `PWR_HYPNO`: bottles adjacent motionless pets into a `TV_HYPNOS` object that
 records the `r_idx` and current hp (SPECIAL_GENE monsters are exempt; deleting the
 monster clears its hp bar).
- `PWR_UNHYPNO`: takes a floor `HYPNOS` object, uses `scatter` to find an empty
 grid, restores the monster as a PET and gives back its recorded hp.
- `PWR_NECRO` / `PWR_INCARNATE`: forwards to the undead and
 possession-return cases.
- `PWR_SPIT_ACID` / `PWR_BR_FIRE`: acid and fire balls.
- `PWR_HYPN_GAZE`: `charm_monster`.
- `PWR_TELEKINES`: `fetch` with `level*10`.
- `PWR_VTELEPORT`: teleport of `10 + 4*level`.
- `PWR_MIND_BLST`: a PSI bolt.
- `PWR_RADIATION`: a NUKE ball centered on the player, radius `3 + level/20`.
- `PWR_SMELL_MET` / `PWR_SMELL_MON`: detect treasure / detect monsters.
- `PWR_BLINK`: a short teleport.
- `PWR_EAT_ROCK`: eats a wall - refuses empty grids, permanent walls and mountains,
 monsters, and trees; adds nutrition by terrain (broken door 3000, vein 5000,
 sand 500, granite 10000), then `wall_to_mud` and walks into the grid (refreshing
 view and panel).
- `PWR_SWAP_POS`: `teleport_swap`.
- `PWR_SHRIEK`: a sound ball of `4*level`, radius eight, plus aggravation.
- `PWR_ILLUMINE`: `lite_area`.
- `PWR_DET_CURSE`: marks every cursed item in the pack and equipment
 `SENSE_CURSED`.
- `PWR_POLYMORPH`: `do_poly_self`.
- `PWR_MIDAS_TCH`: `alchemy`.
- `PWR_GROW_MOLD`: eight friendly `BIZARRE1` molds.
- `PWR_RESIST`: gambles temporary resistances in the order acid, lightning, fire,
 cold, poison - the budget is `lev/10` and each resist in turn is granted when
 `rand_int(5/4/3/2) < budget` (budget decrements per grant); any leftover budget
 always grants poison; each duration is `randint(20) + 20`.
- `PWR_EARTHQUAKE`: an earthquake of radius ten, refused on quest levels and on
 the surface.
- `PWR_EAT_MAGIC`: drains charges (filtered through the recharge hook) - a
 `ROD_MAIN` contributes its `timeout`, anything else `pval * object level`, added
 to `csp`; marks the item `IDENT_EMPTY`; `csp` is capped at `msp`.
- `PWR_WEIGH_MAG`: `report_magics`.
- `PWR_STERILITY`: self-damage of `d30 + 30` in exchange for
 `num_repro += MAX_REPRO`, ending explosive breeding.
- `PWR_PANIC_HIT`: with an adjacent monster, attacks and then teleports 30.
- `PWR_DAZZLE`: stun, confusion and turning, each at `4*level`.
- `PWR_DARKRAY`: a light beam of `2*level`.
- `PWR_RECALL`: recall (with `DF2_ASK_LEAVE`, asks to abandon the level).
- `PWR_BANISH`: deletes adjacent EVIL monsters (not a kill-style deletion).
- `PWR_COLD_TOUCH`: with an adjacent monster, a cold bolt of `2*level`.
- `PWR_LAUNCHER`: sets `throw_mult = 2 + level/16`, then `do_cmd_throw` and
 restores the multiplier.
- `PWR_DODGE`: forwards to `use_ability_blade`.
- An unknown power number goes to the `HOOK_ACTIVATE_POWER` hook; without a hook a
 warning prints and the energy cost is waived.

#### Scenario: Recall to town refused in town

- **WHEN** `PWR_THUNDER`'s "Go back in town" option is chosen while already in
 town
- **THEN** it is refused

#### Scenario: Unknown power falls to the hook

- **WHEN** `power_activate` meets an unknown power number without the
 `HOOK_ACTIVATE_POWER` hook
- **THEN** a warning prints and the energy cost is waived

- **Anchors**: `src/powers.c:130-1259` (the full case switch; THUNDER `:524-619`,
 EAT_ROCK `:896-965`, EAT_MAGIC `:1066-1123`)
