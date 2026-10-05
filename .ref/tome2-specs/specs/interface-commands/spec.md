# interface-commands Specification

## Purpose

Interface commands: `src/cmd4.c` hosts the meta interface — full screen redraw,
the character sheet and the tactic/movement setting switchers, message recall, the
option system (regular / cheat / autosave / window flags / delay and hitpoint
warning pages), preference file loading and dumping, macros and keymaps, the visual
and color editors, notes and version, the level feeling report, the screen dump, the
knowledge menus (artifacts / uniques / objects / kills / recall depths /
corruptions / pets / quests / fates / traps / dungeon towns / notes), the time
report, and the macro recorder. The full player description sheet (display_player)
is documented in specs/player-display/spec.md.

## Requirements

### Requirement: Screen Redraw And Character Sheet

The full screen redraw SHALL trigger `TERM_XTRA_REACT`, schedule the pack combine
and reorder notices, request the full update and redraw masks, request all nine
window bits, and then redraw every terminal. The character sheet SHALL cycle six
display modes: c changes the name (get_name), f dumps the character to a file
(file_character), n/p change the page; the first page supports t/T to step the
tactic setting and e/E to step the movement setting (each wrapping over nine
values, refreshing `PU_BONUS`).

#### Scenario: Tactic step from the character sheet

- **WHEN** the character sheet shows the first page and the player presses t
- **THEN** the tactic setting steps down one, wrapping from 0 to 8, and
 `PU_BONUS` is applied before the prompt line is cleared

- **Anchors**: `src/cmd4.c:27-92` (redraw), `src/cmd4.c:98-214` (character
 sheet), `src/cmd4.c:4603-4627` (tactic and movement settings)

### Requirement: Message Recall

The single-message recall SHALL re-display the most recent message. The message
history SHALL support line-by-line and page-by-page scrolling up and down, ten-line
jumps with + and -, horizontal scrolling in half-screen steps, setting a highlight
string (= key) and searching forward for it (/ key, jumping on a hit and installing
it as the highlight), with the buffer height scaled to the screen.

#### Scenario: Forward search from the history browser

- **WHEN** the player presses / and enters a finder string that matches a later
 message
- **THEN** the browser jumps to the first matching message and the finder string
 becomes the highlight

- **Anchors**: `src/cmd4.c:220-226` (single message), `src/cmd4.c:247-449`
 (history browser)

### Requirement: Option System

The option main menu SHALL dispatch the pages below, and the option dump SHALL
append to a preference file in the user directory.

- The five regular option pages (interface / disturb / game-play / efficiency /
 ToME) plus the read-only birth options page, navigated with keymap direction
 keys, y/n to set.
- The cheat page's six switches (peek object / monster / dungeon / misc,
 omniscience, immortal); setting one ORs (page * 256 + bit) into the `noscore`
 no-scoring marker.
- The autosave page's two switches and the frequency stepper (50 up to 25000 in
 ten ring steps, wrapping back to 0).
- The window flag matrix (8 windows x 16 flags): keymap direction movement, y/n
 to set, and `t` to clear the current flag from every window and every flag
 from the current window, falling through to set the current cell (ignored on
 the main screen); on exit each changed window is erased and refreshed.
- The delay factor and hitpoint warning as 0-9 steppers.
- The dump writes Y:/X: lines (skipping the birth page) and a `W:` line per
 live window per defined flag.
- The by-name setter `change_option` is available to scripts (an unknown name
 logs a violet warning).

#### Scenario: Activating a cheat switch

- **WHEN** the player answers y to a cheat switch
- **THEN** the switch reads yes and `noscore` gains the switch's
 (page * 256 + bit) value

- **Anchors**: `src/cmd4.c:456-580` (cheat), `src/cmd4.c:583-728` (autosave,
 frequency ring `src/cmd4.c:589-602`), `src/cmd4.c:731-750` (change_option),
 `src/cmd4.c:755-876` (pages), `src/cmd4.c:882-1054` (window matrix),
 `src/cmd4.c:1061-1151` (dump), `src/cmd4.c:1199-1459` (main menu)

### Requirement: Preference Lines And Macros

A preference line entered through `do_cmd_pref` SHALL be handed straight to
`process_pref_file_aux`. The macro screen SHALL load a preference file, manage
macros and keymaps, and edit the action buffer. The macro recorder SHALL record
from a trigger key press to the next press of the same key, then attach the
result to a requested trigger key after confirmation.

- Macro add / remove / query: the trigger key sequence is captured without
 macro processing; the action string converts between text and ascii form.
- Keymap add / remove / query: one 256-entry table each for the original and
 roguelike modes.
- The dump writes A:/P: lines for macros and A:/C: lines for keymaps.
- The recorder stops on the second press of the trigger key, which is trimmed
 from the tail, then asks for the trigger key after confirmation and attaches
 the result with `macro_add`.

#### Scenario: Recorder stop and attach

- **WHEN** the player presses the trigger key again to stop a recording and
 answers yes to the confirmation check
- **THEN** the trailing copy of that key is removed and the recorded string is
 attached to the newly requested trigger with `macro_add`

- **Anchors**: `src/cmd4.c:1468-1481` (preference line), `src/cmd4.c:1489-1722`
 (dump), `src/cmd4.c:1733-2112` (macro screen), `src/cmd4.c:4749-4808`
 (recorder)

