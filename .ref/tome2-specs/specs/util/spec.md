# util Specification

## Purpose

The utility layer: `src/util.c` carries the cross-file infrastructure — the path and file
descriptor families, the text/ascii escape conversions, the macro system (the matching,
expansion, and key-entry pipeline), the quark inscription pool, the message ring buffer and
top-line display, the screen save stack, the input helper family (askfor/get_string/
get_check/get_quantity/request_command), wrapped text output, time decomposition, and
miscellany. The terminal abstraction layer Term_* is registered with the platform layer.

## Requirements

### Requirement: Paths And Files

Path handling SHALL: `path_parse` expand ~user/ and ~/ to the user's home directory;
`path_build` use ~-prefixed and absolute paths directly, otherwise join directory plus
separator; `path_temp` take a system temporary name. The file descriptor family SHALL:
fd_make (O_CREAT|O_EXCL|O_WRONLY|O_BINARY), fd_open, fd_lock (flock or lockf exclusive
lock, for the high score board), fd_seek/fd_read/fd_write (16K chunks with a byte-count
check), fd_close, fd_kill/fd_move; fd_copy is an empty shell that always fails
(as-shipped). my_fgets SHALL swallow all three of DOS/Mac/Unix line endings uniformly,
expand tabs to eight-column stops, and strip non-printable characters; my_str_fgets reads a
memory string line by line with a static cursor (passing NULL resets it).

- **Anchors**: `src/util.c:193-364` (paths), `:740-1011` (fd family, the stub at `:708-724`), `:530-597` (fgets), `:427-521` (str_fgets)

### Requirement: Escape Conversion And Macro Triggers

`text_to_ascii` SHALL parse escape sequences: \\x hexadecimal, \\0 through \\3 as two
octal forms, \\\\ \\^ \\s space \\e ESC \\b \\n \\r \\t, and ^X control codes;
`\[name]` goes through macro-trigger expansion — the modifier table sets bits by name
match, the trigger name is looked up for its key code, and the template (with & injecting
the modifier characters and # injecting the key code) composes the action string wrapped in
ascii 31/13; an unknown trigger name emits the 31+13 placeholder. `ascii_to_text` SHALL
reverse this: triggers starting with 31 are restored (keycode back to trigger name), while
everything else is escaped character by character as \\e/\\s/\\b/\\t/\\n/\\r/\\\\/\\^,
^X (below 32), \\0 octal (below 64), and \\x hexadecimal.

- **Anchors**: `src/util.c:1147-1236` (trigger expansion), `:1246-1368` (text_to_ascii), `:1371-1430` (trigger restore), `:1436-1521` (ascii_to_text)

### Requirement: Macro System

The macro table SHALL be stored as parallel pattern/action arrays with macro__use
accelerating lookup by first byte. `macro_add` SHALL overwrite the action on the same
pattern, otherwise append. `inkey_aux` SHALL: after taking the first key, wait up to one
hundred milliseconds (stepping ten at a time) for continuation keys when a prefix-matching
macro exists; the longest complete match wins — surplus keys are pushed back on the queue,
ascii 30 is pushed as a boundary before the action string, and nested macros are stopped by
the 30. `inkey` SHALL handle the global parameters: inkey_xtra delayed flush (clearing the
macro state and the key queue), inkey_scan returning zero when no key is ready, inkey_base
bypassing macros (for recording), inkey_flag command-state cursor rules; a backtick
converts to ESC; ctrl-] (or ctrl-d under the original keyset) triggers an HTML dump;
ascii 31 starts trigger collection until a low-code key; the inkey_next keymap stream
replays characters without triggering macros; the Borg hook inkey_hack can steal input;
every key passes through macro_recorder_add for the recorder. The screen save SHALL use a
depth counter kept in step with character_icky.

#### Scenario: Continuation wait

- **WHEN** the first typed key is a prefix of a stored macro pattern and no further key
 arrives within the stepped 100-millisecond window
- **THEN** the longest complete match wins if there is one, otherwise all typed keys
 are pushed back on the queue in order

- **Anchors**: `src/util.c:1538-1731` (macro table), `:1816-1949` (inkey_aux), `:2039-2275` (inkey), `:2882-2918` (screen save)

### Requirement: Inscription Pool And Message Buffer

