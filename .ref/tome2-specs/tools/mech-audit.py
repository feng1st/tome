#!/usr/bin/env python3
"""Mechanical pre-distillation for a review round (no LLM inside).

Five detectors over specs/<name>/spec.md; each prints a compact adjudication
list (spec line + the relevant source line) for the human/LLM reviewer:

  formula   backticked formula-like strings (containing randint/rand_int/damroll/
            magik/operators) normalized and compared against anchor regions
  seq       slash-separated numeric sequences (>=3 items) compared against the
            numbers appearing in anchor regions (order-insensitive multiset)
  idregion  backticked identifiers absent from ALL of their block's anchor
            regions (possible anchor pointing at the wrong region)
  qregion   quoted strings absent from their block's anchor regions
  dir       bullets with a direction word (below/above/at least/at most/exceeds/
            over/under/fewer/greater) plus a number, paired with anchor-region
            lines that carry the same number and a comparator
"""
import os
import re

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPEC_ROOT = os.path.join(BASE, "specs")
GAME_ROOT = os.path.join(os.path.dirname(BASE), "tome2")

ANCHOR = re.compile(r"((?:src|lib)/[A-Za-z0-9_./+-]+?):(\d+)(?:-(\d+))?")
BLOCK = re.compile(r"^### Requirement:", re.M)
LINEREF = re.compile(r"[A-Za-z0-9_.\-]*\.(?:c|h|lua|txt|map|pkg|prf|atm)?:\d+(?:-\d+)?|:\d+(?:-\d+)?")
BACKTICK = re.compile(r"`([^`]+)`")
SEQ = re.compile(r"\b\d+(?:/\d+){2,}\b")
QUOTE = re.compile(r'"([^"\n]{8,})"')
DIRWORD = re.compile(r"\b(below|above|at least|at most|no more than|not more than|"
                     r"exceeds?|over|under|fewer|greater)\b", re.I)
NUM = re.compile(r"(?<![A-Za-z0-9_./:])(\d{1,6})(?![A-Za-z0-9_])")
FORMULA_HINT = re.compile(r"randint|rand_int|damroll|magik|randnor|rand_range|\+|\*|/|%|<<|>>")

_cache = {}


def lines_of(path):
    if path not in _cache:
        try:
            with open(os.path.join(GAME_ROOT, path), encoding="utf-8", errors="ignore") as f:
                _cache[path] = f.read().splitlines()
        except OSError:
            _cache[path] = []
    return _cache[path]


def region_text(path, a, b):
    ls = lines_of(path)
    return "\n".join(ls[max(a - 1, 0):b])


def norm(s):
    s = re.sub(r"[A-Za-z_][A-Za-z0-9_]*\.", " ", s)   # p_ptr. / m_ptr-> style prefixes
    s = re.sub(r"[A-Za-z_][A-Za-z0-9_]*->", " ", s)
    return re.sub(r"\s+", "", s)


def main():
    names = sorted(d for d in os.listdir(SPEC_ROOT)
                   if os.path.isdir(os.path.join(SPEC_ROOT, d)))
    out = {"formula": [], "seq": [], "idregion": [], "qregion": [], "dir": []}
    for n in names:
        text = open(os.path.join(SPEC_ROOT, n, "spec.md"), encoding="utf-8").read()
        for blk in BLOCK.split(text)[1:]:
            title = blk.splitlines()[0].strip()[:40]
            anchors = [(p, int(a), int(e or a)) for p, a, e in ANCHOR.findall(blk)]
            if not anchors:
                continue
            merged = "\n".join(region_text(p, a, b) for p, a, b in anchors)
            merged_norm = norm(merged)
            prose = LINEREF.sub(" ", ANCHOR.sub(" ", blk))

            # 1 formula
            for b in BACKTICK.findall(prose):
                if FORMULA_HINT.search(b) and re.search(r"\d", b) and len(b) > 10:
                    if norm(b) not in merged_norm:
                        out["formula"].append((n, title, b.strip()[:80]))

            # 2 seq
            for m in SEQ.findall(prose):
                spec_seq = [int(x) for x in m.split("/")]
                region_nums = [int(x) for x in NUM.findall(merged)]
                from collections import Counter
                if Counter(spec_seq) - Counter(region_nums):
                    out["seq"].append((n, title, m))

            # 3 idregion (only region-covered ids: also accept if id appears in region)
            for b in BACKTICK.findall(prose):
                for tok in re.findall(r"[A-Za-z_][A-Za-z0-9_]{3,}", b):
                    if ("_" in tok or tok.isupper()) and tok not in merged:
                        out["idregion"].append((n, title, tok))

            # 4 qregion
            for q in QUOTE.findall(prose):
                if q not in merged and q in open(os.path.join(SPEC_ROOT, n, "spec.md")).read():
                    out["qregion"].append((n, title, q[:60]))

            # 5 dir heuristic
            for line in blk.splitlines():
                if DIRWORD.search(line):
                    for v in NUM.findall(LINEREF.sub(" ", line)):
                        for src in merged.splitlines():
                            if re.search(r"(?<![A-Za-z0-9_./:])" + re.escape(v) + r"(?![A-Za-z0-9_])", src) \
                               and re.search(r"[<>=!]", src):
                                out["dir"].append((n, title, line.strip()[:70], src.strip()[:70]))
                                break

    for k, items in out.items():
        print(f"== {k}: {len(items)}")
        for it in items[:60]:
            print("  | ".join(it))
        if len(items) > 60:
            print(f"  ... +{len(items)-60} more")


if __name__ == "__main__":
    main()
