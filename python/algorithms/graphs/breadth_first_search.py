"""Breadth-First Search — CLRS 22.2 (3e) / 20.2 (4e) / 23.2 (1e–2e).

Level-by-level traversal: shortest paths (in edges) from a source, bipartiteness test and tree diameter.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def bfs(adj: list[list[int]], s: int) -> tuple[list[int | None], list[int | None]]:
    """BFS(G, s): returns (distance in edges or None if unreachable, predecessor)."""
    n = len(adj)
    dist: list[int | None] = [None] * n
    pi: list[int | None] = [None] * n
    dist[s] = 0
    q: deque[int] = deque([s])
    while q:
        u = q.popleft()
        du = dist[u]
        assert du is not None
        for v in adj[u]:
            if dist[v] is None:
                dist[v] = du + 1
                pi[v] = u
                q.append(v)
    return dist, pi


def path(pi: list[int | None], s: int, v: int) -> list[int] | None:
    """PRINT-PATH(G, s, v): the vertices of a shortest path from s to v."""
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


def is_bipartite(adj: list[list[int]]) -> list[int] | None:
    """Exercise 22.2-6: 2-colour an undirected graph with BFS; None if an odd cycle exists."""
    n = len(adj)
    color = [2] * n
    for s in range(n):
        if color[s] != 2:
            continue
        color[s] = 0
        q: deque[int] = deque([s])
        while q:
            u = q.popleft()
            for v in adj[u]:
                if color[v] == 2:
                    color[v] = 1 - color[u]
                    q.append(v)
                elif color[v] == color[u]:
                    return None
    return color


def tree_diameter(adj: list[list[int]]) -> int:
    """Exercise 22.2-7 (a tree's diameter): BFS from any vertex to the farthest vertex, then BFS again."""

    def farthest(s: int) -> tuple[int, int]:
        d, _ = bfs(adj, s)
        v = max((i for i in range(len(adj)) if d[i] is not None), key=lambda i: d[i])  # type: ignore[arg-type,return-value]
        dv = d[v]
        assert dv is not None
        return v, dv

    a, _ = farthest(0)
    return farthest(a)[1]


def demo() -> str:
    lines: list[str] = []
    edges = [(0, 1), (0, 4), (1, 5), (5, 2), (5, 6), (2, 6), (2, 3), (3, 6), (3, 7), (6, 7)]
    adj: list[list[int]] = [[] for _ in range(8)]
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    d, pi = bfs(adj, 1)
    lines.append(f"BFS from vertex 1: distances {d}")
    lines.append(f"predecessors: {pi}")
    lines.append(f"shortest path 1 → 7: {path(pi, 1, 7)}")
    lines.append(f"bipartite? {is_bipartite(adj) is not None}")
    square = [[1, 3], [0, 2], [1, 3], [0, 2]]
    lines.append(f"4-cycle bipartite colouring: {is_bipartite(square)}")
    tree = [[1], [0, 2, 3], [1, 4], [1], [2]]
    lines.append(f"tree diameter = {tree_diameter(tree)}")
    return "\n".join(lines) + "\n"


def _floyd(adj: list[list[int]]) -> list[list[int]]:
    n = len(adj)
    inf = 1 << 30
    d = [[inf] * n for _ in range(n)]
    for u in range(n):
        d[u][u] = 0
        for v in adj[u]:
            d[u][v] = 1
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if d[i][k] + d[k][j] < d[i][j]:
                    d[i][j] = d[i][k] + d[k][j]
    return d


class Tests(unittest.TestCase):
    def test_distances_match_floyd_warshall(self) -> None:
        r = Rng(122)
        inf = 1 << 30
        for _ in range(30):
            n = r.range(1, 12)
            adj = [[v for v in range(n) if u != v and r.f64() < 0.2] for u in range(n)]
            fw = _floyd(adj)
            for s in range(n):
                d, pi = bfs(adj, s)
                for v in range(n):
                    self.assertEqual(d[v], None if fw[s][v] >= inf else fw[s][v])
                    p = path(pi, s, v)
                    if p is not None:
                        dv = d[v]
                        assert dv is not None
                        self.assertEqual(len(p) - 1, dv)

    def test_bipartite_and_diameter(self) -> None:
        c5 = [[(i + 1) % 5, (i + 4) % 5] for i in range(5)]
        self.assertIsNone(is_bipartite(c5))
        c6 = [[(i + 1) % 6, (i + 5) % 6] for i in range(6)]
        col = is_bipartite(c6)
        self.assertIsNotNone(col)
        assert col is not None
        for u in range(6):
            for v in c6[u]:
                self.assertNotEqual(col[u], col[v])
        path_graph = [[x for x in (i - 1, i + 1) if 0 <= x < 7] for i in range(7)]
        self.assertEqual(tree_diameter(path_graph), 6)


if __name__ == "__main__":
    print(demo(), end="")
