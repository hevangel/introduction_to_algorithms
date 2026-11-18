"""Weighted Vertex Cover by LP Rounding — CLRS 35.4 (2e–4e) (2,3,4).

Solve the LP relaxation (x_u + x_v ≥ 1), then round every x_v ≥ ½ up: a 2-approximation for weighted cover.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_EPS = 1e-9


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

    def solve(self) -> tuple[float, list[float]]:
        m, n = self.m, self.n
        r = min(range(m), key=lambda i: self.d[i][n + 1])
        if self.d[r][n + 1] < -_EPS:
            self.pivot(r, n)
            if not self.run(1) or self.d[m + 1][n + 1] < -_EPS:
                raise RuntimeError("LP infeasible")
            for i in range(m):
                if self.basic[i] == -1:
                    s = min(range(n + 1), key=lambda col: (self.d[i][col], self.nonbasic[col]))
                    self.pivot(i, s)
        if not self.run(2):
            raise RuntimeError("LP unbounded")
        x = [0.0] * n
        for i in range(m):
            if 0 <= self.basic[i] < n:
                x[self.basic[i]] = self.d[i][n + 1]
        return (self.d[m][n + 1], x)


def vertex_cover_lp(n: int, edges: list[tuple[int, int]], w: list[float]) -> tuple[float, list[float]]:
    """Solve min sum w_v x_v s.t. x_u + x_v >= 1, 0 <= x <= 1. Returns (LP opt, x)."""
    a: list[list[float]] = []
    b: list[float] = []
    for u, v in edges:
        row = [0.0] * n
        row[u] = -1.0
        row[v] = -1.0
        a.append(row)
        b.append(-1.0)
    for v in range(n):
        row = [0.0] * n
        row[v] = 1.0
        a.append(row)
        b.append(1.0)
    c = [-wi for wi in w]
    val, x = _Lp(a, b, c).solve()
    return (-val, x)


def approx_min_weight_vc(
    n: int, edges: list[tuple[int, int]], w: list[float]
) -> tuple[list[int], float, float]:
    """APPROX-MIN-WEIGHT-VC(G, w): LP, then C = { v : x_v >= 1/2 }. Returns (cover, weight, LP bound)."""
    lp, x = vertex_cover_lp(n, edges, w)
    cover = [v for v in range(n) if x[v] >= 0.5 - 1e-9]
    weight = sum(w[v] for v in cover)
    return (cover, weight, lp)


def optimal_weight(n: int, edges: list[tuple[int, int]], w: list[float]) -> float:
    best = math.inf
    for mask in range(1 << n):
        if all(((mask >> u) & 1) or ((mask >> v) & 1) for u, v in edges):
            wt = sum(w[i] for i in range(n) if (mask >> i) & 1)
            if wt < best:
                best = wt
    return best


def demo() -> str:
    edges1 = [(0, 1), (1, 2), (0, 2)]
    lp1, x1 = vertex_cover_lp(3, edges1, [1.0, 1.0, 1.0])
    c1, wt1, _ = approx_min_weight_vc(3, edges1, [1.0, 1.0, 1.0])
    edges2 = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (1, 3)]
    w2 = [3.0, 1.0, 4.0, 1.0, 5.0]
    c2, wt2, lp2 = approx_min_weight_vc(5, edges2, w2)
    return "\n".join([
        f"triangle, unit weights: LP optimum {lp1:.3f}, x = {[round(v * 1000.0) / 1000.0 for v in x1]}",
        f"  rounded cover {c1} weight {wt1} vs optimum {optimal_weight(3, edges1, [1.0, 1.0, 1.0])}",
        f"weighted 5-vertex graph: cover {c2}, weight {wt2} (LP bound {lp2:.3f}, optimum {optimal_weight(5, edges2, w2)})",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_rounding_gives_valid_cover_within_twice_the_lp_bound(self) -> None:
        r = Rng(254)
        for _ in range(150):
            n = r.range(2, 9)
            edges = [(a, b) for _ in range(r.range(1, 14)) for a, b in [(r.range(0, n - 1), r.range(0, n - 1))] if a != b]
            if not edges:
                continue
            w = [float(r.range(1, 9)) for _ in range(n)]
            cover, weight, lp = approx_min_weight_vc(n, edges, w)
            c_set = set(cover)
            self.assertTrue(all(u in c_set or v in c_set for u, v in edges))
            opt = optimal_weight(n, edges, w)
            self.assertLessEqual(lp, opt + 1e-6)
            self.assertLessEqual(weight, 2.0 * lp + 1e-6)
            self.assertLessEqual(weight, 2.0 * opt + 1e-6)

    def test_triangle_lp_is_half_integral(self) -> None:
        lp, x = vertex_cover_lp(3, [(0, 1), (1, 2), (0, 2)], [1.0, 1.0, 1.0])
        self.assertLess(abs(lp - 1.5), 1e-7)
        self.assertTrue(all(abs(v - 0.5) < 1e-7 for v in x))


if __name__ == "__main__":
    print(demo(), end="")
