# automatizer Specification

## Purpose

The automatizer: `lib/core/auto.lua` carries the rule-set engine — the object action
functions (pick up / destroy / inscribe), object status evaluation (`object_status`),
rule tree compilation (`gen_rule_fct`), the rule-set pipeline
(add/apply/clean/regen/save), the graphical interface helpers (`auto_aux` navigation
and the type catalogue), and quick rule addition (`easy_add_rule`). The XML read/write
functions live in the `xml` module (see specs/lua-core/spec.md); the interactive C-side
UI entry `do_cmd_automatizer` (`src/squeltch.c`, reached from the options screen) is
out of scope here. Language
baseline: the Lua 4 dialect — `%var` upvalue capture, `arg` tables,
`getn`/`tinsert`/`tremove`/`call`, and the `TRUE`/`FALSE` macros.

## Requirements

### Requirement: Object Action Functions

The action functions SHALL behave as follows:

- `auto_nothing` does nothing.
- `auto_inscribe` only inscribes when the object's current inscription `obj.note` is 0 — it stores the inscription with `quark_add` and prints `"<Auto-Inscribe {...}>"`.
- `auto_pickup` applies only to floor items (`item < 0`) and only when `inven_carry_okay` allows — it calls `object_pickup` and prints `"<Auto-pickup>"`.
- `auto_destroy` SHALL carry four protections: unaware items are not destroyed, inscribed items are not destroyed, artifacts are not destroyed, and items with `TR4_CURSE_NO_DROP` and `IDENT_CURSED` are not destroyed; the destruction itself runs through the pack and floor paths (`increase`/`describe`/`optimize` triples) and prints `"<Auto-destroy>"`.

#### Scenario: Protected item

- **WHEN** the destroy action runs on an artifact
- **THEN** the item is left untouched and no `"<Auto-destroy>"` line is printed

- **Anchors**: `lib/core/auto.lua:10-60`

### Requirement: Object Status Evaluation

`object_status` SHALL evaluate as follows:

- For an unidentified object it looks the `obj.sense` up in the eight-entry wording table (`bad`/`very bad`/`average`/`good`/`very good`/`special`/`terrible`; a sense without a table entry returns the empty string).
- For an identified object it dispatches on `wield_slot_ideal`: an artifact reads `"special"` when not cursed and otherwise `"terrible"`; an ego item (`name2`/`name2b > 0`) reads `"very good"`/`"very bad"`; the four weapon slots judge by `to_h+to_d` in three grades; the armor stretch judges by `to_a` in three grades; rings judge by `to_d+to_h`/`to_a`/`pval` (any negative is `bad`, otherwise `average`); amulets judge by `pval`; chests judge by `pval` (0 -> `empty`, negative -> `disarmed`, positive -> `average`); everything else is `average`.

- **Dead code:** an embedded `if nil then` test block (the `select_sense`/`value_check_aux` path) is currently switched off, with the original text preserved in the source.

- **Anchors**: `lib/core/auto.lua:63-158`

### Requirement: Rule Tree Compilation

`gen_rule_fct` SHALL compile recursively: the combinator nodes `and` (true only when
all children are true) and `or` (true as soon as one child is true) compile each
subtree into a closure, skipping `comment` children; a `not` with no child compiles to
an always-true placeholder that the negation turns into a node that never matches. The fifteen condition kinds SHALL be:

- `name` (full lowercase equality against `object_desc`),
- `contain` (`strfind` substring),
- `symbol` (single `d_char` character),
- `inscribed` (`quark_str` substring),
- `discount` and `sval` (range tests after awareness; `min`/`max` may be literals or global names),
- `tval` (literal or global name),
- `status` (compared against `object_status`),
- `state` (identified or not),
- `race`/`subrace`/`class` (compared against the `get_*_name` results),
- `level` (a `player.lev` range),
- `skill` (resolved by `find_skill_i`, then a range),
- `ability` (`has_ability`).

The range rules `sval`, `level`, and `skill` raise an `assert` error when `min`/`max`
are missing. `auto_inscribe_maker` SHALL forward the original `arg` table through
`call` with the inscription argument appended.

