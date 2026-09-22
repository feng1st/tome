#!/usr/bin/env python3
"""codegraph: SCIP index -> review rules -> mermaid markdown 架构图.

Usage:
  codegraph.py index                                   # 生成索引（调用 rust-analyzer，产物进 .codegraph/）
  codegraph.py arch --config codegraph.toml            # 生成 architecture/（mermaid md）
"""
import argparse
import json
import shutil
import subprocess
import sys
import tomllib
from collections import defaultdict
from pathlib import Path

WORK_DIR = Path(".codegraph")

ROLE_DEFINITION = 0x1

KIND = {
    7: "class", 8: "constant", 11: "enum", 12: "enum_member", 15: "field",
    16: "file", 17: "function", 21: "interface", 25: "macro", 26: "method",
    29: "module", 30: "namespace", 33: "object", 49: "struct", 53: "trait",
    54: "type", 55: "type_alias", 61: "variable", 70: "trait_method",
    80: "static_method", 82: "static_variable",
}
REVIEWABLE = {"function", "method", "trait_method", "struct", "enum", "trait",
              "constant", "type_alias", "module", "type"}
STD_TRAIT_METHODS = {
    "partial_cmp", "cmp", "eq", "ne", "hash", "clone", "fmt", "default", "from",
    "into", "drop", "build", "next", "len", "is_empty", "deref", "deref_mut",
    "borrow", "borrow_mut", "as_ref", "as_mut", "index", "index_mut", "add",
    "sub", "mul", "div", "rem", "neg", "not", "deserialize", "serialize",
}


def norm_range(v):
    if not v:
        return None
    if len(v) == 3:
        return (v[0], v[1], v[0], v[2])
    if len(v) == 4:
        return tuple(v)
    return None


def contains(a, b):
    return (b[0], b[1]) >= (a[0], a[1]) and (b[2], b[3]) <= (a[2], a[3])


def short_name(symbol):
    for sep in ("#", "/", "(", ")", "."):
        symbol = symbol.split(sep)[-1] if sep in symbol else symbol
    return symbol or "?"


def load_scip(index_path):
    out = subprocess.run(
        ["scip", "print", "--json", str(index_path)],
        capture_output=True, text=True, check=True,
    ).stdout
    return json.loads(out)


def build_graph(index):
    symbols = {}  # id -> dict
    edges = defaultdict(int)  # (from_sym, to_sym) -> refs

    internal = set()
    external = set()
    for doc in index["documents"]:
        for occ in doc["occurrences"]:
            sym = occ["symbol"]
            if occ.get("symbol_roles", 0) & ROLE_DEFINITION and not sym.startswith("local "):
                internal.add(sym)
    for ext in index.get("external_symbols", []):
        external.add(ext["symbol"])

    for doc in index["documents"]:
        path = doc["relative_path"]
        info = {s["symbol"]: s for s in doc.get("symbols", [])}
        defs = []
        for occ in doc["occurrences"]:
            sym = occ["symbol"]
            if occ.get("symbol_roles", 0) & ROLE_DEFINITION and not sym.startswith("local "):
                r = norm_range(occ.get("range"))
                if r:
                    defs.append((r, sym, norm_range(occ.get("enclosing_range"))))
        defs.sort()

        def_range = {r: sym for r, sym, _ in defs}
        # mod tests 的 def occurrence 只有名字 token，但其 enclosing_range 是整个 mod 项
        test_item_ranges = [
            er for r, sym, er in defs
            if er is not None
            and (info.get(sym, {}).get("display_name") or short_name(sym)) == "tests"
            and info.get(sym, {}).get("kind") == 29
        ]

        for r, sym, er in defs:
            si = info.get(sym, {})
            name = si.get("display_name") or short_name(sym)
            is_test_mod = name == "tests" and si.get("kind") == 29
            symbols[sym] = {
                "id": sym,
                "name": name,
                "kind": KIND.get(si.get("kind", 0), "other"),
                "file": path,
                "line": r[0] + 1,
                "in_test": is_test_mod or any(contains(tr, r) for tr in test_item_ranges),
                "impl_of_external": any(
                    rel.get("is_implementation") and rel["symbol"] in external
                    for rel in si.get("relationships", [])
                ),
            }
        for occ in doc["occurrences"]:
            sym = occ["symbol"]
            if (occ.get("symbol_roles", 0) & ROLE_DEFINITION
                    or sym.startswith("local ") or sym not in internal
                    or sym.endswith(" crate/")):
                continue
            r = norm_range(occ.get("range"))
            if not r:
                continue
            er = norm_range(occ.get("enclosing_range"))
            src = def_range.get(er) if er else None
            if src is None:
                cands = [(dr, ds) for dr, ds, _ in defs if contains(dr, r)]
                src = min(cands, key=lambda d: (d[0][2] - d[0][0], d[0][3] - d[0][1]))[1] if cands else None
            if src and src != sym:
                edges[(src, sym)] += 1

    file_edges = defaultdict(int)
    sym_file = {s: v["file"] for s, v in symbols.items()}
    for (a, b), n in edges.items():
        fa, fb = sym_file.get(a), sym_file.get(b)
        if fa and fb and fa != fb:
            file_edges[(fa, fb)] += n

    return {
        "symbols": list(symbols.values()),
        "edges": [{"from": a, "to": b, "refs": n} for (a, b), n in sorted(edges.items())],
        "file_edges": [{"from": a, "to": b, "refs": n} for (a, b), n in sorted(file_edges.items())],
    }


