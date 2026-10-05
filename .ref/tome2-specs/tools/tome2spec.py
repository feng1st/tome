#!/usr/bin/env python3
"""tome2-spec: inventory, ledger and completeness checking for the tome2 analysis.

The analysis of .ref/tome2 spans multiple sessions. This tool keeps the work
state on disk (.ref/tome2-specs/state.json) so any session can resume without
re-reading content that was already analyzed:

- scan   walk the source tree, classify every file, hash it, merge into state
- next   list pending analyzable items (the analysis work queue)
- show   print the state of items whose path matches a pattern
- mark   record a file as fully analyzed, with the spec files it produced
- skip   record a file as deliberately excluded, with a reason
- check  verify completeness: nothing pending, hashes unchanged, spec refs exist
- report regenerate the human-readable 00-inventory.md and 00-ledger.md

Design invariants:
- Nothing may be read for analysis twice: `mark` refuses to mark an item done
  twice for the same content hash (repeated-analysis guard).
- Nothing may be lost: `check` fails while any analyzable item is pending or
  any source file is unclassified, so completion is machine-verifiable.
"""

import argparse
import hashlib
import json
import re
import subprocess
import sys
from datetime import date
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
SOURCE_ROOT = REPO / ".ref" / "tome2"
SPEC_ROOT = REPO / ".ref" / "tome2-specs"
STATE_PATH = SPEC_ROOT / "state.json"

# Lower rank = analyzed earlier in the queue. Data vocabularies first (they
# enumerate WHAT exists), then engine C (HOW it behaves), then Lua bindings
# and finally Lua content scripts.
CLASS_RANK = {"edit": 0, "c": 1, "h": 2, "pkg": 3, "lua": 4}
CLASS_LABEL = {
    "edit": "数据词表 (lib/edit/*.txt)",
    "c": "引擎源码 (src/*.c)",
    "h": "引擎头文件 (src/*.h)",
    "pkg": "Lua 绑定声明 (src/*.pkg)",
    "lua": "Lua 脚本 (lib/{core,scpt,mods})",
}

# Ordered rules; first match wins. Each rule yields status=skipped with the
# given reason. Anything not matched and not analyzable below becomes
# "unclassified" and makes `check` fail until it is decided by hand.
SKIP_RULES = [
    (r"^\.git/|^\.gitignore$", "版本控制内容"),
    (r"^src/main-|^src/maid-|^src/maim-|^src/iso/", "平台移植与平台前端主循环，机制不随平台"),
    (r"^src/carbon/", "平台资源（Mac Carbon 图标与清单）"),
    (r"^src/makefile|^src/\.#|^src/\.gitignore|^src/ENGLISH\.txt$|\.orig$|\.rej$|\.pch$|\.rc$|\.ico$",
     "构建脚本、版本控制残留或平台资源"),
    (r"^src/(config\.h|h-.+\.h)$", "系统兼容层与构建开关，无游戏机制"),
    (r"^src/(externs|angband)\.h$", "纯声明清单，语义随对应 .c 分析"),
    (r"^src/(irc|cmovie|help|notes|load_gif|readdib)\.(c|h)$",
     "与游戏机制无关的杂项：聊天、演示录制、帮助、笔记输出、GIF/位图解码"),
    (r"^src/z-(form|util|virt|sock)\.(c|h)$", "通用支撑层（格式化/内存/套接字/报错），无游戏机制"),
    (r"^src/z-term\.(c|h)$", "字符终端抽象层；显示由 tome 的 Bevy 前端另行实现"),
    (r"^src/lua/|^src/lauxlib\.h$", "tolua 绑定生成器脚本与嵌入式 Lua 解释器（构建期工具与第三方运行时，非游戏机制）"),
    (r"^lib/xtra/", "音乐/字体/音效/图形资源"),
    (r"^lib/pref/", "前端展示配置（字体/图形/颜色绑定）"),
    (r"^lib/(file|help)/", "帮助与展示文本（法术书文本亦属展示；法术机制由词表与 Lua 覆盖）"),
    (r"^lib/(apex|dngn|save|note|user|info|patch|bone|data|cmov)/", "运行期产物或占位目录"),
    (r"^lib/edit/[^/]+\.map$", "预制关卡布局内容数据，非机制"),
    (r"^(changes\.txt|changes\.old|credits\.txt|angdos\.cfg|tome\.ini)$", "发布说明与平台配置文件"),
]


