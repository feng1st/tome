# spoilers Specification

## Purpose

Spoiler file generation: `src/wizard1.c` hosts the six text spoilers offered by the
`do_cmd_spoilers` debug menu — the brief object list, the full artifact analysis, the
brief monster list, the full monster analysis, the essences table, and the spell
table. The word-wrap buffer `spoil_out` backs all of the output, including the
help-markup output paths used by the essences table.

## Requirements

### Requirement: Generation Entry Point And Text Buffer

`do_cmd_spoilers` SHALL offer six choices and write each spoiler into the user
directory (obj-desc.spo, artifact.spo, mon-desc.spo, mon-info.spo, ess-info.spo,
spell.spo), reporting a failure message when the file cannot be created or closed.
`spoil_out` SHALL wrap the text word by word against a 75-column buffer, carrying a
word that crosses the boundary whole to the next line; the terminating call with a
NULL argument SHALL flush the buffer — a bare newline when the buffer is empty,
otherwise the buffered paragraph followed by a blank line — and reset the buffer.

- The menu loop runs with `character_icky` set and the screen saved and restored; an
 unrecognized key rings the bell.
- Each wrapper prints `Cannot create spoiler file.` when the open fails, `Cannot
 close spoiler file.` when the close fails, and `Successfully created a spoiler
 file.` on success.

#### Scenario: Unknown menu key

- **WHEN** the player presses a key other than 1-6 or ESCAPE in the spoiler menu
- **THEN** the bell rings, no file is written, and the menu redraws

- **Anchors**: `src/wizard1.c:2729-2821` (menu), `src/wizard1.c:50-113` (buffer),
 `src/wizard1.c:384-391` (open failure message)

### Requirement: Brief Object And Monster Lists

The brief object list SHALL group items by the tval groups of `group_item`
(twenty-odd named groups), bubble-sorting each group by cost ascending with level as
the tie-breaker, skipping kinds flagged `TR3_INSTA_ART` or `TR3_NORM_ART` and the
Ring of Powers (k_idx 785); each row SHALL come from `kind_info` on a fake object and
give the base description, damage or armor, weight, level, and cost. The brief
monster list SHALL list every named race with its name (uniques prefixed `[U]`,
others `The ...`), level, rarity, speed relative to 110, hit dice (`RF1_FORCE_MAXHP`
or a single-sided die yields the fixed value `hdice * hside`), armor class, and the
color and character of its display symbol.

#### Scenario: Equal-cost ordering within a group

- **WHEN** two kinds in the same tval group have the same cost
- **THEN** the one with the lower level is listed first

- **Anchors**: `src/wizard1.c:183-254` (tval groups), `src/wizard1.c:365-492`
 (object list), `src/wizard1.c:1343-1475` (monster list)

### Requirement: Full Artifact Analysis

The artifact spoiler SHALL fake one artifact per entry grouped by tval
(`make_fake_artifact` runs `apply_magic` at depth -1 with the good/strong flags all
set; `ART_POWER` — the One Ring — is never passed through `apply_magic`, only its
artifact pval is copied) and, after the full analysis, SHALL output the description,
the long object-property text (`object_out_desc`), and a miscellaneous line (level /
rarity / weight / cost). The analysis SHALL cover:

- the pval effects (all six stats folded into `All stats`);
- slays, brands, immunities, and resistances;
- sustains (all six folded into `All stats`);
- miscellaneous magic — auras, no teleport, antimagic, wraithform, feather fall, see
 invisibility, slow digestion, regeneration, extra shots, drain experience,
 aggravate monsters, blessed blade, a permanent light radius summed over
 `TR3_LITE1`/`TR4_LITE2`/`TR4_LITE3` with wording keyed off the fuel flag, and the
 three curse tiers deduplicated;
- the activation description (`item_activation`).

- **Quirk:** the light line is emitted for every artifact, even at radius 0, and
 the two wordings are attached backwards relative to `TR4_FUEL_LITE`
 ("fuelable lite") — a fuel-burning light prints "forever" while a non-fuel one
 prints "when fueled" (`src/wizard1.c:1030-1041`).

- **Anchors**: `src/wizard1.c:600-824` (flag description tables and structures),
 `src/wizard1.c:841-1114` (analysis), `src/wizard1.c:1191-1240` (faking),
 `src/wizard1.c:1246-1334` (output)

### Requirement: Full Monster Analysis

The full monster spoiler SHALL emit one prose paragraph per race — index, level,
rarity, speed, hit dice, armor class, experience, and the race description text —
assembled sentence by sentence:

- species characterization: sanity-blasting, natural, evil, good, undead, and one of
 dragon / demon / giant / troll / orc / Thunderlord (else "creature");
- movement habits: the three random-move grades, or "does not deign to chase
 intruders";
- fixed depth ("never found out of depth") and forced sleep ("always created
 sluggish");
- fire and electricity auras and bolt reflection;
- escorts and groups;
- innate attacks: shriek for help, rocket, and the four arrow/missile volleys;
- the twenty-two breaths, with "powerfully" appended for `RF2_POWERFUL`;
- the full spell list: balls, Hand of Doom, drain mana, mind blasts, cause-wound
 curses, bolts, scare/blind/confuse/slow/hold, self-buffs, teleport-to/away/level,
 and the summon families, one item per flag;
- casting frequency: 1 time in `200 / (freq_inate + freq_spell)`;
- the mobility block: open doors, bash down doors, pass/bore through walls, push past
 or destroy weaker monsters, pick up or destroy objects, illuminate the dungeon;
- invisibility, cold blood, the two telepathy-shielding grades (empty mind and weird
 mind), explosive breeding, and fast regeneration;
- the four susceptibilities, five immunities, six resistances, and the four
 cannot-be states;
- alertness wording graded on the `sleep` thresholds (eleven grades from "prefers to
 ignore" down to "is ever vigilant for"), with a noticing distance of `10 * aaf`
 feet;
- the drop wording: one / one or two / up to N carried items, with `RF1_DROP_GREAT`
 ("exceptional object"), `RF1_DROP_GOOD`, `RF1_DROP_USEFUL`,
 `RF1_ONLY_ITEM`/`RF1_ONLY_GOLD` phrasing, and the `RF1_DROP_CHOSEN` note ("in
 addition to chosen objects");
- the blow table, each blow with its method and effect text plus damage dice, and the
 `RF1_NEVER_BLOW` note.

- **Anchors**: `src/wizard1.c:1497-2358`

### Requirement: Essences And Spell Spoilers

The essences spoiler SHALL output in help-markup format (links, colors, anchors):
the long alchemy introduction (skill thresholds 5 / 10 / 15 / 25 / 50 each unlocking
a stage; experience shared with artifacts, which receive only 60% of their share;
the base recipe list), the six artifact power categories item by item (level /
experience / power / name / object description plus the essence cost of the matching
recipe), and the base object and ego recipes with their essence requirements. The
spell spoiler SHALL temporarily overwrite the skill value with `SKILL_MAX`
(`src/defines.h:4573`, 50000 — the header prints `Spell Spoiler (Skill Level 50)`),
list the bookless power tables of Mimicry, Mindcraft, Necromancy, and Symbiosis
(name / level / mana cost / failure rate / extra info / description), and then
restore the saved skill value.

- **Anchors**: `src/wizard1.c:2392-2453` (introduction), `src/wizard1.c:2458-2619`
 (essences), `src/wizard1.c:2625-2717` (spells)
