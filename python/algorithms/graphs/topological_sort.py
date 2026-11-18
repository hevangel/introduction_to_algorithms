"""Topological Sort — CLRS 22.4 (3e) / 20.4 (4e) / 23.4 (1e–2e).

Linearly order a DAG's vertices by decreasing DFS finish time, or by repeatedly removing sources (Kahn).
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _dfs_finish_order(adj: list[list[int]]) -> tuple[list[int], bool]:
    """Iterative DFS over all vertices; returns (finish order, has a back edge)."""
    n = len(adj)
    color = [0] * n  # 0 white, 1 gray, 2 black
    finish: list[int] = []
    back = False
    for root in range(n):
        if color[root] != 0:
            continue
        color[root] = 1
        stack = [(root, 0)]
        while stack:
            u, i = stack.pop()
            if i < len(adj[u]):
                stack.append((u, i + 1))
                v = adj[u][i]
                if color[v] == 0:
                    color[v] = 1
                    stack.append((v, 0))
                elif color[v] == 1:
                    back = True
            else:
                color[u] = 2
                finish.append(u)
    return finish, back


def topological_sort(adj: list[list[int]]) -> list[int] | None:
    """TOPOLOGICAL-SORT(G): vertices by decreasing finishing time; None if G has a cycle."""
    finish, back = _dfs_finish_order(adj)
    if back:
        return None
    finish.reverse()
    return finish


def kahn(adj: list[list[int]]) -> list[int] | None:
    """Kahn's algorithm (Exercise 22.4-5): repeatedly output a vertex of in-degree 0."""
    n = len(adj)
    indeg = [0] * n
    for vs in adj:
        for v in vs:
            indeg[v] += 1
    q = deque(v for v in range(n) if indeg[v] == 0)
    order: list[int] = []
    while q:
        u = q.popleft()
        order.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return order if len(order) == n else None


def count_paths(adj: list[list[int]], s: int, t: int) -> int | None:
    """Exercise 22.4-2: number of distinct paths from s to t in a DAG, in Θ(V+E)."""
    order = topological_sort(adj)
    if order is None:
        return None
    ways = [0] * len(adj)
    ways[s] = 1
    for u in order:
        for v in adj[u]:
            ways[v] += ways[u]
    return ways[t]


def is_valid_order(adj: list[list[int]], order: list[int]) -> bool:
    pos = [-1] * len(adj)
    for i, v in enumerate(order):
        pos[v] = i
    return len(order) == len(adj) and all(pos[u] < pos[v] for u, vs in enumerate(adj) for v in vs)


def _show(x: object) -> str:
    """Rust-Debug-style rendering of Option values."""
    return "None" if x is None else f"Some({x})"


def demo() -> str:
    lines: list[str] = []
    # Professor Bumstead's getting-dressed DAG (Figure 22.7)
    names = ["undershorts", "pants", "belt", "shirt", "tie", "jacket", "socks", "shoes", "watch"]
    edges = [(0, 1), (0, 7), (1, 2), (1, 7), (2, 5), (3, 2), (3, 4), (4, 5), (6, 7)]
    adj: list[list[int]] = [[] for _ in range(9)]
    for a, b in edges:
        adj[a].append(b)
    order = topological_sort(adj)
    assert order is not None
    kn = kahn(adj)
    assert kn is not None
    lines.append(f"DFS-based order : {' → '.join(names[i] for i in order)}")
    lines.append(f"Kahn's order    : {' → '.join(names[i] for i in kn)}")
    lines.append(f"valid? {str(is_valid_order(adj, order)).lower()}")
    diamond = [[1, 2], [3], [3], []]
    lines.append(f"paths 0 → 3 in a diamond DAG: {_show(count_paths(diamond, 0, 3))}")
    lines.append(f"cyclic graph: {_show(topological_sort([[1], [0]]))}")
    return "\n".join(lines) + "\n"


def _random_dag(n: int, r: Rng) -> list[list[int]]:
    perm = list(range(n))
    r.shuffle(perm)
    adj: list[list[int]] = [[] for _ in range(n)]
    for i in range(n):
        for j in range(i + 1, n):
            if r.f64() < 0.3:
                adj[perm[i]].append(perm[j])
    return adj


class Tests(unittest.TestCase):
    def test_orders_are_valid(self) -> None:
        r = Rng(124)
        for _ in range(60):
            g = _random_dag(r.range(0, 14), r)
            ts = topological_sort(g)
            kn = kahn(g)
            assert ts is not None and kn is not None
            self.assertTrue(is_valid_order(g, ts))
            self.assertTrue(is_valid_order(g, kn))

    def test_cycles_are_rejected_and_paths_counted(self) -> None:
        self.assertIsNone(topological_sort([[1], [2], [0]]))
        self.assertIsNone(kahn([[1], [2], [0]]))
        g = [[1, 2, 3], [2, 3], [3], []]
        self.assertEqual(count_paths(g, 0, 3), 4)
        self.assertIsNone(count_paths([[0]], 0, 0))


if __name__ == "__main__":
    print(demo(), end="")
