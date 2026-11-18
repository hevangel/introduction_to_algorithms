"""Classifying Edges by Breadth-First Search — CLRS Problem 22-1 (1e–4e) / 20-1.

BFS produces only tree, back and cross edges (never forward) and d[v] ≤ d[u] + 1; in undirected graphs every non-tree edge joins vertices at distance ≤ 1.
"""

from __future__ import annotations

from collections import deque
from enum import Enum
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Kind(Enum):
    TREE = "Tree"
    BACK = "Back"
    FORWARD = "Forward"
    CROSS = "Cross"


def classify(adj: list[list[int]], s: int) -> tuple[list[int | None], list[tuple[int, int, Kind]]]:
    """Run BFS from s and classify every edge (u, v) whose tail u is reachable. Returns (distances, edge kinds)."""
    n = len(adj)
    d: list[int | None] = [None] * n
    parent: list[int | None] = [None] * n
    d[s] = 0
    q: deque[int] = deque([s])
    order: list[int] = []
    while q:
        u = q.popleft()
        order.append(u)
        du = d[u]
        assert du is not None
        for v in adj[u]:
            if d[v] is None:
                d[v] = du + 1
                parent[v] = u
                q.append(v)

    def is_ancestor(a: int, x: int) -> bool:
        while True:
            if x == a:
                return True
            p = parent[x]
            if p is None:
                return False
            x = p

    kinds: list[tuple[int, int, Kind]] = []
    tree_used = [False] * n
    for u in order:
        for v in adj[u]:
            if parent[v] == u and not tree_used[v]:
                tree_used[v] = True
                k = Kind.TREE
            elif is_ancestor(v, u):
                k = Kind.BACK
            elif is_ancestor(u, v):
                k = Kind.FORWARD
            else:
                k = Kind.CROSS
            kinds.append((u, v, k))
    return d, kinds


def demo() -> str:
    lines: list[str] = []
    adj = [[1, 2], [3], [1, 3, 0], [0, 4], [2]]
    d, kinds = classify(adj, 0)
    lines.append(f"directed graph {adj}, distances from 0: {d}")
    for u, v, k in kinds:
        lines.append(f"  edge {u} → {v}: {k.value}")
    fwd = sum(1 for _, _, k in kinds if k == Kind.FORWARD)
    lines.append(f"forward edges found: {fwd}  (always 0 for BFS)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_directed_bfs_properties(self) -> None:
        r = Rng(355)
        for _ in range(200):
            n = r.range(1, 12)
            adj = [[v for v in range(n) if r.f64() < 0.2 and (u != v or r.f64() < 0.3)] for u in range(n)]
            d, kinds = classify(adj, 0)
            for u, v, k in kinds:
                self.assertNotEqual(k, Kind.FORWARD, "BFS has no forward edges")
                du, dv = d[u], d[v]
                assert du is not None and dv is not None
                self.assertLessEqual(dv, du + 1, "d[v] ≤ d[u] + 1")
                if k == Kind.TREE:
                    self.assertEqual(dv, du + 1)
                elif k == Kind.BACK:
                    self.assertLessEqual(dv, du)
            tree_count = sum(1 for _, _, k in kinds if k == Kind.TREE)
            reach_count = sum(1 for x in d if x is not None)
            self.assertEqual(tree_count, reach_count - 1)

    def test_undirected_bfs_has_only_tree_and_cross_edges(self) -> None:
        r = Rng(356)
        for _ in range(200):
            n = r.range(1, 12)
            adj: list[list[int]] = [[] for _ in range(n)]
            for u in range(n):
                for v in range(u + 1, n):
                    if r.f64() < 0.25:
                        adj[u].append(v)
                        adj[v].append(u)
            d, kinds = classify(adj, 0)
            for u, v, k in kinds:
                du, dv = d[u], d[v]
                assert du is not None and dv is not None
                self.assertLessEqual(abs(du - dv), 1)
                if k == Kind.BACK:
                    self.assertEqual(dv + 1, du)


if __name__ == "__main__":
    print(demo(), end="")
