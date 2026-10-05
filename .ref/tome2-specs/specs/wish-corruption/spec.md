# wish-corruption Specification

## Purpose

The wish and corruption bridge: `src/xtra2.c` carries `make_wish` (full-name
matching over the object and monster paths), `test_object_wish` (a brute-force walk
of `k_info` and both ego tables), `clean_wish_name`, plus the corruption C-side
bridge pieces (`gain`/`lose`/`lose_all`/`got`/`dump` forwarding into `lib/scpt`'s
`__corruptions` system) and the `corrupt_corrupted` draw. The corruption data and
the dependency tree live in `lib/scpt/`; see specs/corruption/spec.md.

## Requirements

### Requirement: Wishes

`clean_wish_name`, `test_object_wish` and `make_wish` SHALL implement wish
resolution:

- `clean_wish_name` lowercases, collapses consecutive spaces, and trims leading and
 trailing spaces.
- `test_object_wish` brute-force walks `k_info` (excluding NORM_ART/INSTA_ART/gold
 coins) - `object_prep` plus `apply_magic(dun_level)`, then forced aware/known,
 takes the lowercase description, and the entry qualifies when the wish text
 `strstr`-matches it, or when it is `TV_ROD_MAIN` containing "rod of";
- a qualifier then walks both ego tables (j outer, jb inner, both in reverse order,
 j and jb mutually exclusive when they share the same `before`, with six groups of
 tval/sval range checks), reforges, and returns true on a `stricmp` equality match,
 leaving the item in the forge;
- `make_wish` takes the wish with `get_string("Wish for what? ", 80)`; a wish
 containing "wish" is refused ("You can't wish for a wish!");
- on the object path a hit is delivered with `drop_near`;
- the monster path derives the stance from the prefix (enemy/neutral/friendly/pet/
 companion - companion falls back to pet when the companion slots are full),
 excludes SPECIAL_GENE/NEVER_GENE/UNIQUE, matches by name `strstr`, then walks
 `re_info` in reverse order (filtered by `mego_ok`, ordered by `before`
 names); on a `stricmp` hit `place_monster_one` places it within `scatter(5)` and
 "Your wish becomes truth!" plays.

- **Anchors**: `src/xtra2.c:7534-7566` (name cleaning),
 `src/xtra2.c:7408-7532` (object matching), `src/xtra2.c:7571-7686` (`make_wish`)

### Requirement: Corruption Bridge

The C-side pieces SHALL be pure forwards into Lua - `gain_random_corruption` calls
`gain_corruption()`, `lose_corruption` calls `lose_corruption()`,
`lose_all_corruptions` calls `lose_all_corruptions()`, and all three always return
FALSE; `got_corruptions` loops over `__corruptions_max` asking
`test_depend_corrupt` bit by bit; `dump_corruptions` SHALL write each corruption's
name and description (with the `#####c` color code when the color bit is set, via
the `conv_color` mapping). `corrupt_corrupted` SHALL lose one corruption on
`magik(45)` and gain one otherwise.

- **Anchors**: `src/xtra2.c:7236-7252` (the three forwards),
 `src/xtra2.c:7353-7367` (got), `src/xtra2.c:7372-7393` (dump),
 `src/xtra2.c:7693-7706` (the draw)