The quark pool SHALL deduplicate globally (the same string shares one index) and return the
zero empty string when QUARK_MAX is full. The message ring buffer SHALL: reject overlong
messages (a quarter of the buffer or more); count up on a repeat of the last entry
(replay appends `<Ndx>`); deduplicate near history — a same-text message within a few
entries reuses the old offset; when the buffer tail runs out of space, clean dead messages
and wrap the head, with the tail advancing past any straddled content to just beyond its
terminator; the ring indexes
and last/next pointers maintain the entry cap. The top-line display `cmsg_print` SHALL:
show -more- and wait for a key when the pending content exceeds the width minus eight
(quick_messages takes any key, otherwise only space, return, and ESC work), wrap long text
at spaces between columns forty and seventy-two, refresh the message window instead of
waiting when auto_more is on, and clear the line on an empty argument; `display_message`
SHALL support `#X` inline color changes (## escapes the hash). msg_format/cmsg_format are
vararg wrappers.

#### Scenario: Message repeat counting

- **WHEN** a new message has the same text as the last entry in the ring
- **THEN** the entry's count increments instead of a second entry appearing, and the
 replay shows the count as a `<Ndx>` suffix

- **Anchors**: `src/util.c:2299-2339` (quark), `:2370-2660` (message buffer), `:2667-2876` (top-line display)

### Requirement: Input Helpers

`askfor_aux` SHALL edit at the cursor: the default shown in yellow, printable characters
appended, backspace trimming the tail, ESC clearing and failing, return accepting; in
completion mode (askfor_aux_complete) tab runs `complete_command` against the CLI table by
prefix — the first full match is filled in and candidates listed in a side panel, further
keys converge on the common prefix. `get_string`/`get_check` (with [y/n], Y/y meaning yes) /
`get_com` (ESC meaning no) are the basic three. `get_quantity` SHALL pass command
arguments straight through and clear them, cache on the repeat stack, treat a letter's
first key as select-all, and clamp between zero and max. `request_command` SHALL: enter
count input on '0' (capped at 9999, zero counting as ninety-nine, space asking again for
the command), bypass keymaps on a backslash, hand-enter control codes after ^, expand
through the keymap table for the current keyset mode (store mode checks the ignore table
and skips), auto-repeat TBDoc+ family commands ninety-nine times under always_repeat, and
finally scan equipment inscriptions for `^X` and `^*` interception with a second
confirmation (a refusal changes them to a space). `get_count`/`get_number` SHALL edit
digits in place (backspace deletes, overflow rings the bell and clamps to max, ESC zeroes).

#### Scenario: Count input

- **WHEN** the player types '0' followed by digits before a command key
- **THEN** the digits form the command count capped at 9999 (bell past 1000), with a
 bare zero counting as ninety-nine

- **Anchors**: `src/util.c:3355-3467` (askfor), `:3300-3340` (completion), `:3480-3663` (the basic three and quantity), `:3718-4005` (request_command), `:4238-4347` (number editing)

### Requirement: Text Output And Interface Pieces

`text_out_to_screen` SHALL wrap at the screen width (or text_out_wrap), carrying whole
words to the next line and preserving the text_out_indent; `text_out_to_file` SHALL wrap at
seventy-five columns with indentation and clean trailing spaces across line ends. The
interface pieces SHALL: draw_box (blue frame), display_list (scrolling list with a centered
title and colored selection), input_box (centered input frame), msg_box (centered question
frame), and ask_menu (a twenty-row paginated letter menu with plus/minus paging).

- **Anchors**: `src/util.c:3031-3130` (screen wrapping), `:3145-3252` (file wrapping), `:4742-4821` (frame pieces), `:4630-4701` (menu)

### Requirement: Time And Miscellany

Time decomposition `bst` SHALL convert game turns (with the DAY_START offset) into
MINUTE/HOUR/DAY/YEAR; `get_month_name` takes its name from the nine-segment calendar table
(Yestare/Mettare as single-day festivals in full-name mode) and `get_day` prints English
ordinals. The lookup helpers SHALL: `test_monster_name`/`test_mego_name`/`test_item_name`
do case-insensitive full-name reverse lookup to an index (zero for no hit) and
`get_keymap_dir` extract a direction digit from the keymap (five becomes zero). The
miscellany SHALL: repeat_push/pull/check (the 'n' key replays the previous command,
capacity twenty), is_a_vowel, the value_scale and rescale scaling pair, count_bits,
strlower, get_player_race_name (ordering by the subrace place bit), the scansubdir wrapper,
the Lua timer chain new_timer/del_timer (callback string + delay/countdown + enabled,
driven by the process_world step, every 10 game turns); the gamma table is built as an integer
Taylor-series approximation (by compile switch).

- **Anchors**: `src/util.c:4433-4524` (time and names), `:4101-4151` (keymap directions), `:4154-4231` (repeat), `:4729-4737` (scaling), `:4839-4873` (timers), `:4526-4625` (gamma)
