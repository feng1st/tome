# ui-frames Specification

## Purpose

Screen frames and refresh dispatch: the display half of `src/xtra1.c` — the sidebar
field printers (the `prt_*` family), the main and extra frames
(prt_frame_basic/extra), the monster health bar (health_redraw), the sub-windows
(the `fix_*` family: nine refreshers over eight terminal slots), and the
five-level dispatch of
notice/update/redraw/window/handle. The derived stat calculations (the `calc_*`
family) are covered in specs/player-derive/spec.md; the drawing bodies themselves
(display_player / display_map / display_inven) belong to the cmd3/display side —
see specs/inventory-commands/spec.md and specs/player-display/spec.md.

## Requirements

### Requirement: Sidebar Fields

The field printers SHALL work as follows: `prt_field` clears 13 cells and then shows
light-blue text; `prt_piety` shows nothing without a god, light blue while praying
and green otherwise; `prt_sane` colors the sanity percentage in three tiers (full
green, above 10 x `hitpoint_warn` percent yellow, otherwise red); `prt_stat` colors
damaged stats (orange while `stat_cnt` is active, yellow for permanent loss), green
at full value, purple above the maximum, and appends `!` when `stat_max` reaches
18/100; `prt_title` takes the mimic shape's `show_name` from the Lua
`get_mimic_info`, shows `[=-WIZARD-=]` in wizard mode, `***WINNER***` under
`WINNER_NORMAL`, `***GOD***` under `WINNER_ULTRA`, and otherwise the class titles
stepped every 5 levels; `prt_level` shows `LEVEL` in green at maximum level and
`Level` in yellow otherwise; `prt_exp` shows the amount still needed for the next
level when the `exp_need` option is set (******** at the level cap) and colors the
value green when experience is at its maximum and yellow otherwise; `prt_ac` shows
`dis_ac + dis_to_a`; `prt_hp` switches the label to `DP ` with `TERM_L_DARK` in the
undead state and uses the light-blue / violet / light-red tiers there, uses
green / yellow / red against the `hitpoint_warn` threshold otherwise, and lights the
player's own grid when `player_char_health` is set; `prt_mh` shows the carried
monster's hit points (the `INVEN_CARRY` slot's pval2/pval3) with the same
three-tier scheme and clears the row when empty; `prt_sp` uses the same three tiers.

`prt_depth` SHALL right-align within 13 cells — blank in `wild_mode`, "Arena" in the
arena, the name from `get_dungeon_name` when it fires, "Special" under `DF2_SPECIAL`,
"Quest" inside quests, on the surface the `wf_info` feature name or else
"Town/Wild", times 50 with a trailing `ft` under `depth_in_feet`, a leading minus
sign under `DF1_TOWER`; orange while a word recall is pending. The status line
SHALL: `prt_hunger` in six grades (Faint red "Weak", Weak orange "Weak", Hungry
yellow, blank when fed, Full light green, Gorged dark green); prt_blind / prt_confused
/ prt_afraid / prt_poisoned as orange markers; `prt_dtrap` shows "DTrap" on
`CAVE_DETECT`; `prt_state` across five states (Paralyzed in red; Rest with the four
timed formats plus ***** for -1 and &&&&& for -2; Rep with the repeat count;
Searching; blank); `prt_speed` fast in light green, slow in light umber (in search
mode +10 is added first to visually undo the search slowdown before display);
`prt_study` shows "Skill" while unspent skill points remain; `prt_cut` with eight
wording and color grades, `prt_stun` with three.

#### Scenario: Damaged stat coloring

- **WHEN** a stat's current value is below its maximum, with `stat_cnt` running on
 it
- **THEN** the stat shows in orange, and in yellow once the drain is no longer
 temporary (no `stat_cnt`)

- **Anchors**: `src/xtra1.c:128-153` (field/piety), `src/xtra1.c:159-237`
 (sane/stat), `src/xtra1.c:245-339` (title/level/exp), `src/xtra1.c:345-489`
 (gold/ac/hp/mh/sp), `src/xtra1.c:495-571` (depth), `src/xtra1.c:577-694`
 (hunger and the four states and dtrap), `src/xtra1.c:704-855`
 (state/speed/study), `src/xtra1.c:858-918` (cut/stun)

### Requirement: Frames And Health Bar

`prt_frame_basic` SHALL draw the whole left column in one pass (race, class, title,
level, experience, the six stats, AC, HP, sanity, SP, piety, carried-monster HP,
gold, depth, plus the health bar); `prt_frame_extra` SHALL draw the lower row (cut,
stun, hunger, blind/confused/afraid/poisoned, DTrap, state, speed, study).
`health_redraw` SHALL, under `DRS_SHOW_HEALTH_BAR`, clear the bar when nothing is
tracked, show `[----------]` for an invisible or hallucinated target, and — for a
visible target — draw a `*` bar by health percentage (one cell below 10,
pct/10+1 below 90, full bar otherwise) with the state color overriding the health
color (light red at 10, orange at 25, yellow at 60, light green at 100; violet
afraid, blue asleep, green poisoned, red bleeding).

