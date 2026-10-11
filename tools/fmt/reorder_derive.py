import re
import sys
from pathlib import Path

TIERS = [
    ["Component", "Resource", "Message", "SystemSet", "States", "QueryData"],
    ["Clone", "Copy", "PartialEq", "Eq", "PartialOrd", "Ord", "Hash"],
    ["Debug", "Default"],
    ["Serialize", "Deserialize"],
]

KEY = {name: (tier, index) for tier, names in enumerate(TIERS) for index, name in enumerate(names)}

PATTERN = re.compile(r"#\[derive\(([^)]*)\)\]")


def mask(source):
    out = list(source)
    i = 0
    n = len(source)
    while i < n:
        if source.startswith("//", i):
            j = source.find("\n", i)
            j = n if j < 0 else j
            out[i:j] = " " * (j - i)
            i = j
        elif source.startswith("/*", i):
            depth = 1
            j = i + 2
            while j < n and depth:
                if source.startswith("/*", j):
                    depth += 1
                    j += 2
                elif source.startswith("*/", j):
                    depth -= 1
                    j += 2
                else:
                    j += 1
            out[i:j] = " " * (j - i)
            i = j
        elif source.startswith('r"', i) or source.startswith("r#", i):
            j = i + 1
            hashes = 0
            while j < n and source[j] == "#":
                hashes += 1
                j += 1
            if j < n and source[j] == '"':
                closing = '"' + "#" * hashes
                k = source.find(closing, j + 1)
                k = n if k < 0 else k + len(closing)
                out[i:k] = " " * (k - i)
                i = k
            else:
                i += 1
        elif source[i] == '"':
            j = i + 1
            while j < n:
                if source[j] == "\\":
                    j += 2
                elif source[j] == '"':
                    j += 1
                    break
                else:
                    j += 1
            out[i:j] = " " * (j - i)
            i = j
        elif source[i] == "'" and i + 2 < n:
            if source[i + 1] == "\\" and i + 3 < n and source[i + 3] == "'":
                out[i : i + 4] = " " * 4
                i += 4
            elif source[i + 2] == "'":
                out[i : i + 3] = " " * 3
                i += 3
            else:
                i += 1
        else:
            i += 1
    return "".join(out)


def derive_names(match):
    return [name.strip() for name in match.group(1).split(",") if name.strip()]


def main():
    root = Path(__file__).resolve().parents[2]
    sources = {path: path.read_text() for path in sorted((root / "src").rglob("*.rs"))}
    masked = {path: mask(source) for path, source in sources.items()}

    unknown = []
    for path, text in masked.items():
        for match in PATTERN.finditer(text):
            for name in derive_names(match):
                if name not in KEY:
                    line = sources[path].count("\n", 0, match.start()) + 1
                    unknown.append(f"{path}:{line}: unknown derive {name}")
    if unknown:
        print("\n".join(unknown))
        sys.exit(1)

    changed = 0
    for path, source in sources.items():
        edits = []
        for match in PATTERN.finditer(masked[path]):
            names = derive_names(match)
            ordered = sorted(names, key=KEY.get)
            if ordered != names:
                edits.append((match.start(), match.end(), "#[derive(" + ", ".join(ordered) + ")]"))
        if not edits:
            continue
        rewritten = source
        for start, end, replacement in reversed(edits):
            rewritten = rewritten[:start] + replacement + rewritten[end:]
        path.write_text(rewritten)
        changed += len(edits)
    print(f"reordered {changed} derive blocks")


if __name__ == "__main__":
    main()