def classify(relpath: str):
    """Return (class, skip_reason) for a path relative to the source root.

    class is one of the analyzable classes, or "skip" when a rule excluded
    the file, or "unclassified" when no rule matched and the caller must
    decide by hand.
    """
    for pattern, reason in SKIP_RULES:
        if re.search(pattern, relpath):
            return "skip", reason
    if re.fullmatch(r"lib/edit/[^/]+\.txt", relpath):
        return "edit", ""
    if relpath.startswith("src/") and relpath.endswith(".pkg"):
        return "pkg", ""
    if re.fullmatch(r"lib/(core|scpt|mods)/[^/]+\.lua|lib/module\.lua", relpath):
        return "lua", ""
    if relpath.startswith("src/") and relpath.endswith(".c"):
        return "c", ""
    if relpath.startswith("src/") and relpath.endswith(".h"):
        return "h", ""
    return "unclassified", ""


def load_state():
    if STATE_PATH.exists():
        return json.loads(STATE_PATH.read_text(encoding="utf-8"))
    return {"scanned_at": "", "items": {}}


def save_state(state):
    STATE_PATH.write_text(
        json.dumps(state, ensure_ascii=False, indent=1, sort_keys=True),
        encoding="utf-8",
    )


def hash_file(path: Path):
    digest = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 16), b""):
            digest.update(chunk)
    return digest.hexdigest()


def collect_source_files():
    """Walk the source tree; every file must end up in the state, so hidden
    debris and binary assets are included too (they get skipped by rules)."""
    for path in sorted(SOURCE_ROOT.rglob("*")):
        if path.is_file():
            yield path.relative_to(SOURCE_ROOT).as_posix()


def cmd_scan(_args):
    state = load_state()
    items = state.get("items", {})
    seen = set()
    for relpath in collect_source_files():
        seen.add(relpath)
        path = SOURCE_ROOT / relpath
        sha = hash_file(path)
        klass, reason = classify(relpath)
        old = items.get(relpath)
        if old and old.get("sha256") == sha:
            # Content unchanged: keep every recorded decision untouched.
            old["class"] = klass
            if klass == "skip":
                old["status"] = "skipped"
                old["reason"] = reason
            continue
        if klass == "skip":
            items[relpath] = {
                "class": "skip", "status": "skipped", "reason": reason,
                "sha256": sha, "bytes": path.stat().st_size,
            }
        elif klass == "unclassified":
            items[relpath] = {
                "class": "unclassified", "status": "pending",
                "sha256": sha, "bytes": path.stat().st_size,
            }
        else:
            entry = {
                "class": klass, "status": "pending", "sha256": sha,
                "bytes": path.stat().st_size, "lines": line_count(path),
                "specs": [], "note": "",
            }
            if old:
                # Hash changed: previous analysis no longer covers this file.
                entry["note"] = "源文件内容变化，旧分析作废，需重新分析"
            items[relpath] = entry
    for relpath in set(items) - seen:
        del items[relpath]
    state["items"] = items
    state["scanned_at"] = str(date.today())
    save_state(state)
    counts = {}
    for item in items.values():
        key = (item["class"], item["status"])
        counts[key] = counts.get(key, 0) + 1
    print(f"scanned {len(items)} files under .ref/tome2")
    for (klass, status), n in sorted(counts.items()):
        print(f"  {klass:<13} {status:<8} {n}")


