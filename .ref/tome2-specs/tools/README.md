# Analysis workflow (tome2spec.py)

Goal of the analysis phase: produce a complete as-is spec of `../tome2/` (ToME
2.3.5) for later re-implementation reference. That phase is complete — this file
documents the loop for incremental re-analysis (e.g., after the source changes).
See the root `../README.md` for the corpus overview and the convergence story.

## The two hard constraints

- **Nothing missed**: `state.json` registers every file under `../tome2`.
  Analysis is complete when `check` prints PASS: no pending files, no
  unclassified files, no hash drift on done files, all spec references resolvable,
  every skipped file carries a reason.
- **Nothing analyzed twice**: each file gets one full analysis. `mark` refuses a
  second mark for unchanged content; before touching anything, consult
  `00-ledger.md` or `show <path>` — a fingerprint hit means skip, never re-read.

## The loop

1. `python3 tools/tome2spec.py scan` — re-fingerprint the tree (after source
   changes; drifted files fall out of `done` back into the queue).
2. `python3 tools/tome2spec.py next` — work queue (word-lists first, then engine
   C, then tolua declarations, then Lua).
3. Analyze one file: read it, extract behavior, write or extend
   `specs/<capability>/spec.md`; requirements and scenarios carry `path:line`
   anchors, English only, per `STYLE-GUIDE.md`.
4. `python3 tools/tome2spec.py mark <path> --spec specs/<capability>/spec.md
   --note "<coverage decision>"`.
5. Finish with `python3 tools/tome2spec.py check`; report progress.

## Conventions

- Grepping anchors inside other files while analyzing one file is fine, but that
  does not count as analyzing those files — each still needs its own pass.
- Giant word-lists: analyze the format and flag semantics (header notes + parser
  code + sampled entries); entries are content data, not analyzed one by one —
  say so in the `mark` note.
- One file's behavior may spread across several capabilities; pass several
  `--spec` flags so the ledger records all of them.
- Before delivering, grep the written text against the user-level language
  blacklist (`~/.agents/language-blacklist.txt`) — English text avoids it by
  construction; the writing filter enforces a few English tokens too (see
  STYLE-GUIDE rule 1).
