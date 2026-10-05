# experience-system Specification

## Purpose

Experience and leveling: `src/xtra2.c` carries the check_experience level-up and level-down
loop, check_experience_obj object growth (TR4_LEVELS), the gain_exp/lose_exp distribution
with the black-breath epitaph (the PR1_CORRUPT corruption roll), and experience drain,
vacuum and recovery. The level reward table gain_level_reward is specified in
specs/chaos-patron/spec.md; the level-up power grants apply_level_abilities and the skill
points are specified in specs/skills/spec.md.

## Requirements

### Requirement: Level Up And Down Loop

`check_experience` SHALL:

- clamp exp and max_exp into [0, PY_MAX_EXP], with max_exp raised whenever it falls
 behind;
- run the level-down loop by exp < player_exp[lev-2] x expfact/100, one level per
 iteration (floored at 1);
- run the level-up loop by exp >= player_exp[lev-1] x expfact/100 while lev has not
 reached PY_MAX_LEVEL and max_plev, one level per iteration;
- per level: lite_spot; when max_plv rises, a PRACE with PR1_CORRUPT records
 level_corruption with one-third chance; a sound plays plus "Welcome to level %d."; when
 skill_last_level falls behind, the Lua exec_module_info("skill_per_level") grants skill
 points with an announcement plus PR_STUDY; apply_level_abilities runs; auto-notes record
 "Reached level %d" (marked 'L'); five update groups and three redraw groups are sent;
 with level_corruption, "You feel different..." plus corrupt_corrupted run;
- **Dead code:** the `level_reward` local is never set true anywhere in the function, so
 its `gain_level_reward(0)` branch is unreachable as shipped;
- finish with HOOK_PLAYER_LEVEL(gained).

- **Anchors**: `src/xtra2.c:3512-3648`

### Requirement: Experience Distribution

`gain_exp` SHALL:

- first count the TR4_ART_EXP equipment into num (starting from 1);
- each such item vacuums 2 x amount/(num x 3) experience (clamped to PY_MAX_EXP);
- with PRACE PR1_CORRUPT and max_exp>0, corrupt when randint(max_exp)<amount or
 randint(12000000)<amount (the comment notes 12 million as twice Morgoth's original
 experience);
- the body gains amount/num and HOOK_PLAYER_EXP fires;
- when exp trails max_exp (experience drained), max_exp gains amount/5 (the two-tenths
 vacuum recovery);
- finish with check_experience.

`lose_exp` SHALL subtract no more than the current value, fire HOOK_PLAYER_EXP, and run
check_experience.

`check_experience_obj` SHALL clamp the object exp into [0, PY_MAX_EXP] and then level up
while exp >= player_exp[elevel-1] x 5/2 - announcing "%s gains a level!" plus
object_gain_level.

- **Anchors**: `src/xtra2.c:3652-3681` (object), `:3687-3748` (gain), `:3754-3767`
 (lose)
