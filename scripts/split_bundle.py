#!/usr/bin/env python3
"""Authoring helper: split a bundle (`=== FILE path ===` markers) into repo files."""
import sys, pathlib
root = pathlib.Path(sys.argv[2] if len(sys.argv) > 2 else ".")
cur, buf, n = None, [], 0
def flush():
    global n
    if cur:
        p = root / cur; p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text("".join(buf).strip("\n") + "\n"); n += 1
for line in open(sys.argv[1]):
    if line.startswith("=== FILE ") and line.rstrip().endswith("==="):
        flush(); cur = line[9:].rstrip()[:-3].strip(); buf = []
    else:
        buf.append(line)
flush(); print(f"wrote {n} files")
