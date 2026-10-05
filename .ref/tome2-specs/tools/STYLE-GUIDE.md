# Style Guide — English rewrite of the tome2 as-is specs

This guide governs the one-pass rewrite of every `specs/<name>/spec.md` from Chinese
into English. It is written down so the standard survives long sessions and context
compression: follow it exactly, do not improvise a personal style.

Rewrite completion gate (all four must hold):

1. `python3 tools/spec-lint.py` exits 0 (English-only, structure, anchors resolve).
2. Anchor diff against the pre-rewrite backup (`/tmp/opencode/spec-backup/`) shows no
   silently dropped anchor (`python3 tools/spec-lint.py --diff-old`).
3. `python3 tools/tome2spec.py check` still PASS (the analysis ledger is untouched by
   the rewrite; same file paths, same spec references).
4. Batch status in `tools/rewrite-plan.md` all done.

## What these specs are

As-is specifications of ToME 2.3.5. The game source tree is mounted read-only at
`.ref/tome2/`. `SHALL` records what the shipped code actually does — never design
intent. Where code and its comments (or the data files) disagree, the spec records the
code behavior and flags the mismatch with a `**Discrepancy:**` bullet. Implementation
bugs and oddities stay in the spec, recorded factually, with the markers below.

## File skeleton (fixed)

```markdown
# <capability-name> Specification

## Purpose

<2-6 sentences: what this capability covers, which source files, scope notes,
cross-references to other capabilities.>

## Requirements

### Requirement: <Short Noun Phrase Title>

<One or more SHALL sentences recording the as-is behavior.>

<Optional mechanism bullets — one fact per bullet.>

#### Scenario: <short name>

- **WHEN** <trigger and preconditions>
- **THEN** <observable outcome in the shipped code>

- **Anchors**: `src/foo.c:123-156`, `lib/edit/bar.txt:40`
```

## Rules

1. **English only.** Zero CJK characters anywhere in the file. Plain, direct technical
   prose. Never write the substrings `b​eat`, `p​ark`, `f​lip`, or the phrases
   `f​lip the cell` / `f​lips the cell` (zero-width spacers here only to keep this file
   lint-clean; the writing filter rejects even the bare single words, not just the
   phrases) — pick other words ("struck down", "exceeds", "reversed", "stop moving",
   "set the cell coordinate to the target").
2. **Normative voice, then mechanism.** Each requirement opens with SHALL sentence(s)
   (the premise), followed by supporting mechanism bullets. Split dense paragraph-walls
   into bullets — one fact per bullet. A bullet may still cite inline values.
3. **Fidelity.** Preserve every fact, number, identifier, quirk, and anchor reference
   from the old spec. Never drop a bullet, never fold two facts into one vague
   sentence. The anchor set is diffed mechanically against the pre-rewrite backup;
   silently dropped anchors fail the review. Adding clarifying anchors is fine.
4. **Anchors.** Every Requirement block ends with exactly one `- **Anchors**:` line.
   References are `` `path:line` `` or `` `path:line-range` `` relative to `.ref/tome2/`
   (e.g. `` `src/init1.c:6221-6233` ``). Multiple references separated by `, `.
   A short parenthetical note per reference is allowed (`` `src/levels.c:82-94`
   (naming sample) ``). Do not invent references; keep the old ones verbatim unless a
   fact-audit correction changes them (report that).
5. **Scenarios.** Include `#### Scenario:` blocks for concrete trigger-to-outcome
   behavior — at least one per requirement when the behavior is conditional,
   interactive, or has distinct cases. Pure data-registry requirements (entry
   listings, format tables) may omit scenarios. WHEN may carry preconditions.
6. **Fact audit.** If a statement in the old spec is unclear, self-contradictory, or
   looks wrong, open the anchored region under `.ref/tome2/` and verify against the
   real code/lua/data. Correct the statement, and record the correction in your final
   report. Targeted reads only — never re-read whole systems; the analysis is complete
   and paid for. Do not propagate a claim you cannot support.
7. **Identifiers verbatim.** Macro names, constants, file names, function names, and
   in-game strings stay in backticks unchanged (`DF2_NO_STAIR`, `q_rand.c`,
   `"Nobody ever turns up..."`).
8. **Cross-references** to other capabilities use their paths:
   `see specs/map-format/spec.md`. Rewrite workflow phrasing from the old specs
   ("随 X 分析追加", "本文件只留桥接", "is registered with the X analyses") into plain
   cross-references of that form, or a factual pointer such as "documented under
   `src/xtra1.c`". Never mention the analysis workflow itself in a spec.
9. **Titles.** `# <capability-name> Specification` keeps the existing directory name.
   Requirement titles are short noun phrases in Title Case.
10. **As-is defect markers**, as bullet prefixes:
    - `**Quirk:**` odd but live behavior;
    - `**Dead code:**` unreachable or never exercised in practice;
    - `**Discrepancy:**` code contradicts its comment or the data file.

## Terminology (fixed translations, use consistently)

| old (zh) | English |
|---|---|
| 词表 | word-list (a data file under `lib/edit/`) |
| 词条 | entry |
| 解析 / 装载 | parse / load |
| 城镇 / 毁镇 | town / destroyed-town state |
| 建筑物 | building |
| 特殊层 | special level |
| 分支 / 层深 / 关卡 | branch / depth / dungeon level |
| 野外 | wilderness |
| 骸骨文件 | bone file (player ghost) |
| 宠物 | pet |
| 主保 | patron (Chaos patron) |
| 学派 / 法术 | school (of magic) / spell |
| 六维 / 前置 | the six stats / prerequisite |
| 发动键 | activation key (`mkey`) |
| 钩子 | hook |
| 剧情 / 剧情线 / 任务 | plot / plot line / quest |
| 遗言 / 传闻 | dying words / rumor |
| 咒物 | cursed item |
| 怪物记忆 | monster memory |
| 天赋 | power |
| 存档 | savefile |
| 随机神器 | random artifact (randart) |
| 楼梯 / 陷阱 | staircase / trap |
| 可用数 | availability count |

## Worked example (illustrative shape only, not real content)

```markdown
### Requirement: Recharge Failure On Cursed Devices

Recharging a cursed device SHALL fail without consuming a charge, and the device SHALL
then read as identified-cursed to the player.

#### Scenario: Failure path

- **WHEN** the player recharges a cursed wand and the failure roll triggers
- **THEN** the charge count is unchanged and a failure message built from
  `lib/file/sfail.txt` plays

- **Quirk:** the failure path also resets the recharge counter, unlike the success path.
- **Anchors**: `src/cmd7.c:4809-4830`, `lib/file/sfail.txt:1-6`
```
