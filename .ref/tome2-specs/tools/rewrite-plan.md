# Rewrite Plan — Chinese specs to unified English

Status per batch: pending / dispatched / verified. Completion gate lives in
tools/STYLE-GUIDE.md (lint green + anchor diff clean + tome2spec check PASS + this table all verified).

Pre-rewrite backup (fidelity diff source): /tmp/opencode/spec-backup/ — do not modify.
Verification after each wave: `python3 tools/spec-lint.py --diff-old <names>`; spot-read prose;
fix findings before marking the batch verified.

| batch | specs | status |
|---|---|---|
| B1-monster | monster monster-ai monster-death monster-ego monster-faction monster-generation monster-melee monster-memory | verified |
| B2-object | object object-core object-gen item-set item-usage ego-item artifact randart-generation randart-part alchemy | verified |
| B3-player | player player-derive player-display player-melee player-movement player-ranged player-states experience-system skill skills character-birth class-powers ability | verified |
| B4-divine-magic | racial-powers god god-quest chaos-patron corruption wish-corruption school-magic school-spells spell-casting spell-effects | verified |
| B5-world-format | dungeon dungeon-generation dungeon-level cave-lighting terrain trap vault wilderness wilderness-terrain content-maps map-format edit-format | verified |
| B6-quest-store | quest quest-ultra building building-action store store-owner store-runtime ghost death-score | verified |
| B7-lua | lua-binding lua-core lua-engine module modules hook automatizer scpt-misc | verified |
| B8-engine | game-loop program-entry boot-loading data-loading global-state core-data defines tables capacities util rng save-load | verified |
| B9-ui-misc | spoilers status-screens targeting-panels ui-frames interface-commands inventory-commands pref-file wizard-debug flavor-tables | verified |

Notes:
- Audit wave (post-rewrite, quota-reset resumption): each interrupted batch (B3/B6/B7/B8/B9)
  re-audited vs sources by a fresh agent; B6 done — building, store-runtime, building-action,
  quest-ultra, death-score fixed (rod-timeout tiebreak, has_won, restriction-tier Discrepancy,
  brand multipliers, loan clamp); store, store-owner, ghost clean.
  B7 done — teleport_player_to (invented teleport_to fixed), god/gods.lua invented path removed,
  new_player_spot 5000 attempts, repeat_check restored, call_lua/get_lua_var NULL fallbacks.
  B9 done — flavor-tables consumer corrections (chainswd fortune-cookie/rumor-scroll only,
  Stormbringer hardcoded cmd1.c:2582-2588, book-* split consumers cmd6.c:3638/3662,
  MAX_RANDARTS=84), inventory wand-vs-staff, ui-frames prt_sane 10x hitpoint_warn,
  pref-file weekday window 8:00-16:59, wizard-debug +?/H keys.
  B3 done — player-movement (deep-water bar small map, Yavanna 9000, aura formulas,
  pet-menu give-target Dead code, rest key `&`), player-derive (cnv_stat thresholds,
  tht=50xlvl/10), class-powers (Minor Displacement gate 25, Death-refill Discrepancy),
  player-melee (impact brand damage>50), experience-system (level_reward Dead code);
  character-birth/player/skills/ability/player-states/player-display/player-ranged clean.
  B8 done — game-loop (closing gate 3 warnings, gorged 100/step, piety-drain inversion,
  dungeon-damage group-index Quirk, cheat-death age Discrepancy, +18 scenarios),
  data-loading blow tables 25/35, save-load aux 16 u32b + quick-start 13 fields,
  defines (11 abilities/22 breaths/GF 111 slots/POWER_MAX_INIT 62/building 53),
  tables (option 105/activation 51/gf_names 92/powers 62), util timer 10-turn chain.
  quest done (independent) — plot count 25 impls/23 files (was 18), Farmer Maggot daylight
  6-18 (not night), six staircase directions corrected (FEAT_LESS is the up staircase),
  TR2_SENS_FIRE is sensitivity, wolves/dragons 50%/one-third asleep flags (not friendly),
  Glamdring rollback on o_pop, hobbit reward rod of recall, One Ring dialog special 23,
  max-hp (lives+1) applications; all race/store/kill-list numbers re-verified.
  B2 done — object-gen (+20 fixes: redraw p<60 Discrepancy, wand-charge absorb, no speed-ring
  cap, reorder_pack overflow Dead code, chest pval2 formula, drop_near scoring), item-usage
  (GILGALAD eight beams, ANGUIREL 222); object/object-core/alchemy/ego-item/artifact/
  randart-generation/item-set/randart-part verified clean.
  B4 done — school-spells (invented DEVICE_ETERNAL_LONGANCE removed, Music 13 tunes,
  SPELLBINDER 3100 energy, Device 12/4, WATER_BITE level(50), geomancy 16), spell-effects
  (smash 30 no-effect, describe 34 tiers), school-magic (21 schools, 28 books),
  chaos-patron (REW_HURT_LOT disintegration Discrepancy), god (ask-hook-still-runs,
  pgod<0 unreachable Discrepancy), corruption (14 corruptions, CORRUPT_VAMPIRE_VAMPIRE);
  spell-casting/racial-powers/god-quest/wish-corruption clean.
  B1 done — monster-ai (innate ban chance x2, no_mortal fate flag, SHATTER quake center,
  poison 1/10 counter, DRAIN_MANA heal formula), monster-melee (24 attack methods incl.
  2 XXX placeholders, no_mortal FATE_NO_DIE_MORTAL, explode hp+1), monster-memory (24,
  purpose pointers split), monster-ego (unknown operator falls back to MEGO_ADD, anchors
  re_info.txt 37/17), monster-faction (possession pval2), monster-generation (sleep formula,
  fate message), monster-death (Stormbringer STR+CON); monster OK.
  B5 done — dungeon-generation (small levels 1-3x1-2, wiz_lite condition, auto_scum ceilings
  9..5, streamer depth>33 Discrepancy, vault glyph set, +father-branch Discrepancy), trap
  (deity tiers un-swapped: anger -3000 / wrath -500xlev), cave-lighting (FF1_NO_VISION on raw
  feat), content-maps (haunted.map g = object + ego), wilderness (TOWN_NORMAL_FLOOR restored);
  terrain/map-format/edit-format/dungeon-level/vault/wilderness-terrain clean.

