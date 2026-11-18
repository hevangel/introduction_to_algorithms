"""Floyd-Warshall All-Pairs Shortest Paths — CLRS 25.2–25.3 (3e) / 23.2 (4e) / 26.2 (1e).

Θ(V³) dynamic program over intermediate vertices; also transitive closure (Warshall).
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


def floyd_warshall(w: list[list[int]]) -> tuple[list[list[int]], list[list[int | None]]]:
    """FLOYD-WARSHALL(W): w[i][j] = edge weight, INF if none, w[i][i] ≤ 0. Returns (dist, predecessor matrix)."""
    n = len(w)
    d = [row[:] for row in w]
    pi: list[list[int | None]] = [
        [i if i != j and w[i][j] < INF else None for j in range(n)]
        for i in range(n)
    ]
    for k in range(n):
        dk = d[k]
        pik = pi[k]
        for i in range(n):
            dik = d[i][k]
            if dik >= INF:
                continue
            di = d[i]
            pii = pi[i]
            for j in range(n):
                if dk[j] < INF and dik + dk[j] < di[j]:
                    di[j] = dik + dk[j]
                    pii[j] = pik[j]
    return d, pi


def path(pi: list[list[int | None]], i: int, j: int) -> list[int] | None:
    """PRINT-ALL-PAIRS-SHORTEST-PATH."""
    p = [j]
    cur = j
    while cur != i:
        pred = pi[i][cur]
        if pred is None:
            return None
        cur = pred
        p.append(cur)
        if len(p) > len(pi) + 1:
            return None
    p.reverse()
    return p


def has_negative_cycle(d: list[list[int]]) -> bool:
    """A negative-weight cycle exists iff some diagonal entry of the result is negative (Exercise 25.2-6)."""
    return any(d[i][i] < 0 for i in range(len(d)))


def transitive_closure(adj: list[list[bool]]) -> list[list[bool]]:
    """Warshall's transitive closure: t[i][j] iff a path (of ≥ 0 edges) exists."""
    n = len(adj)
    t = [row[:] for row in adj]
    for i in range(n):
        t[i][i] = True
    for k in range(n):
        for i in range(n):
            if t[i][k]:
                for j in range(n):
                    if t[k][j]:
                        t[i][j] = True
    return t


def matrix(n: int, edges: list[tuple[int, int, int]]) -> list[list[int]]:
    w = [[INF] * n for _ in range(n)]
    for i in range(n):
        w[i][i] = 0
    for u, v, c in edges:
        if c < w[u][v]:
            w[u][v] = c
    return w


def _dijkstra(adj: list[list[tuple[int, int]]], s: int) -> list[int]:
    n = len(adj)
    d = [INF] * n
    d[s] = 0
    pq: list[tuple[int, int]] = [(0, s)]
    while pq:
        du, u = heapq.heappop(pq)
        if du > d[u]:
            continue
        for v, w in adj[u]:
            if du + w < d[v]:
                d[v] = du + w
                heapq.heappush(pq, (d[v], v))
    return d


def demo() -> str:
    lines: list[str] = []
    e = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)]
    d, pi = floyd_warshall(matrix(5, e))
    lines.append("all-pairs shortest-path matrix:")
    for row in d:
        lines.append(f"  {row}")
    p = path(pi, 2, 3)
    lines.append(f"path 3 → 4 (1-based vertices): {[x + 1 for x in p] if p is not None else None}")
    adj = [[False, True, False], [False, False, True], [False, False, False]]
    lines.append("transitive closure of 0→1→2:")
    for row in transitive_closure(adj):
        lines.append(f"  {row}")
    neg, _ = floyd_warshall(matrix(3, [(0, 1, 1), (1, 2, -3), (2, 0, 1)]))
    lines.append(f"negative cycle detected: {has_negative_cycle(neg)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_figure_25_1(self) -> None:
        e = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)]
        m = matrix(5, e)
        d, pi = floyd_warshall(m)
        self.assertEqual(
            d,
            [[0, 1, -3, 2, -4], [3, 0, -4, 1, -1], [7, 4, 0, 5, 3], [2, -1, -5, 0, -2], [8, 5, 1, 6, 0]],
        )
        p = path(pi, 0, 2)
        assert p is not None
        self.assertEqual(p, [0, 4, 3, 2])
        self.assertEqual(sum(m[p[i]][p[i + 1]] for i in range(len(p) - 1)), -3)

    def test_matches_dijkstra_for_nonnegative_weights(self) -> None:
        r = Rng(134)
        for _ in range(40):
            n = r.range(1, 10)
            edges = [(r.range(0, n - 1), r.range(0, n - 1), r.range(0, 15)) for _ in range(r.range(0, 30))]
            d, _ = floyd_warshall(matrix(n, edges))
            adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
            for u, v, w in edges:
                adj[u].append((v, w))
            for s in range(n):
                self.assertEqual(d[s], _dijkstra(adj, s))
            self.assertFalse(has_negative_cycle(d))

    def test_closure_matches_floyd_reachability(self) -> None:
        r = Rng(135)
        n = 9
        adj = [[u != v and r.f64() < 0.15 for v in range(n)] for u in range(n)]
        edges = [(u, v, 1) for u in range(n) for v in range(n) if u != v and adj[u][v]]
        d, _ = floyd_warshall(matrix(n, edges))
        t = transitive_closure(adj)
        for i in range(n):
            for j in range(n):
                self.assertEqual(t[i][j], d[i][j] < INF)


if __name__ == "__main__":
    print(demo(), end="")
