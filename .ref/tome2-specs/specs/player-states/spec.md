# player-states Specification

## Purpose

Player timed states: the `set_*` family of `src/xtra2.c` - roughly forty timed-state
setters sharing one skeleton (value clamping/on-off announcements/PU_BONUS/handle_stuff),
plus the composite states The Rush, grace, the parasite, spell projectors, roots,
breathing and others. The attr setters (set_str and kin) are specified with the misc
material; each state's effect consumption is specified at its own trigger sites
(calc_bonuses, dungeon.c and so on).

## Requirements

### Requirement: Timed State Setters

The `set_*` family SHALL share one skeleton - the argument clamps to 0-10000, the on and
off transitions each announce their own message, a change disturbs the state
(disturb_state option) and sends PU_BONUS followed by handle_stuff.

The composite parameter slots SHALL be: set_project (gf/dam/rad/flag four slots),
set_roots (ac/dam), set_tim_breath (the magical switch picks tim_magic_breath or
tim_water_breath), set_invis (pow slot, cleared on off), set_fast (speed_factor slot,
cleared on off), set_shield (power/opt/opt1/opt2 four slots), set_tim_thunder (p1/p2,
cleared on off), set_tim_regen (pow slot, cleared on off), set_mimic (form and level
slots).

The special cases SHALL be:

- `set_rush` - zeroing means The Bust (paralyzed 50-randint(lev) plus slow with the same
 base +50-randint(lev)); setting it stores rush=v and adds +v to hero/tim_deadly/strike
 each, magik(level/2) grants light_speed otherwise fast(v,10), and another magik(level/2)
 adds tim_esp;
- `set_parasite` on close spawns the parasite monster with 80% chance via
 place_monster_one inside scatter(10) (MSTATUS_ENEMY, food minus 750 floored at 100),
 otherwise announces the parasite dying;
- `set_mimic` on closing the Bear form hides SKILL_BEAR and runs select_default_melee, and
 sends PR_TITLE plus PU_BODY|PU_BONUS|PU_SANITY;
- `set_no_breeders` operates the global no_breeders.

The extra screen-refresh differences SHALL be: set_blind sends
PU_UN_VIEW|VIEW|MONSTERS|MON_LITE plus PR_MAP plus PR_BLIND plus PW_OVERHEAD; set_image
sends PU_MONSTERS plus PW_OVERHEAD|PW_M_LIST; set_lite sends PU_VIEW|PU_MONSTERS plus
PR_MAP; set_hero/set_shero additionally send PU_HP (shero additionally
PR_MAP/PU_MONSTERS/PW_OVERHEAD); set_meditation additionally sends PU_MANA; set_shadow
writes tim_wraith; set_tim_esp/set_tim_thunder/set_tim_invis/set_tim_infra additionally
send PU_MONSTERS; set_protevil/set_protgood/set_protundead/set_mental_barrier and the
oppose_acid/elec/fire/cold/pois four send no PU_BONUS, only handle_stuff.

- **Anchors**: `src/xtra2.c:19-51` (rush), `:58-124` (parasite), `:130-2923` (general
 setters and composite slots), `:1180-1233` (mimic)

### Requirement: Stun, Cut And Food Specials

`set_stun` SHALL force zero under PR1_NO_STUN and judge four bands by old and new values
(>100/50/0) - an upward change announces and, when randint(1000)<v or randint(16)==1,
deals a head strike: one third of the time INT+WIS, otherwise half the time only INT,
otherwise only WIS (sustain blocks each), all via do_dec_stat(STAT_DEC_NORMAL); a downward
change prints "no longer stunned"; sends PU_BONUS plus PR_STUN.

`set_cut` SHALL force zero under PR1_NO_CUT and judge eight bands (>1000/200/100/50/25/10/0)
with wording per band; on an upward change with the same probability, when CHR is not
sustained, "horribly scarred" subtracts CHR; sends PU_BONUS plus PR_CUT.

`set_food` SHALL clamp 0-20000 and judge six bands (PY_FOOD_FAINT/WEAK/ALERT/FULL/MAX and
Gorged) announcing both directions; dropping into Faint or Weak calls drop_from_wild;
sends PU_BONUS plus PR_HUNGER.

`drop_from_wild` SHALL, when the previous turn was in wild_mode (old_wild_mode) and the
player now stands in a wilderness sub-grid, record wilderness_x/y, call change_wild_mode,
and reset energy=100 and energy_use=0.

- **Anchors**: `src/xtra2.c:2931-3084` (stun), `:3092-3292` (cut), `:3294-3307`
 (drop_from_wild), `:3331-3506` (food)

### Requirement: Identity Switching And Rebirth

`switch_class`/`switch_subclass` SHALL swap p_ptr->pclass/pspec and re-hang the
cp_ptr/spp_ptr pointers. `switch_subrace` SHALL - **Quirk:** the bounds check joins its
two comparisons with `&&` (`(racem < 0) && (racem >= max_rmp_idx)`), so the guard can
never fire and no subrace index is ever rejected; with copy_old and the target
SUBRACE_SAVE, the current subrace is first copied into the savefile slot (title/desc kept
in place) before the pointers change. get_subrace_title/set_subrace_title SHALL edit
race_mod_info title directly. `do_rebirth` SHALL set expfact=r_exp+rmp_exp+c_exp, sum the
three hit dice, re-roll hit points with do_cmd_rerate, run check_experience and set
max_plv=lev, then send PR_BASIC plus PU_BONUS plus handle_stuff plus lite_spot.
`resolve_mimic_name` SHALL forward to the same-named Lua function. `set_grace` SHALL clamp
+-300000 and send PU_BONUS plus PR_PIETY plus handle_stuff.

- **Anchors**: `src/xtra2.c:7711-7748` (the three switchers), `:7750-7758` (titles),
 `:7763-7784` (rebirth), `:7789-7795` (mimic name), `:7398-7406` (grace)
