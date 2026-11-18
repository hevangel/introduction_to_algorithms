"""Bitonic Shortest Paths — CLRS Problem 24-6 (1e–4e).

Shortest paths whose edge weights first increase then decrease: sort the edges once and relax them in ascending, then descending, order — O(E lg E).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = (1 << 60)
Edge = tuple[int, int, int]


def bitonic_shortest_paths(n: int, edges: list[Edge], s: int) -> list[int]:
    """Shortest bitonic path lengths from s (weights increase, then decrease), assuming distinct non-negative weights."""
    sorted_edges = sorted(edges, key=lambda e: e[2])
    inc = [INF] * n
    inc[s] = 0
    for u, v, w in sorted_edges:
        if inc[u] < INF and inc[u] + w < inc[v]:
            inc[v] = inc[u] + w
    best = list(inc)
    for u, v, w in reversed(sorted_edges):
        if best[u] < INF and best[u] + w < best[v]:
            best[v] = best[u] + w
    return best


def is_bitonic(w: list[int]) -> bool:
    if not w:
        return True
    peak = max(range(len(w)), key=lambda i: w[i])
    return all(w[i] < w[i + 1] for i in range(peak)) and all(w[i] > w[i + 1] for i in range(peak, len(w) - 1))


def demo() -> str:
    lines: list[str] = []
    e: list[Edge] = [(0, 1, 5), (1, 2, 9), (2, 3, 4), (0, 3, 20), (1, 3, 7), (3, 4, 2), (0, 2, 12), (2, 4, 3)]
    lines.append(f"edges (u, v, weight) {e}")
    d = bitonic_shortest_paths(5, e, 0)
    lines.append(f"bitonic shortest paths from 0: {[-1 if x >= INF else x for x in d]}")
    lines.append(f"path 0→1→2→3→4 has weights 5,9,4,2: increasing then decreasing = {is_bitonic([5, 9, 4, 2])}")
    return "\n".join(lines) + "\n"


def _brute(n: int, edges: list[Edge], s: int) -> list[int]:
    best = [INF] * n
    vis = [False] * n
    vis[s] = True
    ws: list[int] = []

    def go(u: int, length: int) -> None:
        if not ws or is_bitonic(ws):
            if length < best[u]:
                best[u] = length
        else:
            return
        for a, v, w in edges:
            if a == u and not vis[v]:
                vis[v] = True
                ws.append(w)
                go(v, length + w)
                ws.pop()
                vis[v] = False

    go(s, 0)
    return best


class Tests(unittest.TestCase):
    def test_matches_exhaustive_enumeration(self) -> None:
        r = Rng(364)
        for _ in range(300):
            n = r.range(2, 7)
            ws = list(range(1, 41))
            r.shuffle(ws)
            edges = []
            for i in range(r.range(1, 14)):
                u, v = r.range(0, n - 1), r.range(0, n - 1)
                if u != v:
                    edges.append((u, v, ws[i]))
            s = r.range(0, n - 1)
            self.assertEqual(bitonic_shortest_paths(n, edges, s), _brute(n, edges, s), f"{edges}")

    def test_bitonic_predicate(self) -> None:
        self.assertTrue(is_bitonic([1, 4, 9, 6, 2]))
        self.assertTrue(is_bitonic([1, 2, 3]))
        self.assertTrue(is_bitonic([3, 2, 1]))
        self.assertTrue(is_bitonic([5]))
        self.assertFalse(is_bitonic([1, 5, 2, 6]))


if __name__ == "__main__":
    print(demo(), end="")
