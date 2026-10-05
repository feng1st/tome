# skills Specification

## Purpose

The skill system core: `src/skills.c` carries the skill-tree allocation screen
(do_cmd_skill), the skill value folds (get_skill/get_skill_scale/find_skill), the skill
tree display and dump, the skill interactions (recalc_skills_theory exclusions and
increments, Thaumaturgy random spells), the melee-style switching, the activatable skill
menu (do_cmd_activate_skill), the gifted skills at start (do_get_new_skill), the ability
learning system and the level-based auto grants. The base-value sources (the
race/class/spec word-list skill_base/skill_mod) are specified in specs/data-loading/spec.md;
each MKEY case is specified with its own capability, for example specs/class-powers/spec.md.

## Requirements

### Requirement: Skill Value Folding

The folding helpers SHALL: `increase_skill`/`decrease_skill` step by mod (refused with no
points, zero mod, or SKILL_MAX already reached; the cap is bounded by the player level
plus the Lua max_skill_overage, with a msg_box notice on exceeding); `find_skill`
(case-sensitive) and `find_skill_i` (insensitive) look up by name; `get_skill` returns
value/SKILL_STEP; `get_skill_scale` returns scale x value/SKILL_MAX. `compute_skills`
SHALL fold the five layers in order - common (gen_skill_*), race, subrace, class,
speciality - each skill_base/skill_mod through modify_aux by the MEGO modifiers into
v/m. `init_skill` stores the value and sets hidden by SKF1_HIDDEN.

- **Anchors**: `src/skills.c:20-74` (increment), `:81-141` (lookup and fold),
 `:1212-1280` (compute/init)

### Requirement: Skill Tree And Allocation Screen

Tree building SHALL: `is_known` - known when it has a value, or a mod, or wizard; else
recursively any known child; `init_table_aux` collects father-matching, non-hidden, known
entries in order (lev advancing layer by layer, dev expanding or full expanding
everything); `has_child` decides leaves.

The display helpers SHALL: `print_skills` with its coloring rules (zero value zero mod
light dark, zero value with mod orange, maxed light blue, hidden light red, selected light
green plus brackets; leaf ".", expanded "-", collapsed "+"; the value column in xx.yyy
[mod] format); `dump_skills` the same structure into the dump file (the pending entries
are redeemed).

`do_cmd_skill` SHALL: run recalc_skills(TRUE) before entering the screen; keep five
arrays for value/mod/rate plus the invest/bonus work areas; inside the loop run
recalc_skills_theory live before printing; Enter expands or collapses, np pages, arrows
move, right buys and left sells (SKILL_MISC always refused), wizard adds/subtracts bonus
directly with the signs, `?` opens ingame_help; on exit with points spent, flush then
confirm "Save and use these skill values?", with refusal restoring everything; finish with
recalc_skills(FALSE).

- **Anchors**: `src/skills.c:147-220` (tree building), `:226-268` (dump), `:274-336`
 (print), `:434-598` (screen)

### Requirement: Skill Interactions And Random Spells

`recalc_skills_theory` SHALL: first store value = base_val + base_mod x invest + bonus
(capped at SKILL_MAX), then sweep the action table both ways - SKILL_EXCLUSIVE with the
other side invested zeroes it and refunds its points; non-zero percentage entries move
the other side's value by invest x mod x action/100 (capped).

`recalc_skills` SHALL: in the init state record the Thaumaturgy baseline; otherwise add
random spells point by point from the get_skill_scale(THAUMATURGY,100) delta with
generate_spell((level+1)/2) (stopping when spell_num is full) and announce the new count,
send HOOK_RECALC_SKILLS, and refresh
PU_BONUS/HP/MANA/SPELLS/POWERS/SANITY/BODY plus four redraw groups.

- **Anchors**: `src/skills.c:341-379` (recalc), `:384-429` (theory)

### Requirement: Melee Styles

