"""Shortest Paths, Max Flow & Min-Cost Flow as Linear Programs — CLRS 29.2 (2e–4e) (2, 3, 4).

Section 29.2's formulations solved with this repository's simplex and checked against Dijkstra, Edmonds-Karp and successive shortest paths.
"""

from __future__ import annotations

from collections import deque
from dataclasses import dataclass
import heapq
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Edge = tuple[int, int, int]
_EPS = 1e-9
_INF = 1 << 60


@dataclass(eq=True)
class _LpOptimal:
    value: float
    x: list[float]


@dataclass(eq=True)
class _LpUnbounded:
    pass


@dataclass(eq=True)
class _LpInfeasible:
    pass


_LpResult = _LpOptimal | _LpUnbounded | _LpInfeasible


class _Lp:
    def __init__(self, a: list[list[float]], b: list[float], c: list[float]) -> None:
        m, n = len(b), len(c)
        d = [[0.0] * (n + 2) for _ in range(m + 2)]
        for i in range(m):
            for j in range(n):
                d[i][j] = a[i][j]
            d[i][n] = -1.0
            d[i][n + 1] = b[i]
        self.m = m
        self.n = n
        self.basic = [n + i for i in range(m)]
        self.nonbasic = list(range(n)) + [-1]
        for j in range(n):
            d[m][j] = -c[j]
        d[m + 1][n] = 1.0
        self.d = d

    def pivot(self, r: int, s: int) -> None:
        inv = 1.0 / self.d[r][s]
        for i in range(self.m + 2):
            if i != r:
                for j in range(self.n + 2):
                    if j != s:
                        self.d[i][j] -= self.d[r][j] * self.d[i][s] * inv
        for j in range(self.n + 2):
            if j != s:
                self.d[r][j] *= inv
        for i in range(self.m + 2):
            if i != r:
                self.d[i][s] *= -inv
        self.d[r][s] = inv
        self.basic[r], self.nonbasic[s] = self.nonbasic[s], self.basic[r]

    def run(self, phase: int) -> bool:
        x = self.m + 1 if phase == 1 else self.m
        while True:
            s: int | None = None
            for j in range(self.n + 1):
                if phase == 2 and self.nonbasic[j] == -1:
                    continue
                if self.d[x][j] < -_EPS and (s is None or self.nonbasic[j] < self.nonbasic[s]):
                    s = j
            if s is None:
                return True
            r: int | None = None
            for i in range(self.m):
                if self.d[i][s] < _EPS:
                    continue
                if r is None:
                    r = i
                else:
                    a = self.d[i][self.n + 1] / self.d[i][s]
                    b = self.d[r][self.n + 1] / self.d[r][s]
                    if a < b - _EPS or (abs(a - b) <= _EPS and self.basic[i] < self.basic[r]):
                        r = i
            if r is None:
                return False
            self.pivot(r, s)

    def solve(self) -> _LpResult:
        m, n = self.m, self.n
        r = min(range(m), key=lambda i: self.d[i][n + 1])
        if self.d[r][n + 1] < -_EPS:
            self.pivot(r, n)
            if not self.run(1) or self.d[m + 1][n + 1] < -_EPS:
                return _LpInfeasible()
            for i in range(m):
                if self.basic[i] == -1:
                    s = min(range(n + 1), key=lambda col: (self.d[i][col], self.nonbasic[col]))
                    self.pivot(i, s)
        if not self.run(2):
            return _LpUnbounded()
        x = [0.0] * n
        for i in range(m):
            if 0 <= self.basic[i] < n:
                x[self.basic[i]] = self.d[i][n + 1]
        return _LpOptimal(value=self.d[m][n + 1], x=x)


def _simplex(a: list[list[float]], b: list[float], c: list[float]) -> _LpResult:
    assert len(b) > 0
    return _Lp(a, b, c).solve()


def _solve_lp(
    n_vars: int,
    eq: list[tuple[list[float], float]],
    le: list[tuple[list[float], float]],
    c: list[float],
) -> _LpResult:
    del n_vars
    a: list[list[float]] = []
    b: list[float] = []
    for row, rhs in eq:
        a.append(list(row))
        b.append(rhs)
        a.append([-x for x in row])
        b.append(-rhs)
    for row, rhs in le:
        a.append(list(row))
        b.append(rhs)
    return _simplex(a, b, c)


def _conservation(n: int, edges: list[Edge], v: int) -> list[float]:
    del n
    return [float(int(a == v) - int(b == v)) for a, b, _ in edges]


