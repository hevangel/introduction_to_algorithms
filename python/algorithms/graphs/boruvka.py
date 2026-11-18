"""Borůvka's Algorithm — CLRS Problem 23-2 (3e) / 21 problems (4e).

Every component simultaneously picks its cheapest outgoing edge: at least half the components merge each round, O(E lg V) with simple data structures.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Edge = tuple[int, int, int]


def _find(p: list[int], x: int) -> int:
    r = x
    while p[r] != r:
        r = p[r]
    c = x
    while p[c] != r:
        p[c], c = r, p[c]
    return r


def boruvka(n: int, edges: list[Edge]) -> tuple[int, list[int], int]:
    """Borůvka: returns (total weight, chosen edge indices, number of rounds).

    Ties are broken by edge index, preventing cycles. On a disconnected graph it returns a minimum spanning forest.
    """
    p = list(range(n))
    chosen: list[int] = []
    rounds = 0
    while True:
        cheapest: list[tuple[int, int] | None] = [None] * n
        for i, (u, v, w) in enumerate(edges):
            a, b = _find(p, u), _find(p, v)
            if a == b:
                continue
            cand = (w, i)
            for c in (a, b):
                cur = cheapest[c]
                if cur is None or cand < cur:
                    cheapest[c] = cand
        picks = [item[1] for item in cheapest if item is not None]
        if not picks:
            break
        rounds += 1
        for i in picks:
            a, b = _find(p, edges[i][0]), _find(p, edges[i][1])
            if a != b:
                p[a] = b
                chosen.append(i)
    return sum(edges[i][2] for i in chosen), chosen, rounds


def _kruskal(n: int, edges: list[Edge]) -> tuple[int, list[int]]:
    p = list(range(n))
    chosen: list[int] = []
    for i in sorted(range(len(edges)), key=lambda idx: edges[idx][2]):
        a, b = _find(p, edges[i][0]), _find(p, edges[i][1])
        if a != b:
            p[a] = b
            chosen.append(i)
    return sum(edges[i][2] for i in chosen), chosen


def demo() -> str:
    lines: list[str] = []
    e: list[Edge] = [
        (0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2),
        (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7),
    ]
    w, edges_idx, rounds = boruvka(9, e)
    kw, _ = _kruskal(9, e)
    lines.append(f"Borůvka MST weight {w} (Kruskal {kw}) after {rounds} rounds (≤ ⌈lg 9⌉ = 4)")
    lines.append(f"edges: {[e[i] for i in edges_idx]}")
    r = Rng(1)
    n = 5000
    big: list[Edge] = [(r.range(0, v - 1), v, r.range(1, 1000)) for v in range(1, n)]
    for _ in range(20_000):
        a, b = r.range(0, n - 1), r.range(0, n - 1)
        if a != b:
            big.append((a, b, r.range(1, 1000)))
    lines.append(f"random graph n = 5000, m = {len(big)}: {boruvka(n, big)[2]} rounds (lg n = {math.log2(n):.1f})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_kruskal_even_with_tied_weights(self) -> None:
        r = Rng(353)
        for _ in range(150):
            n = r.range(1, 20)
            edges: list[Edge] = []
            for v in range(1, n):
                if r.f64() < 0.9:
                    edges.append((r.range(0, v - 1), v, r.range(1, 6)))
            for _ in range(r.range(0, 30)):
                a, b = r.range(0, n - 1), r.range(0, n - 1)
                if a != b:
                    edges.append((a, b, r.range(1, 6)))
            w, chosen, rounds = boruvka(n, edges)
            kw, kt = _kruskal(n, edges)
            self.assertEqual((w, len(chosen)), (kw, len(kt)))
            self.assertLessEqual(rounds, math.ceil(math.log2(n)) + 1, f"rounds {rounds} n {n}")


if __name__ == "__main__":
    print(demo(), end="")
