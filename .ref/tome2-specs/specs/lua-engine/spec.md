# lua-engine Specification

## Purpose

The Lua script engine: initialization of the embedded Lua interpreter (the standard
libraries, the bit-operation library, the nine tolua binding packages), error message
forwarding, script file loading (with a compiled-cache fallback), string evaluation,
calling Lua global functions from C with a format string (the type protocol for
arguments and return values), and reading global variables from C. The C API surface
exposed by the tolua binding packages (player/player_c/object/monster/spells/quest/
dungeon/util/z_pack) is registered in specs/lua-binding/spec.md; the Lua-side runtime
over that API in specs/lua-core/spec.md.

## Requirements

### Requirement: Interpreter Initialization

The interpreter SHALL be initialized exactly once globally: create the Lua state, load
the four standard libraries (base, string, IO, debug), register `_ALERT` as the error
forwarder (in the message every `#` is replaced by `$`, the text is split on newlines,
and each piece is printed to the message window with a `"LUA:"` prefix), register the
eight bit-library functions (`bnot`, `imod`, `band`, `bor`, `bxor`, `lshift`, `rshift`,
`arshift`), open the nine tolua binding packages (player, player_c, util, z_pack,
object, monster, spells, quest, dungeon), and register the special wrapper for socket
reads.

#### Scenario: Single initialization

- **WHEN** `init_lua` is called a second time
- **THEN** it returns immediately and the interpreter state is not rebuilt

- **Anchors**: `src/script.c:277-312`, error forwarding `src/script.c:41-68`, bit library `src/script.c:219-229`

### Requirement: Boot Loading And Three-Phase Finalization

Script startup SHALL first execute `lib/core/init.lua`, then commit the Lua-side
registrations into the engine in three phases: build the spell school table sized by
`__schools_num` and run `finish_school` on each; build the spell table sized by
`__tmp_spells_num` and run `finish_spell` on each; build the corruption table sized by
`__corruptions_max`.

#### Scenario: School finalization

- **WHEN** the boot load has finished init.lua
- **THEN** the school table is built with the count declared on the Lua side and every school receives one `finish_school` callback

- **Anchors**: `src/script.c:314-340`

### Requirement: Script File Loading

Script loading SHALL support two directory forms: script-directory (`lib/scpt`)
addressing and arbitrary-directory addressing. When the `.lua` source does not exist,
the loader falls back to the compiled cache with the same base name and the `.luo`
suffix. When neither exists, a missing-file notice is printed as needed (the
arbitrary-directory form can be told to stay silent). After loading, the Lua stack
depth is restored.

#### Scenario: Compiled cache fallback

- **WHEN** the requested `.lua` file does not exist but the same-named `.luo` does
- **THEN** the compiled cache version is loaded

- **Anchors**: `src/script.c:342-378` (script directory), `src/script.c:380-417` (arbitrary directory)

### Requirement: String Evaluation

String evaluation SHALL execute a piece of Lua code and take the top-of-stack number
(a failed evaluation yields 0) or the top-of-stack string (a failed evaluation yields
the empty string). The stack depth is restored before and after the call.

#### Scenario: Failure fallback

- **WHEN** the evaluated code contains an error
- **THEN** numeric evaluation returns 0 and string evaluation returns the empty string

- **Anchors**: `src/script.c:419-450`

### Requirement: Function Call Protocol

C calls into Lua global functions SHALL be described by a format string covering the
arguments and the return values. Argument types are `d`/`l` integers, `s` strings,
`O` objects, `M` monsters, and `n` no value (parentheses and commas act as
separators). Return types `d`/`l`/`s`/`O`/`M` are written into the out-parameters in
order; on a type mismatch integers fall back to 0, while strings and user data fall
back to NULL, and an unknown return type raises an error. A failed call prints
an error and returns false. The stack depth is restored before and after the call.

#### Scenario: Type mismatch fallback

- **WHEN** the value returned by the Lua function does not match the declared return type
- **THEN** the corresponding out-parameter falls back to the default for its type and the call still counts as successful overall

- **Anchors**: `src/script.c:465-579`

### Requirement: Global Variable Read

C SHALL be able to read Lua global variables by name: the type protocol matches the
function return values (`d`/`l`/`s`/`O`/`M`), a type mismatch falls back to 0 for
numbers and NULL for strings and user data, and an unknown type raises an error. The
stack depth is restored after
the read.

#### Scenario: Numeric read

- **WHEN** a numeric global variable is read
- **THEN** its value is written to the integer out-parameter; a non-number yields 0

- **Anchors**: `src/script.c:581-641`
