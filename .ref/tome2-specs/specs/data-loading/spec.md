# data-loading Specification

## Purpose

Template word-list parsing: `src/init1.c` (under ALLOW_TEMPLATES) implements the parsers
for all `lib/edit/*.txt` word-lists (the init_*_txt family), the flag-name table constants
(the r/k/f/d/t/wf/st/rp/s/e/ego/esp groups), the modifier converters, and the include
stack. The common word-list contract (header comment format, the V/N/D generic lines, error
codes) is registered in `specs/edit-format/spec.md`; this spec records only the line
grammars and deviations specific to each parser. Entry contents themselves are data and are
not catalogued entry by entry.

## Requirements

### Requirement: Flag Name Tables And Converters

The flag-name tables SHALL let the grab_one_* family set bits by name: monster blow methods
(the 25-entry table — a `*` wildcard plus twenty-four names) and effects (the 35-entry
table — the wildcard plus thirty-four names) (r_info_blow_method/effect), nine monster flag
groups (flags1-9, with flags7 behavior/flags8 habitat/flags9 drops and physiology), six
object flag groups (k_info_flags1-5 plus the flags2_trap trap-only group — note the trap
group shares bit positions with flags2 for same-named entries), one esp group (ESP_ALL on
bit 31), one ego-only group (SUSTAIN/LIMIT_BLOWS and thirty others), f_info_flags1,
d_info_flags1/2, t_info_flags, st_info_flags1, rp_info_flags1/2, s_info_flags1 (the
wf_info_flags1 table is, as shipped, entirely XXX placeholders). The converters SHALL:
`color_char_to_attr` accept the sixteen characters dwsorgbuDWvyRGBU; `monster_ego_modify`
accept +/-/=% mapped to MEGO_ADD/SUB/FIX/PRC (an unknown character is announced and then
treated as ADD); `conv_color[16]` is the reverse table.

- **Anchors**: `src/init1.c:46-1413` (name table constants), `:1419-1482` (colors), `:1486-1504` (modifiers)

### Requirement: File Include Stack

A word-list `<:filename` line SHALL push the new file onto fp_stack (capacity ten) and
continue parsing there; my_fgets_dostack pops back to the enclosing file automatically at
EOF and only ends when the stack is empty. Include names are relative to ANGBAND_DIR_EDIT.
A failure to open the file quits directly. As-shipped defect: the push prints the string
"ibncluding %s" (the typo is preserved as-is).

- **Anchors**: `src/init1.c:1509-1580`

### Requirement: Name Lookup Helpers

`grab_one_class_flag`/`grab_one_race_allow_flag` SHALL match by class/race title names
(unknown_shut_up silences the unknown-flag announcements); `get_activation` matches a
19-character prefix against the activation_names table (about two hundred names,
NO_ACTIVATION through MUSIC); `get_k_flag`/`get_r_flag` SHALL return the flagx32+bit index
(six object groups and nine monster groups scanned in parallel) for reuse by al_info flag
recipes and the x: activation line; `init_al_info_essence` matches a 9-character prefix
against essence_names.

- **Anchors**: `src/init1.c:1588-1699` (class/race allow), `:4596-4672` (k/r flag and essence), `:1209-1413` (activation names)

### Requirement: p_basic Family Word-Lists (Race, Subrace, Class, Meta Class)

`init_player_info_txt` SHALL parse the four-section mixed format. Generic lines — V
version, `<` include, I resets error_idx, H history rows (idx:roll:chart:next:bonus:text
into bg[]). The race section R: — N name (powers four entries -1, abilities ten entries
reset), D description (joined with \n), E six body-part entries, R:R level flag anchors
(lev remembered + opval[lev]), S six stats plus luck, Z racial powers (looked up by
powers_type name, filling powers[0..3] in order), K eight base skills (dis/dev/sav/stl/
srh/fos/thn/thb), k four skill values (basem:base:modm:mod, modifiers converted), b
abilities (level:name, cur_ab increments), M ten body-size entries (male/female base
height/weight/modifier), P r_mhp:r_exp:infra:chart, G player flags, F level flags (written
to oflags*[lev]), O birth equipment (tval:sval:pval:%dd ds, pval optional defaulting
to zero, obj_num increments), C class-choice bitmap. The subrace pool S: is isomorphic
(S:D sets the place-variant marker by buf[4]=='A'; S:S eight stats including mana; S:P
three entries; S:A allowed-race bitmap; S:C picks the pclass/mclass bitmap by buf[4]). The
class section C: — N (obj_num/tit_idx/spec_idx reset, spec table cleared), D:0 description
/ D:1 title sequence, O/E/R/S (eight entries including mana and extra_blows), k/b/g deity
faith ("All Gods" wildcard bitmap or find_god), Z/K/X eight extra skills, P c_mhp:c_exp,
C five perception rows (H/N heavy and magic switches + sense_base/pl/plus), B
blow_num:blow_wgt:blow_mul, G/F, and the skill-specialization subsection C:a — aN name
(spec_idx increments, capped at MAX_SPEC), aD description, aO object, ag god, ak skill,
ab ability, aG flags, aK expected skill (skill_ideal). The meta-class section M: — MN color
character plus name (classes all reset to -1), MC class-name member table.

