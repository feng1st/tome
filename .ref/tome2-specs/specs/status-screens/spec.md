# status-screens Specification

## Purpose

Status overview screen: `src/status.c` hosts the character status query interface
(`status_main`, eight pages) — one page each presenting the flag matrix item by
equipment slot (statistics / movement / combat / resistances / misc / curses / sight
/ companions). The flag aggregation helpers (`object_flags`, `player_flags`) are
documented in specs/object-core/spec.md and specs/ui-frames/spec.md; this spec
records the screen structure and the coloring rules.

## Requirements

### Requirement: Main Menu And Matrix Skeleton

`status_main` SHALL present an eight-item menu (1 Statistics / 2 Movement / 3 Combat
/ 4 Resistances / 5 Misc / 6 Curses / 7 Sight / 8 Companions, quit with q or ESCAPE),
enter it with `character_icky` set and the screen saved, and on exit request the four
redraw groups `PR_WIPE`, `PR_BASIC`, `PR_EXTRA`, and `PR_MAP`. The matrix builder
`az_line` SHALL print one lowercase slot letter per worn item (starting at a) on row
2, take each item's full flag set through `object_flags`, and finish with an `@`
column holding the `player_flags` summary; element 6 of `flag_arr` (the seventh)
marks that the column carries an item and is to be displayed.

#### Scenario: Opening and leaving the status screen

- **WHEN** the player presses 6 in the status menu and then q
- **THEN** the curses page is shown and, after quitting, the screen is restored
 with `PR_WIPE`, `PR_BASIC`, `PR_EXTRA`, and `PR_MAP` requested

- **Discrepancy:** the live call uses `object_flags` rather than
 `object_flags_known` — a debug state flagged by the `DGDGDGDG` comment and the
 `Help me debug` note — so unidentified flags leak into the matrix.

- **Anchors**: `src/status.c:376-435` (main menu), `src/status.c:437-473` (az_line)

### Requirement: Row Renderers

The value renderers SHALL work as follows: `status_bival` prints a light-blue `+`
when the flag is present and a white `.` otherwise; `status_trival` prints `*` for
the first flag, `+` for the second, `.` when neither is present; `status_numeric`
prints `.` for zero, negative values in red and positive values in green, with an
absolute value over 9 shown as `*`; `status_count` sums four flag tests with their
weights and feeds the result to `status_numeric`. The row helpers
`row_bival`/`row_trival`/`row_npval`/`row_count` SHALL emit one cell per column whose
`flag_arr` element 6 is set (`row_npval` degrades to a yellow `*` on the player
column — `player_flags` carry no pval to display — and shows the item pval for
objects). `statline` SHALL show the current stat value via `cnv_stat` in the first
column, color each slot light blue when the matching SUST bit is present, print a
yellow `*` (or white `.`) on the player column, and switch the row-name color once a
sustain is seen; `row_hd_bon` SHALL show to-hit or to-damage selected by its `which`
argument (the player column is always `.`).

#### Scenario: Negative pval cell

- **WHEN** a `row_npval` row checks a worn item whose pval is -3 and the row's
 flag is present
- **THEN** the cell shows a red 3 (a magnitude over 9 would show as red `*`)

- **Anchors**: `src/status.c:475-532` (value renderers), `src/status.c:534-611`
 (row helpers), `src/status.c:613-714` (statline and hd_bon)

### Requirement: The Eight Pages

The pages SHALL render the following rows:

- `status_attr` — six stat rows plus the Luck (`TR5_LUCK`), Life (`TR2_LIFE`), and
 Mana (`TR1_MANA`) pval rows.
 - **Quirk:** the Wisdom row passes `A_INT` instead of `A_WIS`, so it displays the
 Intelligence value (`src/status.c:58`).
- `status_move` — the Fly/Lev trival row (`TR4_FLY` or `TR3_FEATHER`), then Climb,
 Dig, Speed, Wraith, Stealth, Telep.
- `status_sight` — SeeInvis, Invis, Infra, Search, AutoID, the Light row (weighted
 sum of `TR3_LITE1` / `TR4_LITE2` / `TR4_LITE3`), Full ESP, and thirteen ESP family
 rows (orc, troll, dragon, giant, demon, undead, evil, animal, thunderlord, good,
 spider, nonliving, unique).
- `status_item` — left column Sh.fire / Sh.elec / Regen / SlowDigest / Precog /
 Auto.Id / Spell.In; right column (at `row_x_start = 40`) Blessed, Activate,
 EasyKnow, HideType, the four Safe rows (acid / elec / fire / cold), ResMorgul.
- `status_combat` — the pval rows Spell, Blows, Crits, Ammo_Mgt, Ammo_Sht; the brand
 block Vorpal, Quake, Chaotic, Vampiric, Poison, Acidic, Shocks, Burns, Chills,
 Wound (ten rows); right column No.Blow, the three S/K composite rows (undead,
 demon, dragon), five plain slay rows (orc, troll, giant, evil, animal — eight
 slay-family rows in total), and the To-Hit / To-Dmg rows.
- `status_res` — Fire, Cold, Acid, Lightning as immunity/resistance trival rows;
 then Poison, Lite, Dark, Sound, Shards, Nether (a fifth trival row via
 `TR4_IM_NETHER`), Nexus, Chaos, Disen., Confusion, Blindness, Fear, and the
 utility rows Free Act / Reflect / Hold Life.
- `status_curses` — the composite rows Hvy/Nrm, DG/Ty, Prm/Auto plus Perma and
 NoDrop; then Black Breath, Dr.Exp, Dr.Mana, Dr.HP, No Hit, NoTelep, NoMagic,
 Aggrav, Clone, Temp (ten rows); and the Antimagic four-tier count (`TR4_ANTIMAGIC_50/30/20/10`
 weighted 5/3/2/1).
- `status_companion` — SHALL write every `MSTATUS_COMPANION` monster into a
 temporary file (name, level/experience, next-level requirement, hp, AC, speed
 relative to 110, up to four blows as dice pairs), display it with `show_file`, and
 delete the file afterwards.

- **Anchors**: `src/status.c:46-75` (attr), `src/status.c:77-113` (move),
 `src/status.c:115-164` (sight), `src/status.c:166-214` (item),
 `src/status.c:216-273` (combat), `src/status.c:325-374` (res),
 `src/status.c:275-323` (curses), `src/status.c:716-778` (companion)
