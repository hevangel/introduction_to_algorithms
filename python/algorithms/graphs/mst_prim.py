"""Prim's Minimum Spanning Tree — CLRS 23.2 (3e) / 21 (4e) / 24 (1e–2e).

Grow one tree from a root, always adding the lightest edge leaving it (min-priority queue).
"""

from __future__ import annotations

import heapq
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

WeightedAdj = list[list[tuple[int, int]]]


def prim(adj: WeightedAdj, root: int) -> tuple[int, list[int | None]]:
    """MST-PRIM with a binary heap (lazy deletion): O(E lg V). Returns (total weight, parent array)."""
    n = len(adj)
    inf = 1 << 60
    key = [inf] * n
    parent: list[int | None] = [None] * n
    in_tree = [False] * n
    key[root] = 0
    q: list[tuple[int, int]] = [(0, root)]
    total = 0
    while q:
        k, u = heapq.heappop(q)
        if in_tree[u] or k > key[u]:
            continue
        in_tree[u] = True
        total += k
        for v, w in adj[u]:
            if not in_tree[v] and w < key[v]:
                key[v] = w
                parent[v] = u
                heapq.heappush(q, (w, v))
    return total, parent


def prim_array(adj: WeightedAdj, root: int) -> int:
    """Array-based Prim, Θ(V²) — best for dense graphs."""
    n = len(adj)
    inf = 1 << 60
    key = [inf] * n
    in_tree = [False] * n
    key[root] = 0
    total = 0
    for _ in range(n):
        u = -1
        best = inf
        for v in range(n):
            if not in_tree[v] and key[v] < best:
                best = key[v]
                u = v
        if u == -1:
            break
        in_tree[u] = True
        total += key[u]
        for v, w in adj[u]:
            if not in_tree[v] and w < key[v]:
                key[v] = w
    return total


def adjacency(n: int, edges: list[tuple[int, int, int]]) -> WeightedAdj:
    a: WeightedAdj = [[] for _ in range(n)]
    for u, v, w in edges:
        a[u].append((v, w))
        a[v].append((u, w))
    return a


def _kruskal_weight(n: int, edges: list[tuple[int, int, int]]) -> int:
    p = list(range(n))

    def find(x: int) -> int:
        while p[x] != x:
            p[x] = p[p[x]]
            x = p[x]
        return x

    total = 0
    for u, v, w in sorted(edges, key=lambda e: e[2]):
        a, b = find(u), find(v)
        if a != b:
            p[a] = b
            total += w
    return total


def demo() -> str:
    lines: list[str] = []
    e = [
        (0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2),
        (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7),
    ]
    adj = adjacency(9, e)
    w, parent = prim(adj, 0)
    lines.append(f"Prim (heap) total weight  = {w}")
    lines.append(f"Prim (array) total weight = {prim_array(adj, 0)}")
    lines.append(f"parent array (root 0): {parent}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_kruskal_on_random_connected_graphs(self) -> None:
        r = Rng(129)
        for _ in range(60):
            n = r.range(1, 20)
            edges = [(r.range(0, v - 1), v, r.range(1, 50)) for v in range(1, n)]
            for _ in range(r.range(0, 40)):
                a, b = r.range(0, n - 1), r.range(0, n - 1)
                if a != b:
                    edges.append((a, b, r.range(1, 50)))
            adj = adjacency(n, edges)
            k = _kruskal_weight(n, edges)
            root = r.range(0, n - 1)
            self.assertEqual(prim(adj, root)[0], k)
            self.assertEqual(prim_array(adj, root), k)

    def test_parent_pointers_form_a_tree(self) -> None:
        e = [
            (0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2),
            (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7),
        ]
        w, parent = prim(adjacency(9, e), 0)
        self.assertEqual(w, 37)
        self.assertEqual(sum(1 for p in parent if p is None), 1)
        for v in range(9):
            x = v
            steps = 0
            while parent[x] is not None:
                x = parent[x]  # type: ignore[assignment]
                steps += 1
                self.assertLess(steps, 9)
            self.assertEqual(x, 0)


if __name__ == "__main__":
    print(demo(), end="")
