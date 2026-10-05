# hook Specification

## Purpose

The hook system (event dispatch): engine event points (monster death, entering a
dungeon, end of turn, and so on) are identified by hook numbers, and either side (a C
callback or a Lua global function) can attach callbacks to a hook. When an event
triggers, the callback chain runs from its head (new callbacks enter at the head, so a
later attachment runs earlier) and any callback returning true
terminates the chain (the event is "handled"). Callback arguments are described by a
format string (`d`/`l` integers, `s` strings, `O` objects, `M` monsters between the
`(...)` placeholders); Lua-side arguments are pushed through tolua, and extra return
values are written back per the return format string. This file also literally
contains all quest plot implementations (`q_*.c`, see specs/quest/spec.md).

## Requirements

### Requirement: Hook Attachment And Deduplication

Callbacks SHALL be attached with deduplication by name: when a callback of the same
name already exists on that hook, the attach operation returns the existing node
instead of adding a new one; new nodes are inserted at the head of the callback chain.
Lua script hooks are registered under their global function name and their type is
marked as Lua.

#### Scenario: Same-name attachment

- **WHEN** a callback with the same name is attached to the same hook a second time
- **THEN** the chain holds only one copy and the second attach returns the existing node

- **Anchors**: `src/plots.c:81-116`

### Requirement: Hook Removal

Callbacks SHALL be removable either by function pointer or by name: the node is
unlinked from the chain and freed; a missing callback is a no-op.

#### Scenario: Removal by name

- **WHEN** an attached callback is removed by name
- **THEN** its node is unlinked from the chain and the order of the remaining callbacks is unchanged

- **Anchors**: `src/plots.c:119-187`

### Requirement: Chained Triggering

A hook trigger SHALL run each callback in chain order: C callbacks are called with
arguments taken from the parameter stack; Lua callbacks are called as global functions
with arguments pushed through tolua. Any callback returning true SHALL terminate the
chain immediately and the trigger returns true; if every callback returns false the
trigger returns false. When a Lua callback errors, the engine SHALL report the error
and break off the current hook chain (returning false). A trigger without extra
return values (`process_hooks`) uses only the chain's termination semantics.

#### Scenario: Chain termination

- **WHEN** the second callback in the chain returns true
- **THEN** the callbacks after it do not run and the trigger result is true

#### Scenario: Lua error breaks the chain

- **WHEN** a Lua callback call fails
- **THEN** an error notice is printed, the hook chain stops immediately, and the trigger returns false

- **Anchors**: `src/plots.c:232-393`, no-return-value entry point `src/plots.c:406-415`

### Requirement: Extra Return Value Write-Back

A trigger with a return format (`process_hooks_ret`) SHALL write each callback's extra
return values into the global return array per the format string after the chain
ends: `d`/`l` integers (non-numbers become 0), `s` strings (non-strings become
NULL), `O` object pointers and `M` monster pointers (a type mismatch becomes
NULL). The first return value keeps the chain-termination semantics.

#### Scenario: Type mismatch fallback

- **WHEN** a Lua callback's extra return value does not match the format string type
- **THEN** the corresponding return slot is filled per the empty-value rule of that format type

- **Anchors**: `src/plots.c:341-369`, write-back array `src/plots.c:230-231`

### Requirement: Chain Restart

Callback processing SHALL support a restart marker: when a callback sets the restart
marker, the walk over the current hook starts over from the head (rather than at the
next callback) and the marker is cleared right away — so a callback that modified the
chain can have the new chain rescanned.

#### Scenario: Restart

- **WHEN** a C callback sets the restart marker and returns false
- **THEN** the walk returns to the head of the callback chain and starts over

- **Anchors**: `src/plots.c:279-288` (C), `src/plots.c:377-381` (Lua)

### Requirement: Hook Inventory And Rebuild

The hook table SHALL support wiping it as a whole (`wipe_hooks`) and initialization
per quest (`init_hooks`: walks the quest table, and for each quest of C type calls its
init callback, which attaches the plot hooks). It also supports querying whether a
hook has any callback (`check_hook`) and printing the callback roster (`dump_hooks`,
optionally limited to one hook, with the C or Lua type noted).

#### Scenario: Quest initialization

- **WHEN** the hooks are initialized
- **THEN** the init callback of every C-type quest in the quest table is called, and it attaches the plot hooks

- **Anchors**: `src/plots.c:29-46`, `src/plots.c:48-78`