def shortest_path_lp(n: int, edges: list[Edge], s: int, t: int) -> float | None:
    """Shortest-path weight s ~> t as an LP: minimise sum w_e x_e with one unit of flow from s to t."""
    eq = [
        (_conservation(n, edges, v), 1.0 if v == s else (-1.0 if v == t else 0.0))
        for v in range(n)
    ]
    c = [-float(e[2]) for e in edges]
    res = _solve_lp(len(edges), eq, [], c)
    if isinstance(res, _LpOptimal):
        return -res.value
    return None


def max_flow_lp(n: int, edges: list[Edge], s: int, t: int) -> float:
    """Maximum flow as an LP: maximise flow out of s subject to capacities and conservation."""
    eq = [(_conservation(n, edges, v), 0.0) for v in range(n) if v != s and v != t]
    le: list[tuple[list[float], float]] = []
    for i, e in enumerate(edges):
        row = [0.0] * len(edges)
        row[i] = 1.0
        le.append((row, float(e[2])))
    c = [float(int(e[0] == s) - int(e[1] == s)) for e in edges]
    res = _solve_lp(len(edges), eq, le, c)
    if isinstance(res, _LpOptimal):
        return res.value
    return math.nan


def min_cost_flow_lp(
    n: int, edges: list[Edge], cost: list[int], s: int, t: int, d: int
) -> float | None:
    """Minimum-cost flow of value d: edges are (u, v, capacity); cost[i] is unit cost of edge i."""
    eq = [
        (_conservation(n, edges, v), float(d) if v == s else (-float(d) if v == t else 0.0))
        for v in range(n)
    ]
    le: list[tuple[list[float], float]] = []
    for i, e in enumerate(edges):
        row = [0.0] * len(edges)
        row[i] = 1.0
        le.append((row, float(e[2])))
    c = [-float(x) for x in cost]
    res = _solve_lp(len(edges), eq, le, c)
    if isinstance(res, _LpOptimal):
        return -res.value
    return None


def demo() -> str:
    lines: list[str] = []
    sp = [
        (0, 1, 10),
        (0, 3, 5),
        (1, 2, 1),
        (1, 3, 2),
        (2, 4, 4),
        (3, 1, 3),
        (3, 2, 9),
        (3, 4, 2),
        (4, 0, 7),
        (4, 2, 6),
    ]
    lines.append(f"shortest path 0 ⇝ 2 as an LP: {shortest_path_lp(5, sp, 0, 2)}  (Dijkstra: 9)")
    fl = [
        (0, 1, 16),
        (0, 2, 13),
        (1, 3, 12),
        (2, 1, 4),
        (2, 4, 14),
        (3, 2, 9),
        (3, 5, 20),
        (4, 3, 7),
        (4, 5, 4),
    ]
    lines.append(f"max flow of the CLRS network as an LP: {max_flow_lp(6, fl, 0, 5)}  (Edmonds–Karp: 23)")
    net = [(0, 1, 2), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 2)]
    lines.append(f"min-cost flow of 3 units as an LP: {min_cost_flow_lp(4, net, [1, 4, 1, 5, 1], 0, 3, 3)}")
    return "\n".join(lines) + "\n"


def _dijkstra(adj: list[list[tuple[int, int]]], s: int) -> list[int]:
    n = len(adj)
    d = [_INF] * n
    d[s] = 0
    q: list[tuple[int, int]] = [(0, s)]
    while q:
        du, u = heapq.heappop(q)
        if du > d[u]:
            continue
        for v, w in adj[u]:
            if du + w < d[v]:
                d[v] = du + w
                heapq.heappush(q, (d[v], v))
    return d


class _FlowNetwork:
    def __init__(self, n: int) -> None:
        self.n = n
        self.to: list[int] = []
        self.cap: list[int] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def add_edge(self, u: int, v: int, c: int) -> int:
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(c)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.adj[v].append(eid + 1)
        return eid

    def edmonds_karp(self, s: int, t: int) -> int:
        flow = 0
        while True:
            pred = [-1] * self.n
            seen = [False] * self.n
            seen[s] = True
            q = deque([s])
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
            b = _INF
            v = t
            while v != s:
                e = pred[v]
                b = min(b, self.cap[e])
                v = self.to[e ^ 1]
            v = t
            while v != s:
                e = pred[v]
                self.cap[e] -= b
                self.cap[e ^ 1] += b
                v = self.to[e ^ 1]
            flow += b