- **Anchors**: `lib/core/auto.lua:161-260`

### Requirement: Rule-Set Pipeline

The pipeline SHALL work as follows:

- `gen_full_rule` — `args.module` defaults to `"ToME"`; when the module is neither `all` nor the current `game_module`, an empty function is returned; the action function is picked by `args.type` (`destroy`/`pickup`/`inscribe`); when `t[1]` exists the condition tree is compiled, otherwise the condition never matches; the composed closure returns `TRUE` only when the condition passes and the action returned `TRUE`.
- `add_ruleset` SHALL parse through `xml:collect` and build one `{table, fct}` pair per `<rule>` into `__rules`.
- `apply_rules` SHALL call each rule closure in order (`call` with `arg`) and return `TRUE` at the first hit.
- `clean_ruleset` resets the set to zero.
- `auto_aux:save_ruleset` SHALL temporarily switch `xml.write` to the file channel, emit the `"clean_ruleset()"` plus `"add_ruleset\n[[\n"` prologue, print each rule with `print_xml`, close with `"]]\n"`, and then restore the screen writer.
- `regen_ruleset` SHALL recompile every `fct` in full from the current `table`.

#### Scenario: First matching rule wins

- **WHEN** `apply_rules` walks the rule set and a rule's condition passes while its action returns `TRUE`
- **THEN** the walk stops and returns `TRUE` at once

- **Anchors**: `lib/core/auto.lua:263-335` (pipeline), `:474-485` (save), `:771-776` (regen)

### Requirement: Graphical Interface Helpers

The navigation functions SHALL behave as follows:

- `auto_aux` keeps a stack (pairs of `idx`/`rule` pushed together) supporting `go_right` (into the first child), `go_left` (pop), `go_down`/`go_up` (sibling moves, requiring stack depth > 1).
- `scroll_*` adjusts `xml.write_off_x`/`xml.write_off_y` directly.
- `adjust_current` resets the offsets and attaches `__rules[sel].table`.
- `move_up`/`move_down` swap with the neighbor and return the new index.
- `new_rule` SHALL — when `nam` is a table, attach it directly with an empty `fct`; when `typ` is `inscribe` and the argument is empty, ask for the inscription with `input_box` (limit 79); build the `label = "rule"` table with `module = game_module`, `tinsert` it, and bump the counter.
- `rename_rule` changes `args.name`.
- `del_self` SHALL — at the rule level remove the whole entry with `tremove`, decrement the counter, and return `sel - 1`; at the condition level `go_left` first, then `tremove` by `idx`, and return the original `sel`.

The `types_desc` catalogue SHALL list nineteen kinds (`and`/`or`/`not`/`comment`/
`name`/`contain`/`inscribed`/`discount`/`symbol`/`status`/`state`/`tval`/`sval`/
`race`/`subrace`/`class`/`level`/`skill`/`ability`), each a triple (a description or
description table, sample XML, and an interactive `input_box`/`msg_box` constructor;
`status` and `state` use single-key mapping tables). `display_desc` shows the
description for the type. `add_child` SHALL refuse when a `rule`/`not` already has a
child, refuse for anything but the four container kinds, then take the node from the
constructor and `tinsert` it.

- **Anchors**: `lib/core/auto.lua:339-498` (navigation, add and delete), `:500-740` (type catalogue), `:742-769` (display and add_child)

### Requirement: Quick Rule Addition

`easy_add_rule(typ, mode, do_status, obj)` SHALL build the detection string in one of
three modes (`tval` a single condition, `tsval` an `and` wrapping the `tval` plus a
single-point `sval`, `name` the lowercase full name); when `do_status` is set an
`and` wrapping the `status` condition is layered on top; the whole is wrapped in
`<rule module name type>`, inserted at the first position through
`auto_aux:new_rule`, followed by `regen_ruleset`, and two hint lines are printed
(press `=` then `T` to enter the screen and save).

- **Anchors**: `lib/core/auto.lua:780-803`
