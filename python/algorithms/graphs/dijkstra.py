"""Dijkstra's Algorithm — CLRS 24.3 (3e) / 22.3 (4e) / 25.2 (1e).

Greedy single-source shortest paths for non-negative weights, using a min-priority queue.
"""

from __future__ import annotations

import heapq
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

INF = (1 << 60)
WeightedAdj = list[list[tuple[int, int]]]


def dijkstra(adj: WeightedAdj, s: int) -> tuple[list[int], list[int | None]]:
    """DIJKSTRA(G, w, s) with a binary heap: O((V+E) lg V). All weights must be ≥ 0."""
    n = len(adj)
    d = [INF] * n
    pi: list[int | None] = [None] * n
    d[s] = 0
    q: list[tuple[int, int]] = [(0, s)]
    while q:
        du, u = heapq.heappop(q)
        if du > d[u]:
            continue
        for v, w in adj[u]:
            assert w >= 0
            if du + w < d[v]:
                d[v] = du + w
                pi[v] = u
                heapq.heappush(q, (d[v], v))
    return d, pi


def dijkstra_array(adj: WeightedAdj, s: int) -> list[int]:
    """Θ(V²) array version for dense graphs."""
    n = len(adj)
    d = [INF] * n
    done = [False] * n
    d[s] = 0
    for _ in range(n):
        u = -1
        best = INF
        for v in range(n):
            if not done[v] and d[v] < best:
                best = d[v]
                u = v
        if u == -1:
            break
        done[u] = True
        for v, w in adj[u]:
            if d[u] + w < d[v]:
                d[v] = d[u] + w
    return d


def path(pi: list[int | None], s: int, v: int) -> list[int] | None:
    p = [v]
    cur = v
    while cur != s:
        pred = pi[cur]
        if pred is None:
            return None
        cur = pred
        p.append(cur)
    p.reverse()
    return p


def _bellman_ford(n: int, edges: list[tuple[int, int, int]], s: int) -> list[int]:
    d = [INF] * n
    d[s] = 0
    for _ in range(1, n):
        changed = False
        for u, v, w in edges:
            if d[u] < INF and d[u] + w < d[v]:
                d[v] = d[u] + w
                changed = True
        if not changed:
            break
    return d


def demo() -> str:
    lines: list[str] = []
    # Figure 24.6: s=0 t=1 x=2 y=3 z=4
    e = [(0, 1, 10), (0, 3, 5), (1, 2, 1), (1, 3, 2), (2, 4, 4), (3, 1, 3), (3, 2, 9), (3, 4, 2), (4, 0, 7), (4, 2, 6)]
    adj: WeightedAdj = [[] for _ in range(5)]
    for u, v, w in e:
        adj[u].append((v, w))
    d, pi = dijkstra(adj, 0)
    lines.append(f"distances from s: {d}")
    lines.append(f"array version   : {dijkstra_array(adj, 0)}")
    lines.append(f"shortest path s → x: {path(pi, 0, 2)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_figure_24_6(self) -> None:
        e = [(0, 1, 10), (0, 3, 5), (1, 2, 1), (1, 3, 2), (2, 4, 4), (3, 1, 3), (3, 2, 9), (3, 4, 2), (4, 0, 7), (4, 2, 6)]
        adj: WeightedAdj = [[] for _ in range(5)]
        for u, v, w in e:
            adj[u].append((v, w))
        self.assertEqual(dijkstra(adj, 0)[0], [0, 8, 9, 5, 7])

    def test_agrees_with_bellman_ford(self) -> None:
        r = Rng(132)
        for _ in range(60):
            n = r.range(1, 14)
            edges = [(r.range(0, n - 1), r.range(0, n - 1), r.range(0, 20)) for _ in range(r.range(0, 40))]
            adj: WeightedAdj = [[] for _ in range(n)]
            for u, v, w in edges:
                adj[u].append((v, w))
            s = r.range(0, n - 1)
            d, pi = dijkstra(adj, s)
            bf = _bellman_ford(n, edges, s)
            self.assertEqual(d, bf)
            self.assertEqual(dijkstra_array(adj, s), bf)
            for v in range(n):
                p = path(pi, s, v)
                if p is not None:
                    self.assertEqual(p[0], s)


if __name__ == "__main__":
    print(demo(), end="")
