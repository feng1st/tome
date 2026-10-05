# rng Specification

## Purpose

Random number generation: a simple linear congruential generator (a single value, fast,
used in early startup) and a 63-degree state table generator (the real game), both consumed
as 28-bit values and mapped onto ranges either by modulo (fast, slightly biased for large
ranges) or by partitioned division (unbiased); the normal distribution goes through a
256-entry precomputed table with binary search; plus the NdM dice and maximum-roll helpers
and the convenience macros for ranges/spreads/percentage checks. Seed management and the in-savefile state dump belong to
other capabilities: seeding runs in `src/dungeon.c` (see specs/game-loop/spec.md), the
savefile state is in `src/loadsave.c` (see specs/save-load/spec.md).

## Requirements

### Requirement: Dual Generators

The RNG SHALL provide two generators switched by a flag: the simple generator is a linear
congruential recurrence (a single state value; assigning the seed switches the seed); the
complex generator is a 63-degree additive table (the seed is propagated into the table by
the linear congruential step and churned by a warm-up loop of ten cycles per degree). Both
feed their output through the same 28-bit transform (shifted right by four).

#### Scenario: Table initialization

- **WHEN** the complex generator is initialized with a new seed
- **THEN** the state table fills by seed propagation, the warm-up cycles run, and
 subsequent draws take the table path

- **Anchors**: `src/z-rand.c:43` (the recurrence), `src/z-rand.c:74-97` (initialization and warm-up)

### Requirement: Range Mapping

Range draws SHALL come in two shapes: the modulo shape (the 28-bit value taken modulo the
range; large ranges bias toward low values, with a comment convention that ranges stay
below 500000) and the partition shape (the 28-bit value divided by the partition width,
redrawn when it lands in the incomplete partition — unbiased); a range of 1 or less always
yields 0. The integer convenience macros SHALL include: a draw from 0 to M-1 (the current
build takes the partition shape), the closed A-to-B range, the A-plus-minus-D spread, a
draw from 1 to M (yielding 1 when M is 1 or less), and the percentage check.

#### Scenario: Partition redraw

- **WHEN** the partition shape draws a partition number outside the range
- **THEN** the draw repeats until the landing point is valid

- **Anchors**: `src/z-rand.c:106-143` (modulo), `src/z-rand.c:157-212` (partition), macros `src/z-rand.h:30-62`

### Requirement: Normal Distribution

Normal draws SHALL use a 256-entry precomputed table (standard deviation 64, with the last
entries faked so the value range stays within four standard deviations) plus binary search:
a 32768 probability value is drawn to locate the offset, which is then scaled by the
requested standard deviation, with one half negative and one half positive; a standard
deviation below 1 returns the mean.

#### Scenario: Degenerate deviation

- **WHEN** a normal value is requested with a standard deviation below 1
- **THEN** the mean is returned directly

- **Anchors**: `src/z-rand.c:290-330`, precomputed table `src/z-rand.c:230-267`

### Requirement: Dice Draws

An NdM dice roll SHALL sum N individual dice (each 1 to M); a maximum roll SHALL be the
product of N and M.

#### Scenario: Maximum roll

- **WHEN** a scenario such as forced full HP needs the maximum of a hit die
- **THEN** the count is multiplied by the sides with no randomness involved

- **Anchors**: `src/z-rand.c:337-343` (damroll), `src/z-rand.c:349-352` (maxroll)