def line_count(path: Path) -> int:
    with path.open("rb") as fh:
        return sum(chunk.count(b"\n") for chunk in iter(lambda: fh.read(1 << 20), b""))


def cmd_next(args):
    state = load_state()
    pending = [
        (CLASS_RANK.get(i["class"], 9), p, i)
        for p, i in state["items"].items()
        if i["status"] == "pending" and i["class"] in CLASS_RANK
    ]
    pending.sort()
    for _rank, path, item in pending[: args.limit]:
        print(f"{path}  ({item['lines']} lines, {item['class']})")
    print(f"-- {len(pending)} pending items total")


def cmd_show(args):
    state = load_state()
    for path, item in sorted(state["items"].items()):
        if re.search(args.pattern, path):
            print(f"{path}\n  class={item['class']} status={item['status']} "
                  f"sha256={item['sha256'][:12]}")
            if item.get("reason"):
                print(f"  reason: {item['reason']}")
            if item.get("note"):
                print(f"  note: {item['note']}")
            if item.get("specs"):
                print(f"  specs: {', '.join(item['specs'])}")


def cmd_mark(args):
    state = load_state()
    item = state["items"].get(args.path)
    if item is None:
        sys.exit(f"error: {args.path} not in state; run scan first")
    if item["class"] not in CLASS_RANK:
        sys.exit(f"error: {args.path} is not analyzable (class={item['class']})")
    for spec in args.spec:
        if not (SPEC_ROOT / spec).is_file():
            sys.exit(f"error: spec file {spec} does not exist under {SPEC_ROOT}")
    if item["status"] == "done" and not args.force:
        sys.exit(
            f"error: {args.path} is already done for this content hash "
            f"(repeated analysis guard; use --force only to add spec refs)"
        )
    item["status"] = "done"
    item["marked_at"] = str(date.today())
    for spec in args.spec:
        if spec not in item["specs"]:
            item["specs"].append(spec)
    if args.note:
        item["note"] = args.note
    save_state(state)
    cmd_report(argparse.Namespace())
    print(f"marked done: {args.path} -> {', '.join(item['specs'])}")


def cmd_skip(args):
    state = load_state()
    item = state["items"].get(args.path)
    if item is None:
        sys.exit(f"error: {args.path} not in state; run scan first")
    item["status"] = "skipped"
    item["reason"] = args.reason
    save_state(state)
    cmd_report(argparse.Namespace())
    print(f"skipped: {args.path} ({args.reason})")


def cmd_check(_args):
    state = load_state()
    problems = []
    counts = {"analyzable": 0, "done": 0, "skipped": 0, "pending": 0,
              "unclassified": 0}
    for path, item in state["items"].items():
        if item["class"] == "unclassified":
            counts["unclassified"] += 1
            problems.append(f"unclassified: {path}")
            continue
        if item["class"] == "skip":
            counts["skipped"] += 1
            if not item.get("reason"):
                problems.append(f"skipped without reason: {path}")
            continue
        counts["analyzable"] += 1
        if item["status"] == "done":
            counts["done"] += 1
            if not item.get("specs"):
                problems.append(f"done without spec ref: {path}")
            for spec in item.get("specs", []):
                if not (SPEC_ROOT / spec).is_file():
                    problems.append(f"missing spec file {spec} (from {path})")
            if hash_file(SOURCE_ROOT / path) != item["sha256"]:
                problems.append(f"source drift after analysis: {path}")
        elif item["status"] == "skipped":
            counts["skipped"] += 1
            if not item.get("reason"):
                problems.append(f"skipped without reason: {path}")
        else:
            counts["pending"] += 1
    print(f"analyzable={counts['analyzable']} done={counts['done']} "
          f"skipped={counts['skipped']} pending={counts['pending']} "
          f"unclassified={counts['unclassified']}")
    if problems:
        for problem in problems[: args.limit]:
            print(f"  PROBLEM: {problem}")
        if len(problems) > args.limit:
            print(f"  ... and {len(problems) - args.limit} more")
        sys.exit(1)
    if counts["pending"] or counts["unclassified"]:
        print("FAIL: analysis incomplete")
        sys.exit(1)
    print("PASS: analysis complete")


