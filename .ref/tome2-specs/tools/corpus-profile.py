#!/usr/bin/env python3
"""Corpus profile for the English rewrite: old-vs-new granularity and style stats.

For every specs/<name>/spec.md, compared against the pre-rewrite backup at
/tmp/opencode/spec-backup/:
  reqs     '### Requirement:' count (old -> new)
  scen     '#### Scenario:' count (new)
  anchors  parsed anchor refs (old -> new)
  lines    file lines (old -> new)
  markers  Quirk / Dead code / Discrepancy bullets (new)
Flags (review suspects):
  MERGE    new requirement count < 70% of old (granularity collapse)
  SHRUNK   new line count < 90% of old (possible fact loss; zh->en usually expands)
  NOSCEN   new file has zero scenarios
Also cross-ref check: every specs/<x>/spec.md reference must exist.
"""
import os
import re

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPEC_ROOT = os.path.join(BASE, "specs")
OLD_ROOT = "/tmp/opencode/spec-backup"

REQ = re.compile(r"^### Requirement:", re.M)
SCEN = re.compile(r"^#### Scenario:", re.M)
ANCHOR = re.compile(r"((?:src|lib)/[A-Za-z0-9_./+-]+?):(\d+)(?:-(\d+))?")
XREF = re.compile(r"specs/([a-z0-9-]+)/spec\.md")
MARK = re.compile(r"\*\*(Quirk|Dead code|Discrepancy):\*\*")


def stats(path):
    with open(path, encoding="utf-8") as f:
        t = f.read()
    return {
        "reqs": len(REQ.findall(t)),
        "scen": len(SCEN.findall(t)),
        "anchors": len(ANCHOR.findall(t)),
        "lines": t.count("\n") + 1,
        "marks": len(MARK.findall(t)),
        "text": t,
    }


def main():
    names = sorted(d for d in os.listdir(SPEC_ROOT)
                   if os.path.isdir(os.path.join(SPEC_ROOT, d)))
    all_xrefs = set()
    print(f"{'capability':24} {'reqs':>7} {'scen':>5} {'anchors':>9} {'lines':>9} {'mk':>3}  flags")
    for n in names:
        new = stats(os.path.join(SPEC_ROOT, n, "spec.md"))
        old = stats(os.path.join(OLD_ROOT, n, "spec.md"))
        all_xrefs.update(XREF.findall(new["text"]))
        flags = []
        if new["reqs"] < old["reqs"] * 0.7 and old["reqs"] >= 3:
            flags.append(f"MERGE {old['reqs']}->{new['reqs']}")
        if new["lines"] < old["lines"] * 0.9:
            flags.append(f"SHRUNK {old['lines']}->{new['lines']}")
        if new["scen"] == 0:
            flags.append("NOSCEN")
        print(f"{n:24} {old['reqs']:>3}->{new['reqs']:<3} {new['scen']:>5} "
              f"{old['anchors']:>4}->{new['anchors']:<4} {old['lines']:>4}->{new['lines']:<4} "
              f"{new['marks']:>3}  {' '.join(flags)}")
    missing = sorted(x for x in all_xrefs if not os.path.isdir(os.path.join(SPEC_ROOT, x)))
    print("\ncross-ref targets missing:", missing or "none")


if __name__ == "__main__":
    main()
