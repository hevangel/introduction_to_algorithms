"""Approximate Metric TSP — CLRS 37.2 (1e) / 35.2 (2e–4e) (1,2,3,4).

Preorder walk of a minimum spanning tree: a 2-approximation when distances satisfy the triangle inequality.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Pt = tuple[float, float]


def dist(a: Pt, b: Pt) -> float:
    return math.hypot(a[0] - b[0], a[1] - b[1])


def _mst_parents(pts: list[Pt]) -> list[int]:
    n = len(pts)
    key = [math.inf] * n
    parent = [-1] * n
    done = [False] * n
    key[0] = 0.0
    for _ in range(n):
        u = min((v for v in range(n) if not done[v]), key=lambda v: key[v])
        done[u] = True
        for v in range(n):
            d = dist(pts[u], pts[v])
            if not done[v] and d < key[v]:
                key[v] = d
                parent[v] = u
    return parent


def approx_tsp_tour(pts: list[Pt]) -> tuple[float, list[int]]:
    """APPROX-TSP-TOUR(G, c, r): build an MST, list its vertices in preorder, and return (cost, tour)."""
    n = len(pts)
    if n == 0:
        return (0.0, [])
    parent = _mst_parents(pts)
    children: list[list[int]] = [[] for _ in range(n)]
    for v in range(1, n):
        children[parent[v]].append(v)
    tour: list[int] = []
    stack = [0]
    while stack:
        u = stack.pop()
        tour.append(u)
        for c in reversed(children[u]):
            stack.append(c)
    length = sum(dist(pts[tour[i]], pts[tour[(i + 1) % n]]) for i in range(n))
    return (length, tour)


def mst_weight(pts: list[Pt]) -> float:
    if len(pts) <= 1:
        return 0.0
    p = _mst_parents(pts)
    return sum(dist(pts[v], pts[p[v]]) for v in range(1, len(pts)))


def _traveling_salesman_exact(w: list[list[float]]) -> tuple[float, list[int]] | None:
    n = len(w)
    if n == 0:
        return (0.0, [])
    if n == 1:
        return (0.0, [0])
    full = 1 << (n - 1)
    dp = [[math.inf] * (n - 1) for _ in range(full)]
    par = [[-1] * (n - 1) for _ in range(full)]
    for j in range(n - 1):
        dp[1 << j][j] = w[0][j + 1]
    for mask in range(1, full):
        for j in range(n - 1):
            if not ((mask >> j) & 1) or math.isinf(dp[mask][j]):
                continue
            for k in range(n - 1):
                if (mask >> k) & 1:
                    continue
                nm = mask | (1 << k)
                cand = dp[mask][j] + w[j + 1][k + 1]
                if cand < dp[nm][k]:
                    dp[nm][k] = cand
                    par[nm][k] = j
    best_cost = math.inf
    last = -1
    for j in range(n - 1):
        cand = dp[full - 1][j] + w[j + 1][0]
        if cand < best_cost:
            best_cost = cand
            last = j
    if last == -1:
        return None
    tour = [0] * n
    mask = full - 1
    cur = last
    for pos in range(n - 1, 0, -1):
        tour[pos] = cur + 1
        prev = par[mask][cur]
        mask ^= 1 << cur
        cur = prev
    return (best_cost, tour)


def demo() -> str:
    pts: list[Pt] = [(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0), (2.0, 1.5), (6.0, 1.0), (1.0, 5.0)]
    approx, tour = approx_tsp_tour(pts)
    d = [[dist(a, b) for b in pts] for a in pts]
    res = _traveling_salesman_exact(d)
    assert res is not None
    opt, opt_tour = res
    return "\n".join([
        f"MST weight          = {mst_weight(pts):.3f}  (lower bound on OPT)",
        f"approximate tour    = {approx:.3f}  {tour}",
        f"optimal tour (DP)   = {opt:.3f}  {opt_tour}",
        f"ratio = {approx / opt:.3f}  (guaranteed ≤ 2)",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_two_approximation_on_random_euclidean_instances(self) -> None:
        r = Rng(251)
        for _ in range(60):
            n = r.range(3, 9)
            pts = [(r.f64() * 100.0, r.f64() * 100.0) for _ in range(n)]
            length, tour = approx_tsp_tour(pts)
            self.assertEqual(sorted(tour), list(range(n)))
            d = [[dist(a, b) for b in pts] for a in pts]
            res = _traveling_salesman_exact(d)
            assert res is not None
            opt = res[0]
            self.assertGreaterEqual(length, opt - 1e-9)
            self.assertLessEqual(length, 2.0 * opt + 1e-9)
            self.assertLessEqual(mst_weight(pts), opt + 1e-9)

    def test_trivial_cases(self) -> None:
        self.assertEqual(approx_tsp_tour([])[0], 0.0)
        self.assertEqual(approx_tsp_tour([(1.0, 1.0)])[1], [0])
        self.assertAlmostEqual(approx_tsp_tour([(0.0, 0.0), (3.0, 4.0)])[0], 10.0, places=9)


if __name__ == "__main__":
    print(demo(), end="")