### Requirement: Visual And Color Editors

The visual editor SHALL dump the monster, object, and terrain attr/char tables
(R:/K:/F: lines) and adjust each entry's attr and char (n/N to change entry, a/A and
c/C to adjust values, d on terrain to restore the default), plus a whole-table
`reset_visuals`. The color editor SHALL dump V: lines and adjust each color channel
by K/R/G/B steps (repainting immediately via REACT), with optional gamma correction
(1.0-2.5, folded through `gamma_val`).

#### Scenario: Terrain default restore

- **WHEN** the terrain editor shows an entry and the player presses d
- **THEN** the entry's current attr/char are reset to its default attr/char

- **Anchors**: `src/cmd4.c:2118-2595` (visuals), `src/cmd4.c:2601-2896` (colors)

### Requirement: Notes, Version, And Level Feeling

Notes SHALL take up to 60 characters and, when `take_notes` is on, be written to the
notes file, otherwise be merged into the message area. The version display SHALL
read the module author and mail from the module script and run `patchs_display`. The
level feeling SHALL: report the fate sense ("You feel that you will meet your fate
here.") when `fate_flag` is set outside special levels and quests; yield to
`HOOK_FEELING` when a hook handles it; under `DF2_DESC` report the special level
description subject to its conditions (get_level_desc); report the fixed quest
text on quest levels; and on dungeon levels report one of the eleven feeling texts
(a town level inside a dungeon reports the market sounds instead). The screen dump
SHALL write the character layer and the color layer (encoded with
`dwsorgbuDWvyRGBU`) to dump.txt and read it back (`screendump_aux` can be replaced
by a graphical dump).

#### Scenario: Fate sense overrides the feeling

- **WHEN** `do_cmd_feeling` runs with `fate_flag` set on a level that is neither
 special (`DF2_SPECIAL` off) nor a quest
- **THEN** "You feel that you will meet your fate here." is printed before any
 other feeling output

- **Anchors**: `src/cmd4.c:2903-2926` (notes), `src/cmd4.c:2932-2944` (version),
 `src/cmd4.c:2951-3028` (feeling), `src/cmd4.c:3035-3287` (dump)

### Requirement: Knowledge Menus

The knowledge menu SHALL dump each entry to a temporary file shown via
`show_file` — eleven entries are always offered, plus a twelfth when
`take_notes` is on.

- Seen artifacts: those with a non-zero `cur_num`, excluding unidentified ones
 on the current floor, carried by monsters, or in the player's inventory
 (pack and equipment); the plain `TR3_NORM_ART` artifact kinds are listed
 from a separate table.
- Known uniques: insert-sorted by level, Morgoth — r_idx 862 — always sunk to
 the bottom with a 20000 level key, dead ones in red and always listed;
 living ones are listed only when first-sighted, or all of them under
 cheat omniscience.
- The recognized flavor-marked objects.
- The kill counts: a unique counts one once `max_num == 0`, normal races by
 `r_pkills`, with the `plural_aux` English plural rules and their ten-odd
 special cases.
- The recall depth table (the current recall dungeon star-marked).
- The corruption list (dump_corruptions).
- The current pets: uniques in green, pet/companion markers, the upkeep
 percentage derived from total pet levels clamped to 10-100, with the divisor
 at fifteen under Perfect Casting.
- The quest states, sorted by danger: TAKEN shows name and description,
 COMPLETED shows finished-but-unrewarded; random quests show the princess or
 lost sword text and counters for the current level; `dynamic_desc` quests
 dispatch to the Lua `__quest_dynamic_desc`.
- The fate list (dump_fates).
- The known traps.
- The dungeon random towns (`TOWN_KNOWN` ones).
- The notes file (only offered while `take_notes` is on).

#### Scenario: Notes entry follows the take_notes option

- **WHEN** the knowledge menu is drawn with `take_notes` off, then again with it
 on
- **THEN** the menu lists eleven entries in the first case and twelve in the
 second, the extra one being the notes display

- **Anchors**: `src/cmd4.c:3293-3542` (artifacts), `src/cmd4.c:3548-3589`
 (traps), `src/cmd4.c:3597-3729` (uniques), `src/cmd4.c:3732-3818`
 (plurals), `src/cmd4.c:3824-3901` (pets), `src/cmd4.c:3910-4030` (kills),
 `src/cmd4.c:4036-4089` (objects), `src/cmd4.c:4095-4185` (depths and towns),
 `src/cmd4.c:4191-4215` (corruptions), `src/cmd4.c:4221-4358` (quests),
 `src/cmd4.c:4364-4400` (fates and notes), `src/cmd4.c:4406-4571` (menu),
 `src/cmd4.c:4578-4597` (quest quick list)

### Requirement: Time Report

The time report SHALL state the date in the elven calendar (get_month_name/get_day)
and the time of day on a twelve-hour clock; the descriptive sentence comes from
timefun.txt (on a one-in-ten chance or while hallucinating) or timenorm.txt, chosen
by the S:/E:/D: time-segment ranges (several D: lines within a segment are picked
by the in-line randomizer).

#### Scenario: Fun time description roll

- **WHEN** the time is reported and the one-in-ten roll hits or the player
 hallucinates
- **THEN** the descriptive sentence comes from timefun.txt instead of
 timenorm.txt

- **Anchors**: `src/cmd4.c:4633-4742`