AUDIT WAVE COMPLETE — all 91 capabilities independently re-verified against sources.
WAVE 3 (fresh model, independent): B8 done — game-loop symbiote-rebellion precedence,
defines 3 rates + 2 bases, tables extract_energy+30 = 38, capacities V-key restored
(missed by every earlier pass); wave-2 fixes re-verified true.
B6+quest done — quest no-cleanup -> no-genocide (DF2_NO_GENO mistranslation from the
Chinese survived two waves), four quoted messages now verbatim source strings, dragon
ages baby/young/mature/ancient, thieves alarm single door; store-runtime haggle
convergence direction; store-owner header anchors off-by-one corrected (2 justified
MISSING deltas); wave-2 fixes re-verified true.
B4 done — MANWE_BLESS set_lite(0) Discrepancy, hates_* item lists corrected,
inven_damage wands, recharge fail_type wand pval, report_magics 20 states, earthquake
15%, spell_power refusal scope, zero-school any-n/a, stick dead-assert Quirk,
corruption magik percents + five-bracket prefix, PWR_VAMPIRISM precedence +
100-vs-150 Discrepancy, god-quest 12 fields; wave-2 fixes re-verified true.
B1 done — monster-melee/monster-ai hit-check bands un-inverted (k<5 = forced HIT,
5-9 = forced miss per return(k<5); wrong since the original Chinese analysis, survived
two waves), monster error-code names + F: six tables, normalization 0x0463
Discrepancy, monster-ego header anchors re-aligned, monster-generation spirits-not-
elves + paralysis-not-stun + p<60 Discrepancy, monster-memory %02ld + aquatic-only
filter; wave-2 fixes re-verified true.
B7 done — 58 set_* bindings (was "about forty"), Elder Ent six-adjusted/four-scaling,
hook chain runs from head, module unknown-dir NULL-pointer latent-crash Quirk,
lua_bind.c attribution + checked-in fact restored; wave-2 fixes re-verified true.
B9 done — interface-commands dead-uniques listing + INVEN_TOTAL scan, inventory wear
filter un-inverted (wrong since Chinese original), wizard-debug spoilers key is "
(was backtick), x-no-arg gain_exp(+1), / summon-by-type; wave-2 fixes re-verified true.
B5 done — content-maps glyph-field corrections (qrand t = trap, 2056 = CAVE flags,
s_doom 177 lava wall / L:85 deep lava, thrain mimic=61, maeglin 799 Master mindcrafter,
nirnaeth 538/496 Olog/Cave troll), trap AUTO_5 survival un-inverted + wand table is the
#if 0'd one, dungeon D: name offset, map-format M: object letter O, vault padding
direction + dead-path note; one cosmetic SUSPECT noted (town_num 0-999 vs code 1-999).
B2 done — rod sort direction un-inverted (Discrepancy vs comment), fego M4 removed,
egg try-1000 Quirk, trap-overlay loop bounded by max_rmp_idx(10) not max_t_idx(176)
Quirk, item-usage fire-burn inversion + breath temp-resist + healing ladder + egg
incubation + ACT~190, randart doubling gate !a_cursed, anchor re-points across
alchemy/e_info/set_info/ra_info; prior-wave fixes re-verified true.

