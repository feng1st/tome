# chaos-patron Specification

## Purpose

The Chaos patron and his rewards and punishments: `src/xtra2.c` carries
`get_chaos_patron` (the patron number) and `gain_level_reward` (the Chaos reward
table, twenty categories). The reward and patron-name data
(`chaos_patrons`/`chaos_rewards`/`chaos_stats` in `src/tables.c`) is registered in
specs/tables/spec.md. The corruption system itself lives in `lib/scpt/`; the
corruption side is in specs/corruption/spec.md and specs/wish-corruption/spec.md.

## Requirements

### Requirement: Patron Number And Level Reward

`get_chaos_patron` SHALL return `(age + sc) % MAX_PATRON`. `gain_level_reward(chosen_reward)`
SHALL roll the reward type, format the wrath reason, index the patron's reward
table, and execute the drawn `REW_*` case:

- `nasty_chance` starts at 6; it is 2 when the level equals 13, 3 when the level is
 a multiple of 13, and 12 when the level is a multiple of 14.
- When `randint(nasty_chance)` hits, `type = randint(20)` (nasty effects allowed),
 otherwise `type = randint(15) + 5` (nasty effects disallowed); `type` is clamped
 to 1-20 and then reduced by one for indexing.
- `wrath_reason` is formatted as "the Wrath of %s" with the patron name.
- `effect = chaos_rewards[patron][type]`.
- On a one-in-six roll with no `chosen_reward` given, the god grants a random
 corruption instead and returns.

The cases SHALL cover the `REW_*` categories:

- `REW_POLY_SLF`: `do_poly_self`.
- `REW_GAIN_EXP`: `exp/2 + 10`, capped at 100000.
- `REW_LOSE_EXP`: `exp/6`.
- `REW_GOOD_OBJ` / `REW_GREA_OBJ`: `acquirement` of one item (the latter great).
- `REW_CHAOS_WP`: draws from the per-level sword table (dagger up to
 BLADE_OF_CHAOS) and forges `to_h`/`to_d = 3 + randint(depth) % 10`, plus
 `random_resistance(4 + randint(34))`, plus `EGO_CHAOTIC`, plus `apply_magic`.
- `REW_GOOD_OBS` / `REW_GREA_OBS`: `acquirement` of `randint(2) + 1` items.
- `REW_TY_CURSE`: `activate_ty_curse`.
- `REW_SUMMON_M`: `randint(5) + 1` monsters.
- `REW_H_SUMMON`: `activate_hi_summon`.
- `REW_DO_HAVOC`: `call_chaos`.
- `REW_GAIN_ABL`: `do_inc_stat` (one third of the time uses the patron's stat; when
 the patron stat is negative, a random stat).
- `REW_LOSE_ABL`: the same structure, decreasing.
- `REW_RUIN_ABL`: all six stats through `dec_stat(10 + randint(15), TRUE)`.
- `REW_POLY_WND`: `do_poly_wounds`.
- `REW_AUGM_ABL`: all six stats increased.
- `REW_HURT_LOT`: a disintegration ball of `lev*4`, radius four, plus self-damage.
- `REW_HEAL_FUL`: full cleansing (`restore_level`, clearing six states, hp 5000,
 all six stats restored).
- `REW_CURSE_WP`: `curse_weapon`.
- `REW_CURSE_AR`: `curse_armor`.
- `REW_PISS_OFF`: a four-way choice (ty curse / hi summon / cursed item / all-stat
 decrease).
- `REW_WRATH`: `lev*4` self-damage plus an all-stat decrease plus `hi_summon` plus
 the ty curse plus a fifty-percent chance of dual cursed items.
- `REW_DESTRUCT` (non-quest level and inside a dungeon): `destroy_area(25)`.
- `REW_GENOCIDE` / `REW_MASS_GEN`.
- `REW_DISPEL_C`: `lev*4`.
- `REW_IGNORE`: nothing happens.
- `REW_SER_DEMO` / `REW_SER_MONS` / `REW_SER_UNDE`: friendly summons (on failure
 "Nobody ever turns up...").
- An undefined category plays the stammering-voice message.

- **Anchors**: `src/xtra2.c:6735-6738` (patron number), `src/xtra2.c:6741-7172`
 (reward table)
