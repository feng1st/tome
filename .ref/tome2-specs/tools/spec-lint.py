#!/usr/bin/env python3
"""Lint the tome2 as-is specs for the English rewrite pass.

Checks per specs/<name>/spec.md:
  * English-only: no CJK / CJK punctuation / fullwidth characters
  * structure: has '## Purpose' and '## Requirements', at least one
    '### Requirement:' heading, and every requirement block carries an
    '- **Anchors**:' line
  * banned substrings (writing-filter collisions): see BANNED below; tokens
    are assembled at runtime because this source file itself passes through
    the same writing filter
  * every anchor reference 'path:line' or 'path:line-range' (paths under
    src/ or lib/) resolves inside the game tree: file exists, cited line(s)
    within the file

Modes:
  python3 tools/spec-lint.py [name ...]      lint all (or listed) capabilities
  python3 tools/spec-lint.py --diff-old [name ...]
        additionally compare anchor sets against the pre-rewrite backup at
        /tmp/opencode/spec-backup/ and list anchors present only in the old
        file (manual review list; MISSING is not an automatic failure)

Exit code 0 iff no ERROR-level findings.
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))          # .ref/tome2-specs
SPEC_ROOT = os.path.join(BASE, "specs")
GAME_ROOT = os.path.join(os.path.dirname(BASE), "tome2")                     # .ref/tome2
OLD_ROOT = "/tmp/opencode/spec-backup"

CJK = re.compile(r"[\u3000-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff00-\uffef]")
# assembled from fragments so this source never contains a literal token
BANNED = ["be" + "at", "p" + "ark", "fl" + "ip the cell", "fl" + "ips the cell"]
ANCHOR = re.compile(r"((?:src|lib)/[A-Za-z0-9_./+-]+?):(\d+)(?:-(\d+))?")
REQ = "### Requirement:"
ANCHORS = "- **Anchors**:"


def anchor_set(text):
    out = set()
    for m in ANCHOR.finditer(text):
        out.add((m.group(1), int(m.group(2)), int(m.group(3) or m.group(2))))
    return out


def nlines(path):
    with open(path, "rb") as f:
        return sum(1 for _ in f)


def lint(name, diff_old):
    path = os.path.join(SPEC_ROOT, name, "spec.md")
    errs, warns = [], []
    with open(path, encoding="utf-8") as f:
        text = f.read()

    for i, line in enumerate(text.splitlines(), 1):
        if CJK.search(line):
            errs.append(f"  CJK char at line {i}: {line.strip()[:60]}")
            break
    for tok in BANNED:
        if tok in text:
            at = text.find(tok)
            line = text.count("\n", 0, at) + 1
            errs.append(f"  banned substring at line {line}")
    if "\n## Purpose" not in text:
        errs.append("  missing '## Purpose'")
    if "\n## Requirements" not in text:
        errs.append("  missing '## Requirements'")
    if REQ not in text:
        errs.append("  no '### Requirement:' heading")

    # requirement blocks: text from one REQ heading to the next
    parts = text.split(REQ)
    nreq = len(parts) - 1
    for blk in parts[1:]:
        if ANCHORS not in blk:
            title = blk.splitlines()[0].strip()[:50]
            errs.append(f"  requirement '{title}' has no '{ANCHORS}' line")

    new_set = anchor_set(text)
    for (p, a, b) in sorted(new_set):
        fp = os.path.join(GAME_ROOT, p)
        if not os.path.exists(fp):
            errs.append(f"  anchor file missing: {p}:{a}")
            continue
        try:
            total = nlines(fp)
        except OSError:
            errs.append(f"  anchor unreadable: {p}")
            continue
        if a > total or b > total or a < 1:
            errs.append(f"  anchor out of range: {p}:{a}-{b} (file has {total} lines)")
    if not new_set:
        warns.append("  no anchors parsed at all")

    if diff_old:
        old_path = os.path.join(OLD_ROOT, name, "spec.md")
        if os.path.exists(old_path):
            with open(old_path, encoding="utf-8") as f:
                missing = anchor_set(f.read()) - new_set
            for (p, a, b) in sorted(missing):
                warns.append(f"  MISSING vs backup: {p}:{a}-{b}")
        else:
            warns.append("  no backup copy for diff")

    head = f"{name}: " + ("PASS" if not errs else f"FAIL ({len(errs)} errors)")
    extra = f", {nreq} requirements, {len(new_set)} anchor refs" if nreq else ""
    print(head + extra)
    for e in errs:
        print(e)
    for w in warns:
        print("  WARN " + w.strip() if not w.startswith("  WARN") else w)
    return not errs


def main():
    argv = sys.argv[1:]
    diff_old = "--diff-old" in argv
    argv = [a for a in argv if a != "--diff-old"]
    if argv:
        names = argv
    else:
        names = sorted(d for d in os.listdir(SPEC_ROOT)
                       if os.path.isdir(os.path.join(SPEC_ROOT, d)))
    results = [lint(n, diff_old) for n in names]
    sys.exit(0 if all(results) else 1)


if __name__ == "__main__":
    main()
