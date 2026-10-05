#!/usr/bin/env python3
"""Cheap mechanical fact check: do the numbers in a spec actually appear in the
anchored source region?

For every requirement block in every specs/<name>/spec.md:
  1. collect integer literals (value >= 3) from the block text, excluding all
     anchor references (line numbers live there)
  2. collect the block's anchor references (path:line[-end])
  3. a number absent from every anchored region of its own block is a suspect
     for a [COUNT]/[FIELD]-class error; a number absent from the WHOLE game
     tree is HIGH suspicion (invented or misremembered value)

Benign classes (adjudicated by the reviewer): formula coefficients, rolled
bounds, data-file example values.
"""
import os
import re

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPEC_ROOT = os.path.join(BASE, "specs")
GAME_ROOT = os.path.join(os.path.dirname(BASE), "tome2")

ANCHOR = re.compile(r"((?:src|lib)/[A-Za-z0-9_./+-]+?):(\d+)(?:-(\d+))?")
NUM = re.compile(r"(?<![A-Za-z0-9_./:])(\d{1,6})(?![A-Za-z0-9_])")
BLOCK = re.compile(r"^### Requirement:", re.M)
LINEREF = re.compile(r"[A-Za-z0-9_.\-]*\.(?:c|h|lua|txt|map|pkg|prf|atm)?:\d+(?:-\d+)?|:\d+(?:-\d+)?")

_cache = {}


def region_text(path, a, b):
    if path not in _cache:
        try:
            with open(os.path.join(GAME_ROOT, path), encoding="utf-8", errors="ignore") as f:
                _cache[path] = f.read().splitlines()
        except OSError:
            _cache[path] = []
    lines = _cache[path]
    return "\n".join(lines[max(a - 1, 0):b])


def corpus_nums_of(text):
    # data files carry values colon-prefixed (F:G:89:5:955); allow those
    return set(int(m) for m in re.findall(r"(?<![A-Za-z0-9_./])(\d+)(?![A-Za-z0-9_])", text))


def nums_of(text):
    return set(int(m) for m in NUM.findall(text))


def main():
    names = sorted(d for d in os.listdir(SPEC_ROOT)
                   if os.path.isdir(os.path.join(SPEC_ROOT, d)))
    buf = []
    for root, dirs, files in os.walk(GAME_ROOT):
        for f in files:
            if f.endswith((".c", ".h", ".lua", ".pkg", ".txt", ".map")):
                try:
                    buf.append(open(os.path.join(root, f),
                                    encoding="utf-8", errors="ignore").read())
                except OSError:
                    pass
    corpus_nums = corpus_nums_of("\n".join(buf))

    suspects = []
    high = []
    total = 0
    for n in names:
        text = open(os.path.join(SPEC_ROOT, n, "spec.md"), encoding="utf-8").read()
        for blk in BLOCK.split(text)[1:]:
            title = blk.splitlines()[0].strip()[:40]
            anchors = [(p, int(a), int(e or a)) for p, a, e in ANCHOR.findall(blk)]
            if not anchors:
                continue
            prose = LINEREF.sub(" ", ANCHOR.sub(" ", blk))
            nums = {v for v in nums_of(prose) if v >= 3}
            if not nums:
                continue
            merged_nums = nums_of("\n".join(region_text(p, a, b) for p, a, b in anchors))
            for v in sorted(nums):
                total += 1
                if v not in merged_nums:
                    suspects.append((n, title, v))
                    if v not in corpus_nums:
                        high.append((n, title, v))
    print(f"checked {total} pairs; suspects (absent from anchor regions): {len(suspects)}; "
          f"HIGH (absent from whole tree): {len(high)}")
    for n, t, v in high:
        print(f"HIGH {n} | {t} | {v}")


if __name__ == "__main__":
    main()
