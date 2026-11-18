"""Johnson's Algorithm — CLRS 25.3 (3e) / 23.3 (4e) / 26.4 (1e).

All-pairs shortest paths on sparse graphs: reweight with Bellman-Ford potentials, then run Dijkstra from every vertex.
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


def _bellman_ford(n: int, edges: list[tuple[int, int, int]], s: int) -> list[int] | None:
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
    for u, v, w in edges:
        if d[u] < INF and d[u] + w < d[v]:
            return None
    return d


def johnson(n: int, edges: list[tuple[int, int, int]]) -> list[list[int]] | None:
    """JOHNSON(G): matrix of shortest distances (INF for unreachable pairs) or None if a negative cycle exists."""
    ext = list(edges) + [(n, v, 0) for v in range(n)]
    h = _bellman_ford(n + 1, ext, n)
    if h is None:
        return None
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for u, v, w in edges:
        adj[u].append((v, w + h[u] - h[v]))
    d = [[INF] * n for _ in range(n)]
    for u in range(n):
        dist = [INF] * n
        dist[u] = 0
        q: list[tuple[int, int]] = [(0, u)]
        while q:
            du, x = heapq.heappop(q)
            if du > dist[x]:
                continue
            for y, w in adj[x]:
                if du + w < dist[y]:
                    dist[y] = du + w
                    heapq.heappush(q, (dist[y], y))
        for v in range(n):
            if dist[v] < INF:
                d[u][v] = dist[v] + h[v] - h[u]
    return d


def _floyd_warshall(n: int, edges: list[tuple[int, int, int]]) -> tuple[list[list[int]], bool]:
    d = [[INF] * n for _ in range(n)]
    for i in range(n):
        d[i][i] = 0
    for u, v, c in edges:
        if c < d[u][v]:
            d[u][v] = c
    for k in range(n):
        for i in range(n):
            if d[i][k] < INF:
                for j in range(n):
                    if d[k][j] < INF and d[i][k] + d[k][j] < d[i][j]:
                        d[i][j] = d[i][k] + d[k][j]
    has_neg = any(d[i][i] < 0 for i in range(n))
    return d, has_neg


def demo() -> str:
    lines: list[str] = []
    e = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)]
    d = johnson(5, e)
    assert d is not None
    lines.append("Johnson on the Figure 25.1 graph:")
    for row in d:
        lines.append(f"  {row}")
    fw, _ = _floyd_warshall(5, e)
    lines.append(f"equals Floyd-Warshall: {d == fw}")
    lines.append(f"negative cycle graph → {johnson(3, [(0, 1, 1), (1, 2, -3), (2, 0, 1)])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_floyd_warshall_with_negative_edges(self) -> None:
        r = Rng(136)
        tested = 0
        for _ in range(200):
            n = r.range(1, 9)
            edges = [(r.range(0, n - 1), r.range(0, n - 1), r.range(0, 20) - 4) for _ in range(r.range(0, 25))]
            fw, neg = _floyd_warshall(n, edges)
            res = johnson(n, edges)
            if res is None:
                self.assertTrue(neg)
            else:
                self.assertFalse(neg)
                self.assertEqual(res, fw)
                tested += 1
        self.assertGreater(tested, 20)


if __name__ == "__main__":
    print(demo(), end="")