WAVE 3 COMPLETE (fresh model, all 91 capabilities): corrections per batch logged above.
WAVE 4 (convergence check, all 91): B1 done — 2 errors in 8 files (critical-table lower
bounds 5->1, memory frequency "power of ten" -> round-up-to-multiple-of-ten).
B2 done — 12 errors in 10 files (invented names "mage robe"/"Stone of Knowledge" ->
Stone of Lore, corpse-fold target names, chest "(disarmed)" string, output counts
11/16, brand_bolts per-stack not per-bolt, compact_objects branch conflation).
B5 done — 13 errors in 12 files (w_info legend 7 labels corrected per wf_info,
qrand W/w = water / L/l = lava not dragons/liches, FINAL_ARTIFACT_203 is Nether Realm
not Angband, cave-lighting detected-not-disarmed traps, vault anchor re-points).
B4 done — 16 errors in 10 files (AIRWINGS wand/staff swap surviving three waves,
detect_doors inversion, invented BIAS_RIGGER removed, TIME tiers 5/4/1, PSI backlash
demons, do_move multipliers, nexus savable scope, ident_spell both-options gate,
PWR_DET_CURSE pack+equipment).
B3 done — 18 errors in 13 files (alchemy level/4+1 off-by-one, three wand/staff swap
sites + two random-books misreads in class-powers, race OR subrace class set,
point-buy 4/6 keys and deterministic merge, disembodied direction inversion,
TERM_L_DARK twice, phantom class-C record removed, O-line pval position).
B6+quest done — 3 errors in 9 files (One Ring dialog fires while UNTAKEN and takes the
quest itself, quest-ultra stair descends FROM 149, bounty roll uses standard deviation).
B9 done — 1 error in 9 files (timefun/timenorm selection is a one-in-ten roll or
hallucination, not per-option).
B7 done — 8 errors in 8 files (pkg alias notation c@lua inverted since the Chinese
original, 12 sites corrected; GF_INSTA_DEATH targets living monsters; automatizer
empty-not node inverted; module dirs are seventeen).
B8 done — 22 errors in 12 files (data-loading R:O field / SPELL= sign / class X line /
re_info E-vs-D + 1_IN_N on S / ab_info k+S forms; save-load field counts; defines 38
SUMMON_; tables window 32/12 + ma_blows 2d4 + cure 500-5000 + acquirement 30000 +
quest_init_tome per-plot counts; util message-ring tail inversion; global-state 17
word-list families; boot-loading st_idx set). Resumed manually after quota cut.
WAVE 4 COMPLETE: totals per batch B1=2 B2=12 B3=18 B4=16 B5=13 B6=3 B7=8 B8=22 B9=1
= 95 items; 53/91 files CLEAN. Severity mix shifted to counts/wording/anchors; the
remaining inversions were few (util ring tail, INSTA_DEATH living targets, automatizer
not-node, One Ring dialog timing, disembodied direction, pkg alias notation).
Final gates post-wave-4: lint 91/91 PASS; anchor diff 28 entries, all documented
corrections; CJK zero; tome2spec check PASS; corpus 16912 lines.

CONVERGENCE VERDICT: wave-4's 95 items are ~51% mechanism-level (inversions/swaps/
field misreads) — NOT converged to zero, but the cheap mechanical sweeps now close
the gaps a fifth agent wave would have covered:
- tools/num-anchor-check.py: 1925 (number, requirement) pairs; 523 region-level
  suspects; whole-tree filter leaves 1, adjudicated as a legitimate derived range
  (free_act 7000 / fly 15000 -> "7000-14999"). Zero invented constants remain.
