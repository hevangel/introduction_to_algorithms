"""Minimum Mean-Weight Cycle (Karp) — CLRS Problem 24-5 (2e–3e) / 22-5 (4e).

Karp's O(VE) algorithm: min over v of max over k of (d_n(v) − d_k(v))/(n − k).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def min_mean_cycle(n: int, edges: list[tuple[int, int, int]]) -> float | None:
    """Karp's algorithm. Returns the minimum mean weight of a directed cycle, or None if the graph is acyclic."""
    inf = 1 << 60
    d = [[inf] * n for _ in range(n + 1)]
    d[0] = [0] * n
    for k in range(1, n + 1):
        prev = d[k - 1]
        cur = d[k]
        for u, v, w in edges:
            if prev[u] < inf and prev[u] + w < cur[v]:
                cur[v] = prev[u] + w
    best: float | None = None
    for v in range(n):
        if d[n][v] >= inf:
            continue
        worst = max((d[n][v] - d[k][v]) / (n - k) for k in range(n) if d[k][v] < inf)
        best = worst if best is None else min(best, worst)
    return best


def demo() -> str:
    lines: list[str] = []
    e = [(0, 1, 1), (1, 2, 2), (2, 0, 3), (1, 3, -1), (3, 1, 0)]
    lines.append(f"edges {e}")
    lines.append(f"min mean cycle weight = {min_mean_cycle(4, e)} (cycle 1→3→1 has weight −1 over 2 edges)")
    lines.append(f"DAG → {min_mean_cycle(3, [(0, 1, 5), (1, 2, 5)])}")
    return "\n".join(lines) + "\n"


def _brute(n: int, edges: list[tuple[int, int, int]]) -> float | None:
    best: float | None = None
    vis = [False] * n

    def go(start: int, u: int, w: int, length: int) -> None:
        nonlocal best
        for a, b, c in edges:
            if a != u:
                continue
            if b == start:
                m = (w + c) / (length + 1)
                best = m if best is None else min(best, m)
            elif b > start and not vis[b]:
                vis[b] = True
                go(start, b, w + c, length + 1)
                vis[b] = False

    for s in range(n):
        vis = [False] * n
        vis[s] = True
        go(s, s, 0, 0)
    return best


class Tests(unittest.TestCase):
    def test_matches_cycle_enumeration(self) -> None:
        r = Rng(144)
        for _ in range(100):
            n = r.range(1, 6)
            edges = [(r.range(0, n - 1), r.range(0, n - 1), r.range(0, 14) - 4) for _ in range(r.range(0, 12))]
            got = min_mean_cycle(n, edges)
            want = _brute(n, edges)
            if want is None:
                self.assertIsNone(got, f"{edges}")
            else:
                self.assertIsNotNone(got, f"{edges}")
                assert got is not None
                self.assertLess(abs(got - want), 1e-9, f"{got} vs {want} for {edges}")

    def test_simple_cases(self) -> None:
        self.assertEqual(min_mean_cycle(2, [(0, 1, 3), (1, 0, 5)]), 4.0)
        self.assertEqual(min_mean_cycle(1, [(0, 0, -7)]), -7.0)
        self.assertIsNone(min_mean_cycle(3, [(0, 1, 5), (1, 2, 5)]))


if __name__ == "__main__":
    print(demo(), end="")