class _CostNetwork:
    def __init__(self, n: int) -> None:
        self.n = n
        self.to: list[int] = []
        self.cap: list[int] = []
        self.cost: list[int] = []
        self.adj: list[list[int]] = [[] for _ in range(n)]

    def add_edge(self, u: int, v: int, cap: int, cost: int) -> int:
        eid = len(self.to)
        self.to.append(v)
        self.cap.append(cap)
        self.cost.append(cost)
        self.adj[u].append(eid)
        self.to.append(u)
        self.cap.append(0)
        self.cost.append(-cost)
        self.adj[v].append(eid + 1)
        return eid

    def min_cost_flow(self, s: int, t: int, want: int) -> tuple[int, int]:
        flow = 0
        total_cost = 0
        while flow < want:
            d = [_INF] * self.n
            pred = [-1] * self.n
            d[s] = 0
            for _ in range(self.n):
                ch = False
                for u in range(self.n):
                    if d[u] >= _INF:
                        continue
                    for eid in self.adj[u]:
                        if self.cap[eid] > 0 and d[u] + self.cost[eid] < d[self.to[eid]]:
                            d[self.to[eid]] = d[u] + self.cost[eid]
                            pred[self.to[eid]] = eid
                            ch = True
                if not ch:
                    break
            if d[t] >= _INF:
                break
            b = want - flow
            v = t
            while v != s:
                eid = pred[v]
                b = min(b, self.cap[eid])
                v = self.to[eid ^ 1]
            v = t
            while v != s:
                eid = pred[v]
                self.cap[eid] -= b
                self.cap[eid ^ 1] += b
                v = self.to[eid ^ 1]
            flow += b
            total_cost += b * d[t]
        return (flow, total_cost)


class Tests(unittest.TestCase):
    def test_shortest_paths_match_dijkstra(self) -> None:
        r = Rng(380)
        for _ in range(60):
            n = r.range(2, 6)
            edges = [
                (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 9))
                for _ in range(r.range(1, 12))
            ]
            edges = [e for e in edges if e[0] != e[1]]
            adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
            for u, v, w in edges:
                adj[u].append((v, w))
            d = _dijkstra(adj, 0)
            t = n - 1
            res = shortest_path_lp(n, edges, 0, t)
            if res is not None:
                self.assertLess(d[t], _INF)
                self.assertLess(abs(res - float(d[t])), 1e-6)
            else:
                self.assertTrue(d[t] >= _INF or t == 0)

    def test_max_flow_matches_edmonds_karp(self) -> None:
        r = Rng(381)
        for _ in range(60):
            n = r.range(2, 6)
            edges = [
                (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 9))
                for _ in range(r.range(1, 12))
            ]
            edges = [e for e in edges if e[0] != e[1]]
            if not edges:
                continue
            g = _FlowNetwork(n)
            for u, v, c in edges:
                g.add_edge(u, v, c)
            want = float(g.edmonds_karp(0, n - 1))
            self.assertLess(abs(max_flow_lp(n, edges, 0, n - 1) - want), 1e-6)
        fl = [
            (0, 1, 16),
            (0, 2, 13),
            (1, 3, 12),
            (2, 1, 4),
            (2, 4, 14),
            (3, 2, 9),
            (3, 5, 20),
            (4, 3, 7),
            (4, 5, 4),
        ]
        self.assertLess(abs(max_flow_lp(6, fl, 0, 5) - 23.0), 1e-6)

    def test_min_cost_flow_matches_successive_shortest_paths(self) -> None:
        r = Rng(382)
        for _ in range(60):
            n = r.range(2, 6)
            edges = [
                (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 5))
                for _ in range(r.range(1, 12))
            ]
            edges = [e for e in edges if e[0] != e[1]]
            if not edges:
                continue
            cost = [r.range(0, 9) for _ in edges]
            d = r.range(1, 4)
            g = _CostNetwork(n)
            for i, (u, v, c) in enumerate(edges):
                g.add_edge(u, v, c, cost[i])
            sent, c_val = g.min_cost_flow(0, n - 1, d)
            lp = min_cost_flow_lp(n, edges, cost, 0, n - 1, d)
            if sent == d:
                assert lp is not None
                self.assertLess(abs(lp - float(c_val)), 1e-6)
            else:
                self.assertIsNone(lp)


if __name__ == "__main__":
    print(demo(), end="")
