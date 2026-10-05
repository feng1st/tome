# program-entry Specification

## Purpose

Program entry: `src/main.c` carries command-line parsing, path location, multi-user
directory preparation, the display backend dispatch chain, and the game start call. The
main-xxx.c platform backends are out of scope; this spec records only their dispatch
contract.

## Requirements

### Requirement: Paths And Multi-User Directory

`init_stuff` SHALL take the TOME_PATH environment variable (falling back to DEFAULT_PATH;
with ENABLE_BINRELOC it instead builds the path at runtime from DATADIR plus /tome/lib),
append the path separator when missing, and hand the result to `init_file_paths`. Under
PRIVATE_USER_PATH, `private_check_user_directory` SHALL validate or create, level by level
with 0700 permissions, the private directory, its version subdirectory, and the save
subdirectory; failure quits the program.

- **Anchors**: `src/main.c:74-130` (directory checks), `:153-183` (init_stuff)

### Requirement: Directory Override Option

`change_path` SHALL parse `-d<what>=<path>`: the first letter of `what` (case-insensitive)
selects the directory global to replace — a/f/h/i/u/x are always available; b/d/e/s (bone/
data/edit/save) are refused with "Restricted option" under VERIFY_SAVEFILE and overridable
otherwise; a missing equal sign or unknown letter quits with a `quit_fmt` message.

#### Scenario: Restricted override

- **WHEN** the savefile-verification build passes `-ds=/some/path`
- **THEN** the save directory override is refused with the "Restricted option" message

- **Anchors**: `src/main.c:196-298`

### Requirement: Command Line Parsing

`main` SHALL parse, in order: `-n` new character, `-f` arg_fiddle, `-w` arg_wizard,
`-v` arg_sound, `-g` arg_graphics, `-r` force the rogue-like keyset, `-o` force the
original keyset, `-s<num>` number of high scores to show at start (default 10),
`-u<who>` player name and savefile base name (and set no_begin_screen), `-m<sys>` select
the display backend, `-M<which>` force the module (force_module), `-h` usage, `-H <files...>`
convert help files to HTML (after init_lua, each file is split at its first period into
base name and extension and wrapped with head.aux/foot.aux for `txt_to_html`, then the
program returns), `-c <f1> <f2>` convert a changelog to text (`chg_to_txt`, then return),
`-d` directory override, `--` ends the standard options and shifts the remaining arguments
to the platform init_*. Under SET_UID additionally: umask 022, player_uid from getuid
(VMS adds gid x 1000), euid/egid stashed, the check_time and check_load double gate
("The gates to Angband are closed"), safe_setuid_drop as the normal state, and user_name
supplying the default player name.

- **Anchors**: `src/main.c:308-460` (prelude and SET_UID), `:465-744` (argument loop and usage dump)

### Requirement: Startup Sequence

After parsing the game SHALL: run process_player_name(TRUE); install quit_aux as quit_hook
(on quit, term_nuke over the eight windows); run zsock_init (a comment states it must
precede main-net); then try display backends in order by compile switch — glu/gtk2/gtk/
xaw/x11/gcu/cap/dos/ibm/emx/sla/lsl/ami/vme/pgu/iso/lua/net/sdl/dmy, where a `-m` match is
exclusive and each success sets ANGBAND_SYS, and total failure quits with "Unable to
prepare any 'display module'!"; run signals_init; run init_angband (boot-loading, see
specs/boot-loading/spec.md); with show_score, run display_scores and then pause_line; call
play_game(new_game) to enter the main loop; under CHECK_MEMORY_LEAKS run the leak check at
shutdown.

#### Scenario: No usable display backend

- **WHEN** every compiled display backend fails to initialize
- **THEN** the program quits with "Unable to prepare any 'display module'!"

- **Quirk:** the glu backend is attempted a second time between gcu and cap (a duplicated
 dispatch block in the shipped source).

- **Anchors**: `src/main.c:33-46` (quit_hook), `:746-1074`