def layer_of(path, layers):
    best = None
    for l in layers:
        if path.startswith(l["path"]) and (best is None or len(l["path"]) > len(best["path"])):
            best = l
    return best


def review(graph, config):
    layers = config.get("layers", [])
    kind = {s["id"]: s["kind"] for s in graph["symbols"]}
    sym_file = {s["id"]: s["file"] for s in graph["symbols"]}

    # 逻辑依赖边：排除指向模块符号的边（use 路径段 / pub mod 声明是结构噪音）
    logic_edges = [e for e in graph["edges"] if kind.get(e["to"]) != "module"]
    logic_file_refs = defaultdict(int)
    for e in logic_edges:
        fa, fb = sym_file.get(e["from"]), sym_file.get(e["to"])
        if fa and fb and fa != fb:
            logic_file_refs[(fa, fb)] += e["refs"]
    logic_file_edges = [{"from": a, "to": b, "refs": n}
                        for (a, b), n in sorted(logic_file_refs.items())]

    violations = []
    for e in logic_file_edges:
        fl, tl = layer_of(e["from"], layers), layer_of(e["to"], layers)
        if not fl or not tl or fl["name"] == tl["name"]:
            continue
        if tl["name"] not in fl.get("allow", []):
            violations.append({
                "from_file": e["from"], "to_file": e["to"],
                "from_layer": fl["name"], "to_layer": tl["name"], "refs": e["refs"],
            })

    referenced = {e["to"] for e in graph["edges"]}
    dead = [s for s in graph["symbols"]
            if s["kind"] in REVIEWABLE and s["id"] not in referenced
            and not s["impl_of_external"] and not s.get("in_test")
            and not s["id"].endswith(" crate/")
            and s["name"] != "main" and s["name"] not in STD_TRAIT_METHODS
            and "/tests/" not in s["file"]]
    dead.sort(key=lambda s: (s["file"], s["line"]))

    # cycle detection on file graph (Tarjan SCC, iterative)
    adj = defaultdict(set)
    for e in logic_file_edges:
        adj[e["from"]].add(e["to"])
    index_of, low, stack, on_stack = {}, {}, [], set()
    counter = [0]
    cycle_files = set()

    def strongconnect(start):
        work = [(start, iter(sorted(adj.get(start, ()))))]
        index_of[start] = low[start] = counter[0]
        counter[0] += 1
        stack.append(start)
        on_stack.add(start)
        while work:
            node, it = work[-1]
            advanced = False
            for nxt in it:
                if nxt not in index_of:
                    index_of[nxt] = low[nxt] = counter[0]
                    counter[0] += 1
                    stack.append(nxt)
                    on_stack.add(nxt)
                    work.append((nxt, iter(sorted(adj.get(nxt, ())))))
                    advanced = True
                    break
                elif nxt in on_stack:
                    low[node] = min(low[node], index_of[nxt])
            if advanced:
                continue
            work.pop()
            if work:
                parent = work[-1][0]
                low[parent] = min(low[parent], low[node])
            if low[node] == index_of[node]:
                scc = []
                while True:
                    w = stack.pop()
                    on_stack.discard(w)
                    scc.append(w)
                    if w == node:
                        break
                if len(scc) > 1:
                    cycle_files.update(scc)

    for v in list(adj):
        if v not in index_of:
            strongconnect(v)

    cycle_edges = [(e["from"], e["to"]) for e in logic_file_edges
                   if e["from"] in cycle_files and e["to"] in cycle_files]
    return {
        "violations": violations,
        "dead": dead,
        "cycle_files": sorted(cycle_files),
        "cycle_file_edges": cycle_edges,
    }



