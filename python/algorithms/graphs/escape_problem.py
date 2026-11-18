"""The Escape Problem — CLRS Problem 26-1 (1e–4e).

Can m starting points on an n×n grid escape to the boundary along vertex-disjoint paths? Maximum flow with unit vertex capacities.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class _FlowNetwork:
    def __init__(self, n: int) -> None:
        self.n = n
        self.to: list[int] = []
        self.cap: list[int] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def add_edge(self, u: int, v: int, c: int) -> int:
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(c)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.adj[v].append(eid + 1)
        return eid

    def edmonds_karp(self, s: int, t: int) -> int:
        flow = 0
        while True:
            pred = [-1] * self.n
            seen = [False] * self.n
            seen[s] = True
            q: deque[int] = deque([s])
            while q:
                u = q.popleft()
                for e in self.adj[u]:
                    v = self.to[e]
                    if not seen[v] and self.cap[e] > 0:
                        seen[v] = True
                        pred[v] = e
                        q.append(v)
            if not seen[t]:
                return flow
            b = 1 << 60
            v = t
            while v != s:
                e = pred[v]
                if self.cap[e] < b:
                    b = self.cap[e]
                v = self.to[e ^ 1]
            v = t
            while v != s:
                e = pred[v]
                self.cap[e] -= b
                self.cap[e ^ 1] += b
                v = self.to[e ^ 1]
            flow += b


def max_escapees(n: int, starts: list[tuple[int, int]]) -> int:
    """Maximum number of starting points that can escape to the boundary along pairwise vertex-disjoint paths in an n×n grid."""
    def vid(r: int, c: int) -> int:
        return r * n + c

    s, t = 2 * n * n, 2 * n * n + 1
    g = _FlowNetwork(2 * n * n + 2)
    for r in range(n):
        for c in range(n):
            v = vid(r, c)
            g.add_edge(2 * v, 2 * v + 1, 1)
            if r == 0 or c == 0 or r == n - 1 or c == n - 1:
                g.add_edge(2 * v + 1, t, 1)
            for dr, dc in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                nr, nc = r + dr, c + dc
                if 0 <= nr < n and 0 <= nc < n:
                    g.add_edge(2 * v + 1, 2 * vid(nr, nc), 1)
    for r, c in starts:
        g.add_edge(s, 2 * vid(r, c), 1)
    return g.edmonds_karp(s, t)


def can_escape(n: int, starts: list[tuple[int, int]]) -> bool:
    """Is escape possible for all starting points?"""
    return max_escapees(n, starts) == len(starts)


def demo() -> str:
    lines: list[str] = []
    pts = [(1, 1), (1, 3), (3, 1), (3, 3)]
    lines.append(f"5×5 grid, 4 starting points in the middle: all escape? {can_escape(5, pts)} (flow {max_escapees(5, pts)})")
    crowd = [(r, c) for r in range(1, 4) for c in range(1, 4)]
    lines.append(
        f"5×5 grid, all 9 interior vertices occupied: escapees {max_escapees(5, crowd)} of {len(crowd)} "
        "(the 3×3 block has only 12 neighbours but every path must cross its own ring)"
    )
    lines.append(f"single start in the centre of a 3×3 grid: {can_escape(3, [(1, 1)])}")
    return "\n".join(lines) + "\n"


def _brute(n: int, starts: list[tuple[int, int]], i: int, used: list[bool]) -> int:
    if i == len(starts):
        return 0
    best = _brute(n, starts, i + 1, used)

    def dfs(r: int, c: int) -> None:
        nonlocal best
        used[r * n + c] = True
        if r == 0 or c == 0 or r == n - 1 or c == n - 1:
            cand = 1 + _brute(n, starts, i + 1, used)
            if cand > best:
                best = cand
        else:
            for dr, dc in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                nr, nc = r + dr, c + dc
                if not used[nr * n + nc]:
                    dfs(nr, nc)
        used[r * n + c] = False

    sr, sc = starts[i]
    if not used[sr * n + sc]:
        dfs(sr, sc)
    return best


class Tests(unittest.TestCase):
    def test_matches_exhaustive_path_search(self) -> None:
        r = Rng(370)
        for _ in range(60):
            n = r.range(2, 4)
            cells = [(a, b) for a in range(n) for b in range(n)]
            r.shuffle(cells)
            cells = cells[: r.range(0, min(4, n * n))]
            self.assertEqual(max_escapees(n, cells), _brute(n, cells, 0, [False] * (n * n)), f"n={n} {cells}")

    def test_known_cases(self) -> None:
        self.assertTrue(can_escape(3, [(1, 1)]))
        self.assertTrue(can_escape(5, [(1, 1), (1, 3), (3, 1), (3, 3)]))
        self.assertEqual(max_escapees(3, [(1, 1), (0, 0)]), 2)
        crowd = [(r, c) for r in range(1, 4) for c in range(1, 4)]
        self.assertLess(max_escapees(5, crowd), 9)


if __name__ == "__main__":
    print(demo(), end="")