- **Anchors**: `src/init1.c:1788-3439` (main loop), `:1943-2314` (race section), `:2317-2726` (subrace pool), `:2729-3358` (class section and specializations), `:3361-3421` (meta class)

### Requirement: Vault And Terrain Word-Lists

`init_v_info_txt` SHALL: skip Q/T map lines (vault maps and their name lines belong to the
map parser); N name / D description / X typ:rat:hgt:wid / Y fifteen entries (mon[0..9]/
item[0..2]/lvl/dun_type, the special-level configuration); with a false start argument the
header and sizes are not reset (supporting incremental vault parsing inside q_*.txt).
`init_f_info_txt` SHALL: pre-seed the three default texts (wall blocks / cannot be dug
through); on an N line default mimic=i and point text/block/tunnel at the defaults;
D:0 terrain text / D:1 digging text / D:2 block text; M alternate feature number;
S seven-color shimmer; G graphics; E up to four effects (NdM:freq[:GF name or number],
freq stored x10); F flags.

- **Anchors**: `src/init1.c:3445-3670` (vault), `:3701-4052` (terrain)

### Requirement: Object, Artifact, And Alchemy Word-Lists

`init_k_info_txt` SHALL: N (esp zeroed, power=-1) / D description (joined on word spaces) /
G/I four entries (pval2 may be SPELL=name — a find_spell value the caller
interprets) / W level:extra:weight:cost / T btval:bsval artifact base / Z granted powers /
a activation (HARDCORE=name via get_activation; SPELL=name as a negative find_spell
number) / A allocation table (a :level[:chance] sequence, chance defaulting to 1) / P
ac:NdM:to_h:to_d:to_a / F flags / f obvious flags (written to the oflags*/oesp mirror
groups). `init_a_info_txt` SHALL: N defaults the four IGNORE_ flags on a bare line and
errors on TR3_ACTIVATE without an activate number; D description joined on spaces;
I three entries (lookup_kind verifies the base exists); W four entries; P/Z/F/f/a as in k.
`init_al_info_txt` SHALL: I tval:sval:qty:essence name (trailing comment cut at the first
space); a flag recipe rows (qty:flag name:essence, with tval zeroed and sval holding the
get_k_flag index); A flag description blocks of seven entries (group:rtval:rsval:rpval:
pval:level:xp, with completeness checks against the previous block — group/desc present
and item_desc matching rtval presence); F flag names, x activation names (flag negated,
group set to 88), f corpse monster flags (restricted to TV_CORPSE, no rpval, at most six
entries, via get_r_flag), p plural descriptions (restricted to pval-dependent blocks),
D description, d object name; al_head text_size records the structure byte count
((a_idx+1) x sizeof).

- **Anchors**: `src/init1.c:4163-4592` (k), `:5107-5456` (a), `:4676-4994` (al)

### Requirement: Item Set, Skill, And Ability Word-Lists

`init_set_info_txt` SHALL: N (member table 6x6 zeroed) / D description / P artifact:number:
pval (locating or appending the member by artifact number, recording cur_art/cur_num) /
F flags written into the current member tier. `init_s_info_txt` SHALL: T parent:child
(father and order registered incrementally) / E exclusive pairs (two-way SKILL_EXCLUSIVE) /
O opposed pairs (negative percentages) / f friendly pairs (positive percentages — the
documentation says A:, the implemented prefix is f:, recorded as-is) / N (action_mkey
re-zeroed, dev false, random_gain_chance defaulting to 100, action table cleared) /
D description / A mkey:activation description / I growth rate / G random-gain chance /
F flags. `init_ab_info_txt` SHALL: N (skills/need_abilities/forbid_abilities ten entries
and stat six entries reset) / D / A mkey:activation description / I cost / k level:skill
(up to ten entries) / a prerequisite ability / S value:stat name (via stat_names lookup) /
E exclusive pairs (two-way forbid).

- **Anchors**: `src/init1.c:5461-5708` (sets), `:5714-6103` (skills), `:6108-6459` (abilities)

### Requirement: Ego And Randart Template Word-Lists

