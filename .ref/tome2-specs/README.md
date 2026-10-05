# ToME 2.3.5 — As-Is Specification Corpus

A complete as-is specification of the game ToME 2.3.5 (source tree at the read-only
sibling directory `../tome2/`), written as the reference for a future
re-implementation. Every normative statement describes what the shipped code actually
does — never design intent. Implementation bugs, dead code, and comment-vs-code
contradictions are recorded as facts with `**Quirk:**` / `**Dead code:**` /
`**Discrepancy:**` markers.

## Layout

```
00-inventory.md   every source/data/script file, one line each: status and the
                  spec(s) it was analyzed into (generated from state.json)
00-ledger.md      per-file analysis record with content fingerprints (generated)
state.json        the ledger of record (per-file class/status/sha256/specs/note);
                  the two 00-*.md files are regenerated from it by `report`
specs/<name>/     91 capability specifications, OpenSpec-shaped:
spec.md             # <name> Specification / ## Purpose / ## Requirements /
                    ### Requirement: (SHALL premise, one-fact-per-bullet mechanics,
                    #### Scenario: WHEN/THEN) / - **Anchors**: `path:line` refs
tools/            the machinery (see tools/README.md and below)
```

## Reading the corpus

- To find where a source file is documented: grep its path in `00-inventory.md`.
- To find where a mechanic lives: grep the identifier or the capability name in
  `specs/`; cross-references between capabilities use `see specs/<name>/spec.md`.
- Anchors (`src/melee1.c:60-81`) point into `../tome2/`; read the anchored region
  for the authoritative detail — specs never re-transcribe code, tables, or data.

## Scope

Excluded by design (registered as skipped with reasons in the ledger): platform
port code, music/fonts/sounds/graphics, help text. Everything else — engine C,
tolua `.pkg` declarations, Lua (`lib/core`, `lib/scpt`, `lib/mods`, `lib/module.lua`),
data word-lists (`lib/edit`), maps and dungeon definitions (`lib/edit/*.map`,
`lib/dngn`), and in-game text tables (`lib/file`) — is analyzed: 890 files on the
ledger, 289 done, 601 skipped with reasons.

## Quality history and how convergence was judged

The corpus went through five passes; every pass's fixes are logged in
`tools/rewrite-plan.md` (notes section):

1. **Analysis** (Chinese drafts): every file read once, tracked in `state.json`.
2. **Rewrite**: Chinese -> English, unified skeleton (see `tools/STYLE-GUIDE.md`).
3. **Audit wave**: independent agents re-verified every file against the source
   (~150 fixes).
4. **Fresh-model audit wave**: another independent pass (~60 more fixes, mostly
   inversions and field misreads that had survived).
5. **Convergence check**: 95 further items, ~half wording/count-level.

Convergence was then established mechanically instead of by another full agent wave
(cost control — scripts pre-distill, judgement adjudicates):

- **Invented constants**: `tools/num-anchor-check.py` — 1925 (number, requirement)
  pairs; every numeric claim must appear in its anchor region or the whole tree.
  Result: 1 survivor, adjudicated as a legitimate derived range.
- **Invented identifiers**: all 2203 backticked identifiers must exist in the
  source tree. Result: 1 survivor, an intentional gap note (`EFF_DIR5` absent by
  design in the source).
- **Invented messages**: every quoted in-game string must exist in the tree.
  Result: zero fabrications.
- **Inversions**: `tools/mech-audit.py` pairs every direction-word claim (below/
  above/exceeds/...) with same-number comparator lines from its anchor regions;
  adjudication found zero real inversions left.
- **Acceptance sampling**: 16 of 695 requirement blocks (fixed seed 42) verified
  line-by-line at source — 16/16 true.

Verdict: converged. If more assurance is ever wanted, widen the acceptance sample —
do not launch another full pass.

## Tools

All runnable from this directory (`python3 tools/<name>.py`):

| tool | purpose |
|---|---|
| `tome2spec.py` | analysis ledger: `scan` (re-fingerprint the tree), `next` (work queue), `show`, `mark` (record a file as analyzed into specs), `skip`, `check` (the completion gate), `report` (regenerate the 00-*.md views) |
| `spec-lint.py` | corpus gate: English-only, skeleton present, one Anchors line per requirement, anchors resolve (file exists, lines in range), banned-token filter; `--diff-old` diffs anchor sets against a pre-change snapshot |
| `corpus-profile.py` | granularity/style profile per capability (requirement/scenario/anchor counts vs the snapshot; flags merges, shrinkage, missing scenarios; checks cross-ref targets) |
| `num-anchor-check.py` | number-vs-anchor sweep with a whole-tree second pass (see above) |
| `mech-audit.py` | formula echo, sequence tables, identifier/quote region coverage, direction-word pairing |

## Working on this corpus later

- **Source changed?** Re-fingerprint first: `python3 tools/tome2spec.py scan` —
  files whose hash drifted fall out of `done`; re-analyze only those, then `mark`
  and `check`. Never re-read a file whose fingerprint is unchanged (tokens cost).
- **Any review wave?** Snapshot first: `cp -r specs /tmp/opencode/spec-snapshot-N`
  so the wave yields a real file-level diff, and lint with `spec-lint.py
  --diff-old` afterwards.
- **Writing specs?** Follow `tools/STYLE-GUIDE.md` exactly; a writing filter
  rejects a handful of words (guide rule 1).
