# flavor-tables Specification

## Purpose

In-game random text tables and readable texts: `lib/file/` holds 58 files. Two
formats — the get_rnd_line count-style tables (first line the entry count, second
line the `BUFFER LINE ... DO NOT REMOVE` sentinel, then one entry per line; the
format template is sample.txt and the directory description is readme!, the two
files that remain skipped) and the block / time-segment special cases (monspeak's
N: name blocks, timefun/timenorm's S:/E:/D: segments). The text bodies themselves
are not transcribed here; this spec registers each table's consumer call sites and
purpose. dead.txt (the RIP gravestone) and news.txt / news2.txt (the startup
splash) are full-frame ASCII art with color codes.

## Requirements

### Requirement: get_rnd_line Count-Style Tables

The mechanism tables SHALL be registered per consumer — dam_none / dam_med /
dam_lots / dam_huge / dam_xxx (`src/cmd1.c:2116-2146`: the five player-attack
feedback grades, %s filled with the monster name); sfail (`src/cmd7.c:4809`,
`src/cmd7.c:6032`: spell-failure material word combinations); death
(`src/spells1.c:1396`,
`src/spells1.c:1565`, `src/dungeon.c:2082`,
`src/cmd6.c:1220`, `src/cmd6.c:3590`: dying words — take_hit, take_sanity_hit, the
world-loop death branch, and the death-line branches of the fortune-cookie and
rumor-scroll switches all draw from this table); mondeath
(`src/xtra2.c:4587`: unique death lines, reached through mon_take_hit); monfear,
bravado, and speakpet (`src/melee2.c:6670-6674`: monster fear talk / monster
bravado / pet chatter, a three-way pick); monspeak (`src/melee2.c:6665` via
get_xtra_line: unique-only speech blocks, block format `N:<index>:<name>` plus a
count line plus speech lines, format self-documented in the file header); rumors
(`src/bldg.c:773`, `src/store.c:141`: building and in-store rumors, plus the
default branch of the same two `src/cmd6.c:1226` / `src/cmd6.c:3597` switches);
chainswd (`src/cmd6.c:1206`, `src/cmd6.c:3574`: chainsword-noise lines that no
chainsword code actually reads — the only consumers are the fortune-cookie and
rumor-scroll switches, while the Stormbringer effects are hardcoded in
`src/cmd1.c:2582-2588`); error (`src/cmd6.c:1212`,
`src/cmd6.c:3581`, `src/dungeon.c:4696`: the same easter-egg switches plus the
insanity-driven message shown on an invalid command); silly
(`src/monster2.c:1264`: silly monster rename table — personal names mixed with
monster names); elvish (`src/object1.c:356`: elvish syllable name building).
Retired leftovers SHALL be recorded as-is: smeagol.txt and smeagolr.txt currently
have no direct src consumer (Smeagol-specific lines, referenced by neither
get_rnd_line nor get_xtra_line; leftovers of an old feature).

- **Anchors**: `lib/file/dam_none.txt:1-6` (representative format, the other four
 share it), `lib/file/sfail.txt:1-6`, `lib/file/death.txt:1-6`,
 `lib/file/mondeath.txt:1-8`, `lib/file/monfear.txt`, `lib/file/bravado.txt`,
 `lib/file/speakpet.txt`, `lib/file/monspeak.txt:1-10` (format self-description),
 `lib/file/rumors.txt`, `lib/file/chainswd.txt`, `lib/file/error.txt`,
 `lib/file/silly.txt`, `lib/file/elvish.txt`, `lib/file/smeagol.txt`,
 `lib/file/smeagolr.txt`; consumers at `src/cmd1.c:2116-2146`,
 `src/cmd7.c:4809`, `src/spells1.c:1396`, `src/dungeon.c:2082`,
 `src/cmd6.c:1206-1226`, `src/xtra2.c:4587`, `src/melee2.c:6660-6674`,
 `src/bldg.c:773`, `src/store.c:141`, `src/monster2.c:1264`, `src/object1.c:356`

### Requirement: Full-Frame Art And Time Tables

The art files SHALL be: dead.txt (`src/files.c:5250`: the character-death RIP
gravestone, full-frame with `#x` color codes); news.txt and news2.txt
(`src/init2.c:6614-6627`: the startup splash, both files carrying the same picture
with the choice made by the parity of the current time (`time(NULL) % 2`);
main-mac/main-crb reference them too). The
time tables SHALL be timefun.txt and timenorm.txt (`src/cmd4.c:4676-4680`: the fun set on a one-in-ten roll or
while hallucinating, the normal set otherwise, block format `S:<start> E:<end>
D:<text>`, the in-game time hints).

- **Anchors**: `lib/file/dead.txt:1-6`, `lib/file/news.txt:1-6`,
 `lib/file/news2.txt:1-6`, `lib/file/timefun.txt:1-10`,
 `lib/file/timenorm.txt:1-8`; consumers `src/files.c:5250`,
 `src/init2.c:6614-6627`, `src/cmd4.c:4676-4680`

### Requirement: randart Name Tables

rart_f.txt and rart_s.txt SHALL be the randart name pair tables (one full and short
name per line, in one-to-one correspondence, e.g. "The Bag of Tricks"/"a Bag");
`MAX_RANDARTS` is 84 (`src/defines.h:164`) and birth.c:655-663 loads
random_artifacts[i].name_full/name_short line by line at character birth (the files
hold 86 and 87 lines respectively — only the first `MAX_RANDARTS` lines are read),
with attr and activation rolled on the spot; the naming and registration machinery
is documented in specs/randart-generation/spec.md.

- **Anchors**: `lib/file/rart_f.txt:1-6`, `lib/file/rart_s.txt:1-6`; load point
 `src/birth.c:655-663`

### Requirement: Readable Book Texts (book-*.txt)

The thirty book-*.txt files SHALL be fetched via the `book-%d.txt` pattern at
`src/cmd6.c:3638`, `src/cmd6.c:3662` by book sval inside `do_cmd_read_scroll` —
the map-parchment branch parses a count line plus x/y/w/h quadruples from the
file, the normal branch displays the file itself through `show_file` (prose with
color codes) when the book is read; book-0 through book-20 correspond to the
spellbooks and tomes, book-101 through book-107 are the short texts of artifact and
quest books (book-101 being an elvish translation-table sample), book-200 through
book-203 are placeholder stubs of a few digit lines, book-4/book-8 are short
pieces, the rest are medium or long.

- **Anchors**: `lib/file/book-1.txt:1-8` (representative),
 `lib/file/book-101.txt:1-4`, `lib/file/book-200.txt`, the remaining book-*.txt
 files each at 1 to their last line; consumers `src/cmd6.c:3638`,
 `src/cmd6.c:3662`