def logic_file_refs(graph):
    """符号级逻辑依赖聚合到文件级（排除指向模块符号的结构性边）。"""
    kind = {s["id"]: s["kind"] for s in graph["symbols"]}
    sym_file = {s["id"]: s["file"] for s in graph["symbols"]}
    counts = defaultdict(int)
    for e in graph["edges"]:
        if kind.get(e["to"]) == "module":
            continue
        fa, fb = sym_file.get(e["from"]), sym_file.get(e["to"])
        if fa and fb and fa != fb:
            counts[(fa, fb)] += e["refs"]
    return counts


def module_children(prefix, files):
    """返回 prefix 目录的直接子节点：[(名称, 完整前缀, 是否目录)]，按名称排序。"""
    seen = {}
    for f in files:
        rest = f[len(prefix) + 1:] if f.startswith(prefix + "/") else None
        if not rest:
            continue
        seg, _, remainder = rest.partition("/")
        if remainder:
            seen.setdefault(seg, (seg, f"{prefix}/{seg}", True))
        else:
            seen.setdefault(seg, (seg, f"{prefix}/{seg}", False))
    return sorted(seen.values())


def mermaid_diagram(name, prefix, children, refs):
    """为一个模块生成 mermaid 图：子系统内部依赖 + 对外的直接依赖。"""
    child_depth = prefix.count("/") + 1
    node_ids = {}
    nodes = []     # (id, label, is_external)
    edges = defaultdict(int)  # (src_id, dst_id) -> refs

    def child_of(path):
        for cname, cprefix, _ in children:
            if path == cprefix or path.startswith(cprefix + "/"):
                return cprefix
        return None

    def node_id(key, label, external):
        if key not in node_ids:
            nid = f"n{len(node_ids)}"
            node_ids[key] = nid
            nodes.append((nid, label, external))
        return node_ids[key]

    for (fa, fb), n in refs.items():
        ca, cb = child_of(fa), child_of(fb)
        if ca and cb:
            if ca == cb:
                continue
            a = node_id(ca, ca.split("/")[-1], False)
            b = node_id(cb, cb.split("/")[-1], False)
            edges[(a, b)] += n
        elif ca:
            ext_key = "/".join(fb.split("/")[:child_depth + 1])
            a = node_id(ca, ca.split("/")[-1], False)
            b = node_id(("ext", ext_key), ext_key[len("src/"):] + "〔外部〕", True)
            edges[(a, b)] += n
        elif cb:
            ext_key = "/".join(fa.split("/")[:child_depth + 1])
            a = node_id(("ext", ext_key), ext_key[len("src/"):] + "〔外部〕", True)
            b = node_id(cb, cb.split("/")[-1], False)
            edges[(a, b)] += n

    if len(nodes) < 2:
        return None

    lines = ["flowchart LR"]
    for nid, label, external in nodes:
        safe = label.replace('"', "'")
        lines.append(f'  {nid}["{safe}"]')
    for (a, b), n in sorted(edges.items()):
        lines.append(f"  {a} -->|{n}| {b}")
    ext_ids = [nid for nid, _, external in nodes if external]
    if ext_ids:
        lines.append("  classDef external fill:#3a3a3a,stroke:#888,stroke-dasharray:5 5,color:#bbb;")
        lines.append("  class " + ",".join(ext_ids) + " external;")
    return "\n".join(lines)