The style table SHALL: melee_skills with three entries (MASTERY/HAND/BEAR) and the
matching name table. `get_melee_skill` looks up by the current melee_style;
`get_melee_skills` counts the available styles (value and not hidden) and fills the bool
table; `choose_melee` SHALL present a letter menu, reporting "You are already using..."
on a re-selection; on switching, every weapon slot is taken off (cursed refuses with a
message, a full pack force-drops), melee_style is set, energy_use=100, and
PU_BONUS/PU_HP/PR_MH refresh; `select_default_melee` picks the default from the first
available style.

- **Anchors**: `src/skills.c:605-742`

### Requirement: Activatable Skill Menu

`do_cmd_activate_skill_aux` SHALL gather: with multiple styles, a leading "Change melee
mode" (p value zero); s_info entries with a non-zero action_mkey and not hidden
(SKILL_LEARN is exempt from hidden) listed deduplicated by mkey with action_desc; acquired
ab_info entries listed deduplicated by mkey; letter selection, `@` selection by name, sign
paging, `*` toggling the hidden display.

`do_cmd_activate_skill` SHALL: repeat-stack and command_arg direct selection (a failed
mkey validity check reports "Uh?"); a zero value goes to choose_melee; invuln and
disrupt_shield break first; the switch dispatches by MKEY_* over 21 cases -
ANTIMAGIC/MINDCRAFT/ALCHEMY/MIMIC/POWER_MAGE/RUNE/FORGING/INCARNATION/TELEKINESIS/BLADE/
SUMMON/NECRO/SYMBIOTIC/TRAP/STEAL/DODGE/SCHOOL/COPY/BOULDER/COMPANION (requiring
SKILL_LORE >= 12)/PIERCING; the default hands over to the HOOK_MKEY hook.

- **Anchors**: `src/skills.c:770-922` (menu), `:925-1053` (dispatch)

### Requirement: Gifted Skills

`do_get_new_skill` SHALL: build the SKF1_RANDOM_GAIN skill pool plus the -1 sentinel and
draw four at random (each passing its random_gain_chance roll, no repeats); grant a value
by the current mod - mod<300 grants value 1000 and mod 300 minus current, mod<500 grants
value=mod and mod 100 (capped 500), otherwise value=mod x 3 and mod 0, with no mod
granting 1000/300; value capped at SKILL_MAX; ask_menu picks one of four; on an exclusion
conflict (SKILL_EXCLUSIVE with the other side valued) flush then confirm; success
announces "You can now learn..." or "Your knowledge ... increases." by whether mod
existed; recalc_skills runs before and after. `validate_autoskiller`/`autoskiller_level`
are unfinished placeholders (**Dead code:** the "I dont work, fix me" comment and an
`#if 0` body).

- **Anchors**: `src/skills.c:1085-1111` (validation placeholder), `:1113-1207` (`#if 0`),
 `:1282-1430` (gift)

### Requirement: Forbidding Predicates

`forbid_gloves` SHALL be true while any of
Sorcery/Mana/Fire/Air/Water/Earth/Thaumaturgy has a value (non-FA gloves forbidden);
`forbid_non_blessed` SHALL be true only under GOD_ERU (edged weapons forbidden).

- **Anchors**: `src/skills.c:1057-1074`

### Requirement: Ability System

The ability helpers SHALL: `find_ability` look up by name; `has_ability` check acquired;
`can_learn_ability` check - not yet acquired, enough points for the cost, all ten skill
levels and prerequisite abilities met with no forbidding ability, the six stats'
stat_ind meeting the requirements (required value minus three), and the HOOK_LEARN_ABILITY
hook not vetoing; `gain_ability` double-confirm "Learn this ability(this is permanent)?"
then set acquired and subtract points; `add_sorted_ability` insert by name order;
`print_abilities`/`do_cmd_ability` list everything (acquired light blue Known, learnable
white with cost, unlearnable dark), arrows move and right buys, wizard adds/subtracts
acquired with the signs, `?` opens ingame_help, exit refreshes the full PU/PR groups.
`apply_level_abilities` SHALL auto-grant at level-up by matching the level field of the
abilities[10] tables of class/speciality/race/subrace, with an announcement (the
level-one grant is silent).

- **Anchors**: `src/skills.c:1441-1540` (learning), `:1569-1655` (display),
 `:1660-1759` (screen), `:1764-1795` (auto grant)
