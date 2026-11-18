"""Yen's Improvement to Bellman-Ford — CLRS Problem 24-1 (1e–4e) / 22-1.

Relax forward edges in increasing vertex order, then backward edges in decreasing order: at most ⌈|V|/2⌉ passes instead of |V|−1.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = ((1 << 63) - 1) // 4
Edge = tuple[int, int, int]


def _relax(d: list[int], edge_set: list[Edge]) -> bool:
    changed = False
    for u, v, w in edge_set:
        if d[u] < INF and d[u] + w < d[v]:
            d[v] = d[u] + w
            changed = True
    return changed


def yen_bellman_ford(n: int, edges: list[Edge], s: int) -> tuple[list[int] | None, int]:
    """Bellman–Ford with Yen's edge ordering.

    Returns (distances, or None on a reachable negative cycle; passes that changed something).
    """
    fwd = [e for e in edges if e[0] < e[1]]  # E_f: edges that go "up" in the vertex order
    bwd = [e for e in edges if e[0] > e[1]]  # E_b: edges that go "down"
    fwd.sort(key=lambda e: e[0])  # increasing source: a whole increasing run is relaxed in one sweep
    bwd.sort(key=lambda e: -e[0])  # decreasing source: a whole decreasing run in one sweep
    d = [INF] * n
    d[s] = 0
    passes = 0
    for _ in range(n):
        c1 = _relax(d, fwd)
        c2 = _relax(d, bwd)
        if not (c1 or c2):
            return d, passes
        passes += 1
    return None, passes  # still changing after n passes: negative cycle


def plain_passes(n: int, edges: list[Edge], s: int) -> int:
    """Number of passes of the ordinary Bellman–Ford (arbitrary edge order) until nothing changes."""
    d = [INF] * n
    d[s] = 0
    passes = 0
    while True:
        ch = _relax(d, edges)
        if not ch or passes > n:
            return passes
        passes += 1


def _bellman_ford(n: int, edges: list[Edge], s: int) -> list[int] | None:
    d = [INF] * n
    d[s] = 0
    for _ in range(n - 1):
        if not _relax(d, edges):
            break
    for u, v, w in edges:
        if d[u] < INF and d[u] + w < d[v]:
            return None
    return d


def demo() -> str:
    lines: list[str] = []
    # a path whose vertex numbers zig-zag, so shortest paths alternate between increasing and decreasing runs
    n = 9
    order = [0, 8, 1, 7, 2, 6, 3, 5, 4]
    edges: list[Edge] = [(order[i], order[i + 1], 1) for i in range(len(order) - 1)]
    d, passes = yen_bellman_ford(n, edges, 0)
    assert d is not None
    lines.append(f"zig-zag path of {n} vertices: distances {d}")
    lines.append(f"Yen passes that changed something: {passes} (≤ ⌈n/2⌉ = {-(-n // 2)}); worst-case ordinary Bellman–Ford passes: n−1 = {n - 1}")
    lines.append(f"ordinary Bellman–Ford with the edges in the worst order needed {plain_passes(n, edges[::-1], 0)} passes")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_same_distances_as_bellman_ford_and_few_passes(self) -> None:
        r = Rng(357)
        for _ in range(200):
            n = r.range(2, 12)
            # weights c + h[u] − h[v] with c ≥ 0 may be negative but never create a negative cycle
            h = r.vec(n, 0, 6)
            edges: list[Edge] = []
            for _ in range(r.range(1, 30)):
                u, v = r.range(0, n - 1), r.range(0, n - 1)
                w = r.range(0, 5) + h[u] - h[v]
                if u != v:
                    edges.append((u, v, w))
            s = r.range(0, n - 1)
            d, passes = yen_bellman_ford(n, edges, s)
            self.assertEqual(d, _bellman_ford(n, edges, s))
            self.assertLessEqual(passes, -(-n // 2) + 1, f"passes {passes} n {n}")

    def test_negative_cycle_is_reported_and_zigzag_needs_half_the_passes(self) -> None:
        self.assertIsNone(yen_bellman_ford(3, [(0, 1, 1), (1, 2, -3), (2, 0, 1)], 0)[0])
        n = 21
        order: list[int] = []
        lo, hi = 0, n - 1
        while lo <= hi:
            order.append(lo)
            if lo != hi:
                order.append(hi)
            lo += 1
            if hi == 0:
                break
            hi -= 1
        edges: list[Edge] = [(order[i], order[i + 1], 1) for i in range(len(order) - 1)]
        d, passes = yen_bellman_ford(n, edges, order[0])
        self.assertTrue(d is not None and passes <= n // 2 + 1, passes)


if __name__ == "__main__":
    print(demo(), end="")