`init_e_info_txt` SHALL: N (ten tval entries set to 255, five flag groups zeroed, power=-1,
cur_r=-1, cur_t=0); D description lines are disabled by `#if 0` (as shipped, e_info has no
D: parse branch); T tval:min:max up to ten lines; R rarity up to five lines (cur_r
incremented then stored in rar[cur_r], the first R line fills rar[0]); X pos:slot:rating
(the slot value is commented out of use, only rating is stored and before=pos=='B');
W level:rarity:mrarity:cost / C max_to_h:max_to_d:max_to_a:max_pval / Z / a / r:N required
flags (the need_flags* six groups) / r:F forbidden flags / F/f flags (indexed by the cur_r
group, an error when cur_r==-1). `init_ra_info_txt` SHALL: G generation table rows
(chance:NdM:plus, ra_gen increments); N (twenty tval entries set to 255, flag groups
zeroed, power=-1); T up to twenty lines; X value:max; W three entries; C four max entries;
Z; F flags (also collecting ego_flags into fego); A resistance flags (written to aflags*/
aesp).

- **Anchors**: `src/init1.c:6685-7123` (e), `:7128-7240` (randart flag conversion), `:7248-7545` (ra)

### Requirement: Monster, Monster Ego, And Trap Word-Lists

`init_r_info_txt` SHALL: N (the four drop groups default to OBJ_GENE_*, freq zeroed twice
over); D/G/I speed:NdM:aaf:ac:sleep; E six body-part entries (more weapons than arms
quits); O four drop-tendency entries; W level:rarity:weight:mexp (a zero weight converts
to 100); B up to four attacks (method:effect:NdM looked up by name); F base flags
(flags1/2/3/7/8/9); S spell flags (1_IN_N becomes freq=100/N, the rest go into flags4/5/6);
post-parse table-wide fixups — flags8 is XORed with 1 (the WILD_ONLY bit inverts its
meaning), monsters carrying only WILD_TOO get flags8=0x0463 (full habitat expansion).
`init_re_info_txt` SHALL: N (blow/r_char/nr_char reset, blowm defaulting to MEGO_ADD);
G graphics (* wildcard means MEGO_CHAR_ANY); I/W/B of six/five/four entries all carrying
modifier prefixes (the value is shifted left two places with the low two bits holding the
MEGO mode); F required flags (R_CHAR_X collected, capped at five); H forbidden (R_CHAR_
capped at five); M/O base flags (O supports the MF_ALL full ban); S/T spell flags (S
supports 1_IN_N, T supports MF_ALL). As shipped: the E: line advertised in the header
comment has no parse branch. `init_t_info_txt` SHALL: N/D/I eight entries (difficulty:probability:
another:p1valinc:minlevel:NdM:color character) / F flags (each line first zeroes, a
single line overwrites).

- **Anchors**: `src/init1.c:7670-8093` (r), `:8325-8854` (re), `:8884-9110` (t)

### Requirement: Dungeon And Store Family Word-Lists

`init_d_info_txt` SHALL: N (size/ix/iy/ox/oy default -1, fill_method default 1, the five
rules modes zeroed, the four object groups defaulted, generator defaulting to "dungeon");
D takes the first three characters as short_name and the rest as description; W six entries
mindepth:maxdepth:min_plev:next:min_m_alloc_level:max_m_alloc_chance; L the three floor
types (f1:p1:f2:p2:f3:p3, with the short form storing only the percentage in slot [1]);
O drop tendencies; G generator name; A the three wall types in eight entries (including
outer_wall/inner_wall) or the short form; E effects (as in f_info, GF names via
d_info_dtypes); F flags (dedicated tokens — WILD_x_y__x_y entrances and exits, SIZE_x_y,
FILL_METHOD_n, FINAL_OBJECT_n, FINAL_ARTIFACT_n, FINAL_GUARDIAN_n, the rest through
d_info_flags1/2); R percent:mode monster rules (rule_num increments and the accumulated
boundaries expand into rule_percents[100]); M/S rule flags (R_CHAR_ capped at five).
`init_st_info_txt` SHALL: N; I item-name lookup rows (test_item_name plus rarity);
T rarity:tval:sval (an sval of 256 or more means the whole tval, stored as tv+10000);
G/A the six actions / F store flags / O the four owners / W max_obj (clamped to
STORE_INVEN_MAX). `init_ba_info_txt` SHALL: N/C the three cost tiers (HATED/NORMAL/LIKED) /
I action:action_restr:letter[:letter_aux]. `init_ow_info_txt` SHALL: N/C the three cost
factors / I max_cost:max_inflate:min_inflate:haggle_per:insult_max / L liked and H hated
race or class bitmaps (grab_one_race_flag tries race before class).

- **Anchors**: `src/init1.c:9268-9872` (d), `:9932-10231` (st), `:10236-10416` (ba), `:10421-10654` (ow)

### Requirement: Wilderness Terrain Word-List

`init_wf_info_txt` SHALL: N name / D description / W six entries level:entrance:road:feat:
terrain_idx:map character (the character is registered as an index via the wildc2i reverse
table); X the eighteen-entry terrain table (MAX_WILD_TERRAIN); F flags (the name table is,
as shipped, all XXX placeholders — no valid flag can be parsed).

- **Anchors**: `src/init1.c:10683-10922`
