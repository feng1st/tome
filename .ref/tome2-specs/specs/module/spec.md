# module Specification

## Purpose

The module system: the T-Engine runs several game modules on one engine (ToME is the
official module). This capability covers module discovery and selection (command-line
forcing, automatic selection of a single module, interactive selection of several),
activation after selection (the Lua-side initialization and the engine parameter read
back), and the redirection of the data directories into the module subdirectories. The
module registry and its metadata are defined on the Lua side
(`lib/mods/mods_aux.lua`, `lib/mods/modules.lua`, `lib/module.lua`) — covered in
specs/modules/spec.md.

## Requirements

### Requirement: Directory Redirection

Module activation SHALL redirect seventeen engine data directories into the module's
subdirectories (`lib/mods/<module>/<directory>`); the user-writable directories
(`user`, `note`, `cmov`, plus `apex`, `save`, and `data` depending on build options)
are instead redirected to the user's private path, and the module save directory
additionally turns off the setuid switch in the savefile code.

- **Quirk:** there is no guard for unknown directory names — an unrecognized name leaves the local target pointer NULL, and the trailing path rewrite then dereferences it (a latent crash); the shipped callers only ever pass known names, so the case is never exercised.

#### Scenario: Save directory

- **WHEN** a module is activated and the save directory is redirected
- **THEN** the save directory points into the module subdirectory under the user's private path, and the savefile code turns the setuid switch off

- **Anchors**: `src/modules.c:34-96`

### Requirement: Module Discovery And Loading

Module selection SHALL first load the module registry Lua (`mods_aux.lua` and
`modules.lua`, preferring the core directory and falling back to the modules
directory), then obtain the module count from Lua. When the command line forces a
module, it is looked up by name and selected automatically. With exactly one module,
that module is selected automatically, with no interaction.

#### Scenario: Single module auto-selected

- **WHEN** the module count is 1
- **THEN** that module is selected and activated directly, without entering the selection screen

- **Anchors**: `src/modules.c:174-212`

### Requirement: Interactive Selection

With several modules the engine SHALL present a selection screen: the module names are
laid out in a grid (the selected entry is marked with brackets and its description is
shown), the direction keys 8/2/4/6 move the cursor (scrolling wraps around), Return
selects, Esc exits the program. Enter converts the selection through the letter index
and the result is bounds-checked.

#### Scenario: Wrap-around scroll

- **WHEN** the cursor sits on the last module and the right-move key is pressed
- **THEN** the cursor returns to the first module

- **Anchors**: `src/modules.c:98-137` (list rendering), `src/modules.c:219-285` (key handling)

### Requirement: Module Activation

The selected module SHALL be reported to the Lua side (`assign_current_module`), and
its parameters are read back into engine globals: the level cap, the death dungeon
number, the three random artifact generation chances, and the version triple. A
non-ToME module SHALL have the main window title changed to the engine name plus the
module name. Finally the player name is reprocessed.

#### Scenario: Parameter read back

- **WHEN** a module is activated
- **THEN** the level cap, death dungeon, random artifact chances, and version numbers all sit in the engine globals

- **Anchors**: `src/modules.c:139-168`
