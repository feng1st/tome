# wizard-debug Specification

## Purpose

Wizard debug commands: `src/wizard2.c` hosts every command of the KTRL-A debug menu
`do_cmd_debug` — object and artifact creation, the object plaything (tweak / reroll
/ statistics / quantity), character editing, level and town jumps, summoning, full
curing, hitpoint rerolls, monster level and faction control, direct Lua calls, body
takeover and forced mimic form, and direct grants of fates, corruptions, and
rewards. Most commands take their parameter from `command_arg`.

## Requirements

### Requirement: Debug Command Table

`do_cmd_debug` SHALL dispatch on a single key: spoilers menu ("), status panel (A),
on-line help (?), full cure (a), teleport to target (b), change host monster (B, advised only for a
naked body), create object by index (-), create object by menu (c), create named
artifact by index (C, landing it after `apply_magic` at full strength), detect
everything (d), change dungeon (D, landing on its shallowest level), edit character
(e), set grid mana (E), fully identify (f), good acquisition (g), reroll hitpoints
(h), identify (i), jump level (j, clamped to the current dungeon's limits, honoring
the autosave option), self-knowledge (k), induce object Awareness up to a level
threshold (l), magic mapping (m), random corruption (M), random reward (r), place a
trap underfoot (R), summon a friendly named monster (N), summon a hostile named
monster (n — `m_allow_special` overrides the unique limits, ten placement
attempts), object plaything (o), phase door (p), panic save (P), take a quest (q,
setting TAKEN, hooking the plot line, and triggering the C-type init), mark the
whole map known plus wizard light (u), undead form (U), monster horde (H, in the
`MONSTER_HORDES` build), summon N monsters (s), set
grid special (S), teleport bypassing no-teleport (t), teleport to a town (T, finding
the wilderness coordinates and then changing level), very good acquisition (v),
wizard light (w), wish (W), add experience (x, gaining the current experience plus
one when no argument is given), wipe nearby monsters (z), Lua variable viewer (_), set mimic
form (*, 100 turns), gain a fate plus omniscience (+), change grid feature (F,
first reporting the trap, the old feature, and special), change monster faction (=),
raise a monster's level (@, through the experience mechanism), summon a type via
`summon_specific` (/), and execute a Lua string (>); unknown keys fall through to
`HOOK_DEBUG_COMMAND`.

- **Anchors**: `src/wizard2.c:1718-2055` (command table)

### Requirement: Object Plaything

`do_cmd_wiz_play` SHALL loop over a chosen object on a copy: accept (write back to
the original object and refresh); statistics (`wiz_statistics` sampling 100000
`make_object` rolls per normal / good / excellent quality, comparing same
tval/sval objects by the pval/to_a/to_h/to_d quadruple into match / better /
worse / other with a live display, and temporarily lifting the artifact uniqueness
limit for the run); reroll (`wiz_reroll_item`: `apply_magic` in the bad / normal /
good / excellent grades, or randart generation); tweak (`wiz_tweak_item`: pval,
pval2, pval3, the three to_* values, name2, name2b, sval, object experience
(recomputing the experience level for `TR4_LEVELS` objects), and timeout, refreshing
the debug page at each step); quantity (1-99); and apply magic (keeping both ego
slots while rerolling the base object). The debug page SHALL show the object's full
parameters and the 32-bit bitmaps of the first three flag groups (with character-axis
labels).

#### Scenario: Statistics roll run

- **WHEN** the player picks excellent quality in `wiz_statistics` for an artifact
- **THEN** the artifact's uniqueness marker is cleared for the run, 100000
 `make_object` rolls are made with good and great set, and the running
 match/better/worse/other counts are displayed every 100 rolls

- **Anchors**: `src/wizard2.c:1210-1343` (plaything), `src/wizard2.c:978-1163`
 (statistics), `src/wizard2.c:865-959` (reroll), `src/wizard2.c:778-859`
 (tweak), `src/wizard2.c:505-565` (debug page)

### Requirement: Helper Routines

`do_cmd_wiz_cure_all` SHALL remove curses, restore the six stats, restore drained
experience, refill hitpoints, sanity, and mana, refill the symbiote, clear the timed
states and Black Breath, satiate hunger, and redraw the whole screen.
`do_cmd_rerate` SHALL reroll the per-level hitpoint table from the hit dice (bounds
following the birth rules) and report the life rating.
`teleport_player_town` SHALL autosave per the autosave option and then find the
town's wilderness entrance coordinates to change level. `do_cmd_wiz_change` SHALL
interactively edit the six stats (3-118), gold, experience (through
`check_experience`), and the luck base value.

- **Anchors**: `src/wizard2.c:1435-1492` (cure all), `src/wizard2.c:90-150`
 (rerate), `src/wizard2.c:55-84` (town teleport), `src/wizard2.c:348-441`
 (character edit)
