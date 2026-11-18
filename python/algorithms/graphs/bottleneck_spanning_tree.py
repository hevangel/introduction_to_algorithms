"""Bottleneck Spanning Trees — CLRS Problem 23-3 (1e–4e).

A spanning tree minimising its heaviest edge: every MST is one; test "bottleneck ≤ b" by connectivity of the light edges and search over weights.
"""

from __future__ import annotations

from bisect import bisect_left
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


def connected_with_limit(n: int, edges: list[Edge], b: int) -> bool:
    """Does the subgraph of edges with weight ≤ b connect all n vertices?"""
    p = list(range(n))
    comps = n
    for u, v, w in edges:
        if w <= b:
            x, y = _find(p, u), _find(p, v)
            if x != y:
                p[x] = y
                comps -= 1
    return comps <= 1


def min_bottleneck(n: int, edges: list[Edge]) -> int | None:
    """Smallest possible bottleneck weight over all spanning trees, found by binary search over sorted distinct weights: O(E lg E)."""
    if n <= 1:
        return -(1 << 62)
    ws = sorted({e[2] for e in edges})
    lo, hi = 0, len(ws)
    while lo < hi:
        mid = (lo + hi) // 2
        if connected_with_limit(n, edges, ws[mid]):
            hi = mid
        else:
            lo = mid + 1
    return ws[lo] if lo < len(ws) else None


def bottleneck(edges: list[Edge], tree: list[int]) -> int | None:
    """The bottleneck (maximum edge weight) of a set of tree edges."""
    return max((edges[i][2] for i in tree), default=None)


def _kruskal_tree(n: int, edges: list[Edge]) -> list[int]:
    p = list(range(n))
    chosen: list[int] = []
    for i in sorted(range(len(edges)), key=lambda idx: edges[idx][2]):
        a, b = _find(p, edges[i][0]), _find(p, edges[i][1])
        if a != b:
            p[a] = b
            chosen.append(i)
    return chosen


def demo() -> str:
    lines: list[str] = []
    e: list[Edge] = [
        (0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2),
        (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7),
    ]
    mst = _kruskal_tree(9, e)
    lines.append(f"minimum bottleneck by search: {min_bottleneck(9, e)}")
    lines.append(f"bottleneck of the MST (Kruskal): {bottleneck(e, mst)}  — an MST is always a minimum bottleneck spanning tree")
    lines.append(f"connected using edges ≤ 8? {connected_with_limit(9, e, 8)}; ≤ 9? {connected_with_limit(9, e, 9)}")
    return "\n".join(lines) + "\n"


def _spanning_tree_bottlenecks(n: int, edges: list[Edge]) -> list[int]:
    m = len(edges)
    res: list[int] = []
    for mask in range(1 << m):
        if mask.bit_count() != n - 1:
            continue
        ids = [i for i in range(m) if (mask >> i) & 1]
        p = list(range(n))
        ok = True
        for i in ids:
            a, b = _find(p, edges[i][0]), _find(p, edges[i][1])
            if a == b:
                ok = False
                break
            p[a] = b
        if ok:
            res.append(max(edges[i][2] for i in ids))
    return res


class Tests(unittest.TestCase):
    def test_matches_brute_force_and_the_mst_is_a_bottleneck_tree(self) -> None:
        r = Rng(352)
        for _ in range(100):
            n = r.range(2, 6)
            edges: list[Edge] = [(r.range(0, v - 1), v, r.range(1, 15)) for v in range(1, n)]
            for _ in range(r.range(0, 5)):
                a, b = r.range(0, n - 1), r.range(0, n - 1)
                if a != b:
                    edges.append((a, b, r.range(1, 15)))
            want = min(_spanning_tree_bottlenecks(n, edges))
            self.assertEqual(min_bottleneck(n, edges), want)
            mst = _kruskal_tree(n, edges)
            self.assertEqual(bottleneck(edges, mst), want)

    def test_disconnected_graph_has_no_spanning_tree(self) -> None:
        self.assertIsNone(min_bottleneck(4, [(0, 1, 3), (2, 3, 5)]))
        self.assertTrue(connected_with_limit(2, [(0, 1, 3)], 3))
        self.assertFalse(connected_with_limit(2, [(0, 1, 3)], 2))


if __name__ == "__main__":
    print(demo(), end="")