- identifier sweep (2203 tokens): only EFF_DIR5 (intentional gap note).
- quote sweep: zero fabricated messages.
- acceptance sample: 16 of 695 requirement blocks (seed 42) verified line-by-line
  at source — 16/16 true.
MECHANICAL ROUND (no subagents, in-session adjudication of script-distilled lists):
- tools/mech-audit.py detectors: seq 22/22 benign; qregion 105 benign (existence
  proven tree-wide); formula 265 benign (path pollution + previously verified);
  dir 265 checked pairs, zero inversions; idregion 1176 informational only
  (tree-wide identifier existence already clean).
- Real fix from this round: 36 Unicode math symbols (x / minus / -> / >=) plus 5
  plus-minus signs normalized to ASCII; remaining non-ASCII = em dashes only.
- Spot-verified en route: feeling band table, regen floor at 1 hp, cuts 1/2/3
  tiers, approximate_distance 41/25/8 tiers.
Snapshot protocol: take cp -r specs /tmp/opencode/spec-snapshot-waveN BEFORE any
future wave so the wave yields a real file-level diff.
Final gates: lint 91/91 PASS; anchor diff = 24 entries, all documented corrections
(header-anchor re-points + the 3 original data-file fixes); 2203 identifiers checked,
sole unresolved = EFF_DIR5 (intentional gap note); CJK zero; tome2spec check PASS.
Corpus: 16897 lines, 584 scenarios.
Final gate (post-audit): spec-lint 91/91 PASS; anchor diff = 4 documented corrections only
(t_info 1-42->1-41, book-101 1-5->1-4, test.lua 6-64->finer, re_info.txt 15->17);
zero CJK; zero fabricated quotes (4 suspects all %s/%c-abbreviated real messages);
identifier corpus check 2187 tokens, sole gap = EFF_DIR5 (intentional, documented);
tome2spec check PASS; corpus 16778 lines, 584 scenarios.
- Identifier-corpus check (2112 backticked tokens vs source tree): 7 real errors fixed
  (PWR_BR_ELEM, REW_MASS_GEN, private_check_user_directory, do_cmd_automatizer,
  control_one_undead, invented is_resting removed, FINAL_ARTIFACT_203 example);
  2 false positives kept (EFF_DIR5 absent-gap note, R_CHAR_Z data form).
- B3/B6/B7/B8/B9 subagents hit the usage cap after writing their files but before
  reporting; all their files verified by the main session via spec-lint --diff-old
  (zero dropped anchors) and spot reads. The one unfinished file, quest, was
  rewritten by the main session (proper nouns re-verified against source:
  The Watcher in the Water / Feagwath / Dwar, Dog Lord of Waw / Hoarmurath of Dir /
  SV_POTION_AUGMENTATION / create_molds_hook).
- Intentional anchor-set deltas vs backup (corrections, not drops):
  content-maps t_info.txt 1-42 -> 1-41 (file has 41 lines);
  flavor-tables book-101.txt 1-5 -> 1-4 (file has 4 lines);
  scpt-misc test.lua 6-64 -> refined finer ranges (8-101 etc.) after re-read.

## Dispatch prompt skeleton (per batch; used verbatim in subagent prompts)

```
Goal: rewrite N existing analysis specs from Chinese into English, unified style, in place.
Read FIRST and follow exactly: /home/feng/Projects/tome/.ref/tome2-specs/tools/STYLE-GUIDE.md
Game source (read-only fact source): /home/feng/Projects/tome/.ref/tome2/
Pre-rewrite backup (do not modify): /tmp/opencode/spec-backup/
Files to rewrite (paths relative to /home/feng/Projects/tome/.ref/tome2-specs/):
<list specs/<name>/spec.md>
Per file: read the Chinese spec; rewrite fully in English per the guide; overwrite in place.
Fidelity: every fact, number, identifier, quirk, and anchor reference must survive;
do not summarize facts away; do not invent facts.
Fact audit: when a statement is unclear or looks wrong, open the anchored region under
/home/feng/Projects/tome/.ref/tome2/ and verify; fix and report the correction. Targeted reads only.
Output: zero CJK characters; never write the joined substrings listed in guide rule 1 (rephrase instead).
Rename the anchor line to "- **Anchors**:" (was zh). Do not run tools/tome2spec.py, do not touch
state.json, 00-*.md, or any file outside the list.
Report one line per file: <name> OK | CORRECTED: <what> | ANOMALY: <what>; then "invalid anchors: <list|none>".
```