#### Scenario: Unseen or hallucinated target

- **WHEN** the tracked monster is not currently visible (`ml` off) or the player
 hallucinates
- **THEN** the bar shows `[----------]` instead of a health bar

- **Quirk:** the dead-target branch is always false as written — `!hp < 0` parses
 as `(!hp) < 0`, so it falls through to the regular bar drawing
 (`src/xtra1.c:963`).

- **Anchors**: `src/xtra1.c:936-1018` (health bar), `src/xtra1.c:1025-1069`
 (basic frame), `src/xtra1.c:1075-1099` (extra frame)

### Requirement: Sub-windows

The `fix_*` helpers SHALL activate each sub-window in turn (the 8 terminals,
filtered by `window_flag`), draw it, and restore the previous term:
fix_inven/fix_equip call display_inven/display_equip; fix_player calls
`display_player(0)`; fix_message prints the messages in reverse order and clears
each line's tail; fix_irc_message shows only messages of type `MESSAGE_IRC`;
fix_overhead calls display_map; fix_monster calls display_roff when a monster is
tracked; fix_object clears the window and runs `object_out_desc` on
`tracked_object`, printing "You see nothing special." when it has no description.
`fix_m_list` SHALL print "You can not see clearly" and return under
hallucination; otherwise it SHALL reset and recount `total_visible` per race
(`RF9_MIMIC` monsters count only when their held object is marked, all others by
ml), print "You see no monsters." when none are visible, and otherwise list each
visible race with an `(xN)` suffix where the count exceeds one, colored — uniques
light blue, ever-killed races whose level exceeds the current depth violet (red for
uniques), never-killed non-uniques green, and the rest slate — wrapped in 26-column
bands sized by the window height.

#### Scenario: Monster list under hallucination

- **WHEN** the monster list window refreshes while the player hallucinates
- **THEN** it shows "You can not see clearly" and no list is drawn

- **Anchors**: `src/xtra1.c:1105-1417` (the eight helpers),
 `src/xtra1.c:1421-1568` (monster list)

### Requirement: Refresh Dispatch

`notice_stuff` SHALL handle only `PN_COMBINE` (combine_pack) and `PN_REORDER`
(reorder_pack). `update_stuff` SHALL process, in order: `PU_BODY` to calc_body;
`PU_BONUS` clearing `PU_POWERS` first and then running calc_powers and
calc_bonuses(FALSE) (as-is: the BONUS branch swallows the POWERS bit);
`PU_TORCH`/`PU_HP`/`PU_SANITY`/`PU_MANA`/`PU_SPELLS`/`PU_POWERS` each to their
calc_*; the character-not-generated and icky checks stop the pass before the screen
group; the screen group SHALL map `PU_UN_VIEW` to forget_view, `PU_VIEW` to
update_view, `PU_FLOW` to update_flow, `PU_DISTANCE` to update_monsters(TRUE) while
also clearing `PU_MONSTERS`, `PU_MONSTERS` to update_monsters(FALSE), and
`PU_MON_LITE` to update_mon_lite() when `monster_lite` is set. `redraw_stuff` SHALL
be gated on the character-generated and icky checks, raise `HOOK_REDRAW`, and then:
`PR_WIPE` (msg_print(NULL) plus Term_clear), `PR_MAP`, `PR_BASIC` which folds in the
dozen-plus field bits before calling prt_frame_basic, the per-field bits
(MISC/TITLE/LEV/EXP/STATS/ARMOR/HP/MANA/PIETY/MH/GOLD/DEPTH/HEALTH each to its
prt_*), `PR_EXTRA` which folds the state bits before prt_frame_extra, and the
per-state bits (CUT/STUN/HUNGER/BLIND/CONFUSED/AFRAID/POISONED/DTRAP/STATE/SPEED/
STUDY/SANITY each to its prt_*). `window_stuff` SHALL OR the eight windows'
`window_flag` values into a mask and then dispatch the nine classes
PW_INVEN/EQUIP/PLAYER/M_LIST/MESSAGE/IRC/OVERHEAD/MONSTER/OBJECT to their fix_*.
`handle_stuff` SHALL run update, then redraw, then window.

#### Scenario: BONUS swallows POWERS

- **WHEN** `PU_BONUS` and `PU_POWERS` are both requested in the same
 `update_stuff` pass
- **THEN** `calc_powers` and `calc_bonuses(FALSE)` run inside the BONUS branch
 and the separate POWERS branch is skipped

- **Anchors**: `src/xtra1.c:4039-4058` (notice), `src/xtra1.c:4064-4166` (update),
 `src/xtra1.c:4172-4383` (redraw), `src/xtra1.c:4389-4491` (window and handle)