def cmd_report(_args):
    state = load_state()
    items = state["items"]

    def render(path_filter):
        rows = []
        for path, item in sorted(items.items()):
            if item["class"] != path_filter:
                continue
            if item["status"] == "skipped":
                continue
            specs = ", ".join(item.get("specs", [])) or "-"
            rows.append(f"| {path} | {item.get('lines', '-')} | "
                        f"{item['status']} | {specs} |")
        return rows

    lines = [
        "# tome2 分析清单",
        "",
        f"扫描日期: {state['scanned_at']}；状态与指纹的唯一事实来源是 "
        "state.json，本文件由 tools/tome2spec.py report 生成，勿手改。",
        "",
        "完成判定: `python3 tools/tome2spec.py check` 输出 PASS 即分析完成",
        "（不存在待分析、不存在未分类、哈希无漂移、spec 引用齐备）。",
        "",
    ]
    for klass, label in CLASS_LABEL.items():
        rows = render(klass)
        lines += [f"## {label}", "",
                  "| 文件 | 行数 | 状态 | 产物 spec |", "|---|---|---|---|"]
        lines += rows or ["| (空) | | | |"]
        lines.append("")
    lines += ["## 跳过项", "",
              "以下文件不进入分析；每项必须带理由，理由缺失会被 check 判失败。",
              "", "| 文件 | 理由 |", "|---|---|"]
    for path, item in sorted(items.items()):
        if item["status"] == "skipped":
            lines.append(f"| {path} | {item.get('reason', '')} |")
    lines.append("")
    (SPEC_ROOT / "00-inventory.md").write_text(
        "\n".join(lines), encoding="utf-8")

    ledger = [
        "# tome2 分析账本",
        "",
        "已完成分析的内容登记于此，由 tools/tome2spec.py report 生成。",
        "再次进入分析前先查本表（或用 show 按路径查询），命中即跳过，不重读。",
        "",
        "| 完成日期 | 源文件 | 产物 spec | 备注 |", "|---|---|---|---|",
    ]
    for path, item in sorted(items.items()):
        if item["status"] == "done":
            ledger.append(
                f"| {item.get('marked_at', '')} | {path} | "
                f"{', '.join(item.get('specs', []))} | {item.get('note', '')} |")
    ledger.append("")
    (SPEC_ROOT / "00-ledger.md").write_text(
        "\n".join(ledger), encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="cmd", required=True)

    sub.add_parser("scan", help="walk source tree, classify and hash files")
    p = sub.add_parser("next", help="list pending analyzable items")
    p.add_argument("--limit", type=int, default=20)
    p = sub.add_parser("show", help="show state of matching items")
    p.add_argument("pattern")
    p = sub.add_parser("mark", help="record a file as fully analyzed")
    p.add_argument("path")
    p.add_argument("--spec", action="append", required=True,
                   help="spec file produced (relative to tome2-specs), repeatable")
    p.add_argument("--note", default="")
    p.add_argument("--force", action="store_true",
                   help="allow re-marking done items (e.g. to add spec refs)")
    p = sub.add_parser("skip", help="record a file as excluded")
    p.add_argument("path")
    p.add_argument("--reason", required=True)
    p = sub.add_parser("check", help="verify analysis completeness")
    p.add_argument("--limit", type=int, default=30)
    sub.add_parser("report", help="regenerate 00-inventory.md / 00-ledger.md")

    args = parser.parse_args()
    {"scan": cmd_scan, "next": cmd_next, "show": cmd_show, "mark": cmd_mark,
     "skip": cmd_skip, "check": cmd_check, "report": cmd_report}[args.cmd](args)


if __name__ == "__main__":
    main()
