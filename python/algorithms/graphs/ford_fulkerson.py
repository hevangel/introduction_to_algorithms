"""Ford-Fulkerson & Edmonds-Karp Maximum Flow — CLRS 26.1–26.2 (1e–3e) / 24.1–24.2 (4e).

Augment along residual paths: DFS (Ford-Fulkerson) or BFS shortest paths (Edmonds-Karp, O(VE²)); min-cut extraction.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class FlowNetwork:
    """Flow network with paired residual edges (edge e and its reverse e ^ 1)."""

    def __init__(self, n: int) -> None:
        self.n = n
        self.to: list[int] = []
        self.cap: list[int] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def copy(self) -> FlowNetwork:
        g = FlowNetwork(self.n)
        g.to = self.to[:]
        g.cap = self.cap[:]
        g.adj = [row[:] for row in self.adj]
        return g

    def add_edge(self, u: int, v: int, c: int) -> int:
        """Add edge u → v with capacity c; returns the edge id (the reverse edge is id + 1)."""
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(c)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.adj[v].append(eid + 1)
        return eid

    def flow_on(self, eid: int) -> int:
        """Flow currently pushed through edge id (= residual capacity of its reverse edge)."""
        return self.cap[eid ^ 1]

    def capacity_of(self, eid: int) -> int:
        return self.cap[eid] + self.cap[eid ^ 1]

    def edmonds_karp(self, s: int, t: int) -> int:
        """EDMONDS-KARP: always augment along a shortest (fewest-edge) residual path — O(V·E²)."""
        flow = 0
        while True:
            pred = [-1] * self.n
            seen = [False] * self.n
            seen[s] = True
            q: deque[int] = deque([s])
            while q:
                u = q.popleft()
                for e in self.adj[u]:
                    v = self.to[e]
                    if not seen[v] and self.cap[e] > 0:
                        seen[v] = True
                        pred[v] = e
                        q.append(v)
            if not seen[t]:
                return flow
            b = 1 << 60
            v = t
            while v != s:
                e = pred[v]
                if self.cap[e] < b:
                    b = self.cap[e]
                v = self.to[e ^ 1]
            v = t
            while v != s:
                e = pred[v]
                self.cap[e] -= b
                self.cap[e ^ 1] += b
                v = self.to[e ^ 1]
            flow += b

    def ford_fulkerson_dfs(self, s: int, t: int) -> int:
        """FORD-FULKERSON with depth-first augmenting paths."""
        def dfs(u: int, f: int, seen: list[bool]) -> int:
            if u == t:
                return f
            seen[u] = True
            for e in self.adj[u]:
                v = self.to[e]
                if not seen[v] and self.cap[e] > 0:
                    d = dfs(v, min(f, self.cap[e]), seen)
                    if d > 0:
                        self.cap[e] -= d
                        self.cap[e ^ 1] += d
                        return d
            return 0

        flow = 0
        while True:
            seen = [False] * self.n
            f = dfs(s, 1 << 60, seen)
            if f == 0:
                return flow
            flow += f

    def min_cut_source_side(self, s: int) -> list[bool]:
        """After a maximum flow: vertices reachable from s in the residual graph."""
        seen = [False] * self.n
        seen[s] = True
        st = [s]
        while st:
            u = st.pop()
            for e in self.adj[u]:
                v = self.to[e]
                if not seen[v] and self.cap[e] > 0:
                    seen[v] = True
                    st.append(v)
        return seen

    def cut_capacity(self, side: list[bool]) -> int:
        """Capacity of the cut (S, V−S) in the original network."""
        return sum(self.capacity_of(e) for e in range(0, len(self.to), 2) if side[self.to[e ^ 1]] and not side[self.to[e]])

    def net_flow(self, u: int) -> int:
        """Net flow out of vertex u (0 for internal vertices — flow conservation)."""
        return sum(self.flow_on(e) if e % 2 == 0 else -self.flow_on(e ^ 1) for e in self.adj[u])


def clrs_network() -> FlowNetwork:
    """Figure 26.1: a shipping network. s=0, v1=1, v2=2, v3=3, v4=4, t=5."""
    g = FlowNetwork(6)
    for u, v, c in [(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)]:
        g.add_edge(u, v, c)
    return g


def demo() -> str:
    lines: list[str] = []
    a = clrs_network()
    lines.append(f"Edmonds-Karp max flow       = {a.edmonds_karp(0, 5)}")
    side = a.min_cut_source_side(0)
    lines.append(f"min cut: S = {[v for v in range(6) if side[v]]}, capacity = {a.cut_capacity(side)}")
    b = clrs_network()
    lines.append(f"Ford-Fulkerson (DFS) value  = {b.ford_fulkerson_dfs(0, 5)}")
    lines.append(f"flow conservation: net flow at v1..v4 = {[a.net_flow(v) for v in range(1, 5)]}")
    return "\n".join(lines) + "\n"


def _random_network(r: Rng) -> tuple[FlowNetwork, int]:
    n = r.range(2, 9)
    g = FlowNetwork(n)
    for _ in range(r.range(1, 25)):
        u, v = r.range(0, n - 1), r.range(0, n - 1)
        if u != v:
            g.add_edge(u, v, r.range(1, 20))
    return g, n


class Tests(unittest.TestCase):
    def test_book_value_23(self) -> None:
        self.assertEqual(clrs_network().edmonds_karp(0, 5), 23)
        self.assertEqual(clrs_network().ford_fulkerson_dfs(0, 5), 23)

    def test_max_flow_min_cut_theorem_and_conservation(self) -> None:
        r = Rng(138)
        for _ in range(80):
            g, n = _random_network(r)
            a, b = g.copy(), g.copy()
            f = a.edmonds_karp(0, n - 1)
            self.assertEqual(f, b.ford_fulkerson_dfs(0, n - 1))
            side = a.min_cut_source_side(0)
            self.assertFalse(side[n - 1])
            self.assertEqual(a.cut_capacity(side), f)
            for v in range(1, n - 1):
                self.assertEqual(a.net_flow(v), 0)
            self.assertEqual(a.net_flow(0), f)
            for e in range(0, len(a.to), 2):
                self.assertGreaterEqual(a.flow_on(e), 0)
                self.assertLessEqual(a.flow_on(e), a.capacity_of(e))


if __name__ == "__main__":
    print(demo(), end="")
