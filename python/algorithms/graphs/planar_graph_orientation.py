"""Dynamic Planar Graph Low-Outdegree Orientation — CLRS Problem 20-5 (4e).

Orient edges dynamically during incremental planar vertex insertion so out-degree stays ≤ 5 and adjacency queries run in O(1) time.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class DynamicPlanarGraph:
    """Dynamic simple planar graph with invariant out_degree(v) <= 5 for every vertex."""

    def __init__(self, n: int) -> None:
        self.out_adj: list[list[int]] = [[] for _ in range(n)]
        self.reorientations = 0

    def num_vertices(self) -> int:
        return len(self.out_adj)

    def out_degree(self, u: int) -> int:
        return len(self.out_adj[u])

    def max_out_degree(self) -> int:
        return max((len(a) for a in self.out_adj), default=0)

    def reorientation_count(self) -> int:
        return self.reorientations

    def has_edge(self, u: int, v: int) -> bool:
        """O(1) adjacency test: checks at most 5 outgoing arcs from u and at most 5 from v."""
        return v in self.out_adj[u] or u in self.out_adj[v]

    def add_edge(self, u: int, v: int) -> None:
        """Insert an undirected edge {u, v} oriented outward from the lower-outdegree endpoint."""
        if u == v or self.has_edge(u, v):
            return
        src, dst = (u, v) if len(self.out_adj[u]) <= len(self.out_adj[v]) else (v, u)
        self.add_directed_edge(src, dst)

    def add_directed_edge(self, src: int, dst: int) -> None:
        """Add edge src -> dst and trigger path-reversal rebalancing if out_degree(src) > 5."""
        if src == dst or self.has_edge(src, dst):
            return
        self.out_adj[src].append(dst)
        if len(self.out_adj[src]) > 5:
            self._rebalance(src)

    def add_vertex_with_edges(self, neighbors: list[int]) -> int:
        """Insert a new vertex connected to neighbors in the current planar graph, returning its vertex id."""
        v = len(self.out_adj)
        self.out_adj.append([])
        for u in neighbors:
            self.add_edge(v, u)
        return v

    def _rebalance(self, overloaded: int) -> None:
        n = len(self.out_adj)
        parent = [-1] * n
        q: deque[int] = deque([overloaded])
        parent[overloaded] = overloaded
        target: int | None = None

        while q:
            x = q.popleft()
            if x != overloaded and len(self.out_adj[x]) < 5:
                target = x
                break
            for y in self.out_adj[x]:
                if parent[y] == -1:
                    parent[y] = x
                    q.append(y)

        assert target is not None, "planar graph must admit out-degree <= 5 orientation"
        cur = target
        while cur != overloaded:
            prev = parent[cur]
            adj_prev = self.out_adj[prev]
            pos = adj_prev.index(cur)
            adj_prev[pos] = adj_prev[-1]
            adj_prev.pop()
            self.out_adj[cur].append(prev)
            self.reorientations += 1
            cur = prev


def orient_planar_static(n: int, edges: list[tuple[int, int]]) -> DynamicPlanarGraph:
    """Static 5-degeneracy linear-time orientation for a planar graph given all at once."""
    adj: list[list[int]] = [[] for _ in range(n)]
    for u, v in edges:
        if u != v and v not in adj[u]:
            adj[u].append(v)
            adj[v].append(u)
    deg = [len(a) for a in adj]
    removed = [False] * n
    stack = [u for u in range(n) if deg[u] <= 5]
    out_adj: list[list[int]] = [[] for _ in range(n)]
    peeled = 0

    while stack:
        u = stack.pop()
        if removed[u]:
            continue
        removed[u] = True
        peeled += 1
        for v in adj[u]:
            if not removed[v]:
                out_adj[u].append(v)
                deg[v] -= 1
                if deg[v] <= 5:
                    stack.append(v)
    assert peeled == n, "input graph is not 5-degenerate (non-planar)"
    g = DynamicPlanarGraph(0)
    g.out_adj = out_adj
    return g


def demo() -> str:
    lines: list[str] = []
    g = DynamicPlanarGraph(6)
    oct_edges = [
        (0, 1), (1, 2), (2, 0),
        (3, 4), (4, 5), (5, 3),
        (0, 3), (0, 5), (1, 3), (1, 4), (2, 4), (2, 5),
    ]
    for u, v in oct_edges:
        g.add_edge(u, v)
    hub = g.add_vertex_with_edges([0, 1, 2, 3, 4, 5])
    lines.append(
        f"planar graph vertices = {g.num_vertices()}, max out-degree = {g.max_out_degree()}, "
        f"edge flips = {g.reorientation_count()}"
    )
    lines.append(f"O(1) query has_edge({hub}, 4) = {g.has_edge(hub, 4)}, has_edge(0, 4) = {g.has_edge(0, 4)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_incremental_triangulation_preserves_out_deg_le_5(self) -> None:
        r = Rng(2005)
        g = DynamicPlanarGraph(4)
        faces = [(0, 1, 2), (0, 1, 3), (0, 2, 3), (1, 2, 3)]
        all_edges: list[tuple[int, int]] = []
        for u in range(4):
            for v in range(u + 1, 4):
                g.add_edge(u, v)
                all_edges.append((u, v))
        for _ in range(80):
            fi = r.range(0, len(faces) - 1)
            a, b, c = faces[fi]
            faces[fi] = faces[-1]
            faces.pop()
            nv = g.add_vertex_with_edges([a, b, c])
            all_edges.extend([(nv, a), (nv, b), (nv, c)])
            faces.extend([(a, b, nv), (b, c, nv), (c, a, nv)])
            self.assertLessEqual(g.max_out_degree(), 5)
        for u, v in all_edges:
            self.assertTrue(g.has_edge(u, v))
            self.assertTrue(g.has_edge(v, u))
        stat = orient_planar_static(g.num_vertices(), all_edges)
        self.assertLessEqual(stat.max_out_degree(), 5)
        for u, v in all_edges:
            self.assertTrue(stat.has_edge(u, v))

    def test_forced_reorientation_restores_bound(self) -> None:
        g = DynamicPlanarGraph(11)
        for rim in range(1, 11):
            g.add_directed_edge(0, rim)
            self.assertLessEqual(g.max_out_degree(), 5)
        self.assertGreaterEqual(g.reorientation_count(), 5)
        for rim in range(1, 11):
            self.assertTrue(g.has_edge(0, rim))
            self.assertTrue(g.has_edge(rim, 0))


if __name__ == "__main__":
    print(demo(), end="")
