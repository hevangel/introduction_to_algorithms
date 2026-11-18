"""Minimum-Cost Flow — CLRS 29.2 (2e–4e); Problem 29-5 (3e–4e).

Send a required amount of flow at minimum total cost by successive shortest augmenting paths in the residual graph (with negative reverse costs).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass
class _Edge:
    to: int
    cap: int
    cost: int


class CostNetwork:
    """Min-cost flow network with paired residual edges (edge e and its reverse e ^ 1, with cost −cost)."""

    def __init__(self, n: int) -> None:
        self.n = n
        self.e: list[_Edge] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def copy(self) -> CostNetwork:
        g = CostNetwork(self.n)
        g.e = [_Edge(ed.to, ed.cap, ed.cost) for ed in self.e]
        g.adj = [row[:] for row in self.adj]
        return g

    def add_edge(self, u: int, v: int, cap: int, cost: int) -> int:
        eid = len(self.e)
        self.e.append(_Edge(v, cap, cost))
        self.adj[u].append(eid)
        self.e.append(_Edge(u, 0, -cost))
        self.adj[v].append(eid + 1)
        return eid

    def flow_on(self, eid: int) -> int:
        return self.e[eid ^ 1].cap

    def min_cost_flow(self, s: int, t: int, want: int) -> tuple[int, int]:
        """Push up to want units from s to t, each along a cheapest residual path."""
        flow = 0
        cost = 0
        inf = 1 << 60
        while flow < want:
            d = [inf] * self.n
            pred = [-1] * self.n
            d[s] = 0
            for _ in range(self.n):
                ch = False
                for u in range(self.n):
                    if d[u] >= inf:
                        continue
                    for eid in self.adj[u]:
                        ed = self.e[eid]
                        if ed.cap > 0 and d[u] + ed.cost < d[ed.to]:
                            d[ed.to] = d[u] + ed.cost
                            pred[ed.to] = eid
                            ch = True
                if not ch:
                    break
            if d[t] >= inf:
                break
            b = want - flow
            v = t
            while v != s:
                eid = pred[v]
                if self.e[eid].cap < b:
                    b = self.e[eid].cap
                v = self.e[eid ^ 1].to
            v = t
            while v != s:
                eid = pred[v]
                self.e[eid].cap -= b
                self.e[eid ^ 1].cap += b
                v = self.e[eid ^ 1].to
            flow += b
            cost += b * d[t]
        return flow, cost


def _hungarian(cost: list[list[int]]) -> int:
    n = len(cost)
    if n == 0:
        return 0
    inf = 1 << 60
    u = [0] * (n + 1)
    v = [0] * (n + 1)
    p = [0] * (n + 1)
    way = [0] * (n + 1)
    for i in range(1, n + 1):
        p[0] = i
        j0 = 0
        minv = [inf] * (n + 1)
        used = [False] * (n + 1)
        while True:
            used[j0] = True
            i0 = p[j0]
            delta = inf
            j1 = 0
            for j in range(1, n + 1):
                if not used[j]:
                    cur = cost[i0 - 1][j - 1] - u[i0] - v[j]
                    if cur < minv[j]:
                        minv[j] = cur
                        way[j] = j0
                    if minv[j] < delta:
                        delta = minv[j]
                        j1 = j
            for j in range(n + 1):
                if used[j]:
                    u[p[j]] += delta
                    v[j] -= delta
                else:
                    minv[j] -= delta
            j0 = j1
            if p[j0] == 0:
                break
        while True:
            j1 = way[j0]
            p[j0] = p[j1]
            j0 = j1
            if j0 == 0:
                break
    return sum(cost[p[j] - 1][j - 1] for j in range(1, n + 1))


def demo() -> str:
    lines: list[str] = []
    g = CostNetwork(4)
    g.add_edge(0, 1, 2, 1)
    g.add_edge(0, 2, 1, 4)
    g.add_edge(1, 2, 1, 1)
    g.add_edge(1, 3, 1, 5)
    g.add_edge(2, 3, 2, 1)
    lines.append(f"send 3 units from s to t: (flow, cost) = {g.copy().min_cost_flow(0, 3, 3)}")
    lines.append(f"send 1 unit  : {g.copy().min_cost_flow(0, 3, 1)}")
    lines.append(f"send 5 units (only 3 fit): {g.copy().min_cost_flow(0, 3, 5)}")
    cost = [[9, 2, 7], [6, 4, 3], [5, 8, 1]]
    a = CostNetwork(8)
    for i in range(3):
        a.add_edge(6, i, 1, 0)
        a.add_edge(3 + i, 7, 1, 0)
        for j in range(3):
            a.add_edge(i, 3 + j, 1, cost[i][j])
    lines.append(f"3×3 assignment as a flow problem: {a.min_cost_flow(6, 7, 3)} (Hungarian: {_hungarian(cost)})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_assignment_via_flow_equals_hungarian(self) -> None:
        r = Rng(372)
        for _ in range(80):
            n = r.range(1, 6)
            c = [r.vec(n, 0, 30) for _ in range(n)]
            g = CostNetwork(2 * n + 2)
            s, t = 2 * n, 2 * n + 1
            for i in range(n):
                g.add_edge(s, i, 1, 0)
                g.add_edge(n + i, t, 1, 0)
                for j in range(n):
                    g.add_edge(i, n + j, 1, c[i][j])
            f, cost = g.min_cost_flow(s, t, n)
            self.assertEqual(f, n)
            self.assertEqual(cost, _hungarian(c))

    def test_small_network_and_flow_conservation(self) -> None:
        g = CostNetwork(4)
        ids = [
            g.add_edge(0, 1, 2, 1),
            g.add_edge(0, 2, 1, 4),
            g.add_edge(1, 2, 1, 1),
            g.add_edge(1, 3, 1, 5),
            g.add_edge(2, 3, 2, 1),
        ]
        self.assertEqual(g.min_cost_flow(0, 3, 3), (3, 3 + 5 + 6))
        out0 = g.flow_on(ids[0]) + g.flow_on(ids[1])
        self.assertEqual(out0, 3)


if __name__ == "__main__":
    print(demo(), end="")