def arch(graph, out_dir, config):
    refs = logic_file_refs(graph)
    files = sorted({s["file"] for s in graph["symbols"]})
    out = Path(out_dir)
    out.mkdir(parents=True, exist_ok=True)

    diagrams = []  # (标题, 文件名, mermaid 源码)

    def gen(name, prefix):
        children = module_children(prefix, files)
        if len(children) < 3:
            return
        src = mermaid_diagram(name, prefix, children, refs)
        if not src:
            return
        fname = "overview" if prefix == "src" else prefix[len("src/"):].replace("/", "__")
        (out / f"{fname}.mmd").write_text(src + "\n")
        diagrams.append((name, fname, src))
        for cname, cprefix, is_dir in children:
            if is_dir:
                gen(cprefix[len("src/"):].replace("/", " / "), cprefix)

    gen("总览", "src")

    md = ["# 架构依赖图", "",
          "由 `codegraph.py arch` 自动生成（数据源：rust-analyzer SCIP 索引），",
          "边为符号级引用聚合，标注数字为引用次数；虚线灰框为外部直接依赖。", ""]

    if config:
        result = review(graph, config)
        md += ["## Review 结果", ""]
        md.append(f"### 层级违规引用（{len(result['violations'])}）")
        md.append("")
        if result["violations"]:
            for v in result["violations"]:
                md.append(f"- **{v['from_layer']} → {v['to_layer']}**："
                          f"`{v['from_file']}` → `{v['to_file']}`（{v['refs']} 次引用）")
        else:
            md.append("无")
        md.append("")
        md.append(f"### 循环依赖文件（{len(result['cycle_files'])}）")
        md.append("")
        if result["cycle_files"]:
            for f in result["cycle_files"]:
                md.append(f"- `{f}`")
        else:
            md.append("无")
        md.append("")
        md.append(f"### 疑似死代码（{len(result['dead'])}）")
        md.append("")
        if result["dead"]:
            for d in result["dead"]:
                md.append(f"- `{d['file']}:{d['line']}` {d['kind']} `{d['name']}`")
        else:
            md.append("无")
        md.append("")
        print(f"review: {len(result['violations'])} violations, "
              f"{len(result['cycle_files'])} cycle files, "
              f"{len(result['dead'])} dead candidates", file=sys.stderr)

    for title, fname, src in diagrams:
        md += [f"## {title}", "", "```mermaid", src, "```", ""]
    (out / "README.md").write_text("\n".join(md))
    print(f"arch: {len(diagrams)} diagrams -> {out_dir}/README.md", file=sys.stderr)


def make_index():
    WORK_DIR.mkdir(exist_ok=True)
    subprocess.run(["rust-analyzer", "scip", "."], check=True)
    shutil.move("index.scip", WORK_DIR / "index.scip")
    print(f"wrote {WORK_DIR}/index.scip", file=sys.stderr)


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)

    sub.add_parser("index")

    ar = sub.add_parser("arch")
    ar.add_argument("--index", default=str(WORK_DIR / "index.scip"))
    ar.add_argument("--out-dir", default="architecture")
    ar.add_argument("--config")

    args = ap.parse_args()
    if args.cmd == "index":
        make_index()
        return
    index = load_scip(args.index)
    graph = build_graph(index)
    print(f"graph: {len(graph['symbols'])} symbols, {len(graph['edges'])} edges, "
          f"{len(graph['file_edges'])} file edges", file=sys.stderr)

    if args.cmd == "arch":
        config = tomllib.loads(Path(args.config).read_text()) if args.config else None
        arch(graph, args.out_dir, config)


if __name__ == "__main__":
    main()
