"""Minimum Path Cover of a DAG — CLRS Problem 26-2 (1e–4e).

The fewest vertex-disjoint paths covering a DAG equals |V| − (maximum bipartite matching); with shared vertices allowed it equals the largest antichain (Dilworth).
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _hopcroft_karp(left: int, right: int, adj: list[list[int]]) -> tuple[int, list[int], list[int]]:
    nil = -1
    inf = 1 << 60
    ml = [nil] * left
    mr = [nil] * right
    size = 0

    def dfs(l: int, dist: list[int]) -> bool:
        for r in adj[l]:
            l2 = mr[r]
            if l2 == nil or (dist[l2] == dist[l] + 1 and dfs(l2, dist)):
                ml[l] = r
                mr[r] = l
                return True
        dist[l] = inf
        return False

    while True:
        dist = [inf] * left
        q: deque[int] = deque()
        for l in range(left):
            if ml[l] == nil:
                dist[l] = 0
                q.append(l)
        found = False
        while q:
            l = q.popleft()
            for r in adj[l]:
                l2 = mr[r]
                if l2 == nil:
                    found = True
                elif dist[l2] == inf:
                    dist[l2] = dist[l] + 1
                    q.append(l2)
        if not found:
            break
        for l in range(left):
            if ml[l] == nil and dfs(l, dist):
                size += 1
    return size, ml, mr


def min_disjoint_path_cover(n: int, edges: list[tuple[int, int]]) -> list[list[int]]:
    """Minimum number of vertex-disjoint paths that cover all vertices of a DAG, with the paths themselves."""
    adj: list[list[int]] = [[] for _ in range(n)]
    for u, v in edges:
        adj[u].append(v)
    _, nxt, _ = _hopcroft_karp(n, n, adj)
    has_pred = [False] * n
    for u in range(n):
        if nxt[u] != -1:
            has_pred[nxt[u]] = True
    paths: list[list[int]] = []
    for start in range(n):
        if not has_pred[start]:
            path = [start]
            x = start
            while nxt[x] != -1:
                x = nxt[x]
                path.append(x)
            paths.append(path)
    return paths


def min_path_cover_with_overlap(n: int, edges: list[tuple[int, int]]) -> int:
    """Minimum number of paths covering all vertices when paths may share vertices (Exercise 26-2(c))."""
    reach = [[False] * n for _ in range(n)]
    for u, v in edges:
        reach[u][v] = True
    for k in range(n):
        for i in range(n):
            if reach[i][k]:
                for j in range(n):
                    if reach[k][j]:
                        reach[i][j] = True
    closure = [(i, j) for i in range(n) for j in range(n) if reach[i][j]]
    return len(min_disjoint_path_cover(n, closure))


def demo() -> str:
    lines: list[str] = []
    e = [(0, 1), (1, 2), (0, 3), (3, 2), (2, 4), (5, 3)]
    cover = min_disjoint_path_cover(6, e)
    lines.append(f"DAG edges {e}")
    lines.append(f"minimum vertex-disjoint path cover ({len(cover)} paths): {cover}")
    lines.append(f"minimum cover when paths may overlap: {min_path_cover_with_overlap(6, e)} (= size of the largest antichain)")
    return "\n".join(lines) + "\n"


def _random_dag(n: int, r: Rng) -> list[tuple[int, int]]:
    perm = list(range(n))
    r.shuffle(perm)
    return [(perm[i], perm[j]) for i in range(n) for j in range(i + 1, n) if r.f64() < 0.35]


def _topological_sort(adj: list[list[int]]) -> list[int]:
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
    return order


def _brute_disjoint(n: int, edge_set: set[tuple[int, int]], order: list[int], pos: int, ends: list[int]) -> int:
    if pos == n:
        return len(ends)
    v = order[pos]
    ends.append(v)
    best = _brute_disjoint(n, edge_set, order, pos + 1, ends)
    ends.pop()
    for i in range(len(ends)):
        if (ends[i], v) in edge_set:
            old = ends[i]
            ends[i] = v
            cand = _brute_disjoint(n, edge_set, order, pos + 1, ends)
            if cand < best:
                best = cand
            ends[i] = old
    return best


class Tests(unittest.TestCase):
    def test_disjoint_cover_is_valid_and_optimal(self) -> None:
        r = Rng(368)
        for _ in range(100):
            n = r.range(1, 7)
            e = _random_dag(n, r)
            cover = min_disjoint_path_cover(n, e)
            seen = sorted(v for p in cover for v in p)
            self.assertEqual(seen, list(range(n)))
            edge_set = set(e)
            for p in cover:
                for i in range(len(p) - 1):
                    self.assertIn((p[i], p[i + 1]), edge_set)
            adj: list[list[int]] = [[] for _ in range(n)]
            for u, v in e:
                adj[u].append(v)
            topo = _topological_sort(adj)
            self.assertEqual(len(cover), _brute_disjoint(n, edge_set, topo, 0, []))

    def test_overlapping_cover_equals_maximum_antichain(self) -> None:
        r = Rng(369)
        for _ in range(100):
            n = r.range(1, 8)
            e = _random_dag(n, r)
            reach = [[False] * n for _ in range(n)]
            for u, v in e:
                reach[u][v] = True
            for k in range(n):
                for i in range(n):
                    if reach[i][k]:
                        for j in range(n):
                            if reach[k][j]:
                                reach[i][j] = True
            antichain = max(
                m.bit_count()
                for m in range(1 << n)
                if all(
                    i == j or not ((m >> i) & 1) or not ((m >> j) & 1) or not reach[i][j]
                    for i in range(n)
                    for j in range(n)
                )
            )
            self.assertEqual(min_path_cover_with_overlap(n, e), antichain)


if __name__ == "__main__":
    print(demo(), end="")
