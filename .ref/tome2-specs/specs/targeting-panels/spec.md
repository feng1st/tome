# targeting-panels Specification

## Purpose

View porting and targeting: `src/xtra2.c` hosts the panel scroll system
(get_screen_size / panel_bounds / change_panel / verify_panel / resize_map /
resize_window), the target and look systems (target_able / target_okay, ang_sort,
target_set_prepare / aux / set, target_object), the three direction prompts
(get_aim_dir / get_rep_dir / get_hack_dir) with the tgt_pt point picker, and the
look_mon_desc health wording. The gamepad key mapping `get_keymap_dir` is
documented in specs/util/spec.md.

## Requirements

### Requirement: Panel Scrolling

The sizing helpers SHALL work as follows: `get_screen_size` derives the visible
area from ROW_MAP/COL_MAP (width halved under `use_bigtile`); `panel_bounds` derives
the panel_row/panel_col min/max/prt values from it. `change_panel` SHALL move half a
screen per step in the given direction, clamped to the map, and on a change request
`PU_MONSTERS`, `PR_MAP`, and `handle_stuff`. `verify_panel` SHALL center on the
player when `center_player` is set (clamped at the edges); otherwise it scrolls in
half-screen steps once the player strays two grids vertically or four grids
horizontally from the panel edge, disturbing play when `disturb_panel` is set and
the view is not centered; the `#if 0`-ed automatic object detection for shops is
disabled. `resize_map` SHALL, when a level has been generated (`character_dungeon`), mark the
panel invalid, run `verify_panel`, request the full notice/update/redraw sets plus
`Term_redraw`; `resize_window` SHALL, under the same gate, re-trigger React and then
request every `PW_*` bit plus `handle_stuff`.

#### Scenario: Edge-triggered half-screen scroll

- **WHEN** `center_player` is off and the player comes within two grids of the
 panel's top or bottom edge
- **THEN** the panel scrolls in half-screen steps until the player is no longer
 in the edge zone

- **Anchors**: `src/xtra2.c:4890-4921` (size and bounds),
 `src/xtra2.c:4931-4974` (change_panel), `src/xtra2.c:4985-5121` (verify_panel),
 `src/xtra2.c:5127-5170` (map), `src/xtra2.c:5176-5200` (window)

### Requirement: Targeting And Looking

The eligibility helpers SHALL work as follows: `target_able` requires a live,
visible (ml), projectable, non-hallucinated, non-friendly monster without
`RF7_NO_TARGET`; `target_okay` is always true for a fixed point target
(target_who < 0) and requires `target_able` for a monster target, tracking its
movement into target_row/col. `look_mon_desc` SHALL use the "damaged" wording family
for undead, demons, non-living creatures, and the `Egv` display characters, with
five health grades (unhurt / somewhat wounded at 60 / wounded at 25 / badly wounded
at 10 / almost dead). The sort helper SHALL be `ang_sort`, an in-place quicksort
over the temp array driven by the `ang_sort_comp`/`ang_sort_swap` hooks, with the
distance hook ranking by the doubled diagonal distance.

`target_set_accept` SHALL
accept the player grid unconditionally, visible monsters, marked objects, and —
under `CAVE_MARK` — trapped grids or `FF1_NOTICE` terrain (open doors, broken
doors, and door segments count as uninteresting). `target_set_prepare` SHALL scan
the panel (with `expand_look` exempt from line of sight), filter through
`target_able` in TARGET_KILL mode, and sort by distance. `target_set_aux` SHALL:

- show "something strange" under hallucination, and sleeping MIMICs by their held
 object;
- for a visible monster, track it, show the health bar, call `handle_stuff`, and on
 r enter the `screen_roff` recall loop, with a description line carrying level /
 health condition / quest / clone / state suffixes (neutral, pet, coaligned,
 companion, partial);
- then list the marked floor objects;
- show known traps by their `t_info` ident or "an unknown trap";
- show terrain with the grid mimic taking priority, `FEAT_NONE` when not visible,
 `FEAT_SHOP` as the shop name with the "the entrance to the " prefix, `FEAT_MORE`
 with a special flag using the `d_info` text, and wilderness `FEAT_TOWN` using the
 `wf_info` name (see note);
- show known fountains with a potion name as their info;
- append `(feat:mimic:special)` in wizard mode.

`target_set` SHALL run a two-mode loop — interesting mode (t/./5/0 locks the
target, space/*/+/- page through choices under `expand_list`, p recenters and
recomputes then falls through into free mode, o switches to free mode, direction
keys hop monsters via `target_pick`
or change panel and recompute, rolling back on failure); free mode (t records
target_who = -1 as a fixed point, directions move grid by grid with panel changes
as needed, wizard mode crossing the whole map, otherwise clamped inward by one
grid); on exit it clears the temp array, runs `verify_panel`, and requests the three
refresh sets, with a non-zero target_who marking success.

#### Scenario: Living monster health wording

- **WHEN** the look command describes a living monster whose hit points are
 between 25% and 59% of its maximum
- **THEN** the description reads "wounded" (the "damaged" family for undead,
 demons, non-living creatures, and `Egv` characters)

- **Anchors**: `src/xtra2.c:5208-5251` (health wording),
 `src/xtra2.c:5263-5316` (sorting), `src/xtra2.c:5337-5398` (eligibility),
 `src/xtra2.c:5408-5518` (distance hook and point picking),
 `src/xtra2.c:5524-5628` (accept and prepare), `src/xtra2.c:5631-6087` (aux and
 target_object), `src/xtra2.c:6133-6508` (target_set)

### Requirement: Direction Prompts

`get_aim_dir` SHALL take the direction from the repeat stack when active (5
requires `target_okay`), keep `command_dir` in force across calls, lock onto the
existing target under `use_old_target`, scale its prompt to whether a target
exists, accept t/T/./5/0 to take the target, route * through
`target_set(TARGET_KILL)` and the rest through the keymap, ring the bell and
re-prompt when direction 5 is given without a valid target, force a random
direction while confused (playing "You are confused."), and finish with
`repeat_push`. `get_rep_dir` SHALL share the same skeleton but refuse direction 5
and, while confused, randomize with 75% probability. `get_hack_dir` SHALL be the
aim skeleton without the repeat stack and the automatic target lock. `tgt_pt`
SHALL implement cursor picking — space confirms, directions move the cursor grid
by grid (bouncing back at the panel or map edge), ESCAPE cancels.

#### Scenario: Bell on targetless direction 5

- **WHEN** `get_aim_dir` receives direction 5 (or t/./0) and no valid target
 exists
- **THEN** the bell rings and the prompt repeats

- **Anchors**: `src/xtra2.c:6523-6640` (aim), `src/xtra2.c:6660-6732` (rep),
 `src/xtra2.c:7254-7348` (hack), `src/xtra2.c:7178-7233` (tgt_pt)
