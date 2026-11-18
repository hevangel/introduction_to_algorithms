"""Hamiltonian Cycle & Exact TSP — CLRS 36.2, 36.5.4 (1e) / 34.2, 34.5.4 (2e–4e) (1,2,3,4).

Exponential exact algorithms: backtracking search, and the Held-Karp O(n²2ⁿ) dynamic program for tours.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def hamiltonian_cycle_backtracking(adj: list[list[bool]]) -> list[int] | None:
    """Find a Hamiltonian cycle starting at 0 by backtracking."""
    n = len(adj)
    if n == 0:
        return None
    if n == 1:
        return [0]
    path = [0]
    used = [False] * n
    used[0] = True

    def go() -> bool:
        last = path[-1]
        if len(path) == n:
            return adj[last][path[0]]
        for v in range(n):
            if not used[v] and adj[last][v]:
                used[v] = True
                path.append(v)
                if go():
                    return True
                path.pop()
                used[v] = False
        return False

    return path if go() else None


def traveling_salesman_exact(dist: list[list[float]]) -> tuple[float, list[int]] | None:
    """Held-Karp: minimum-cost tour over all n cities in Θ(n² 2^n) time."""
    n = len(dist)
    if n == 0:
        return None
    if n == 1:
        return (0.0, [0])
    full = 1 << (n - 1)
    dp = [[math.inf] * (n - 1) for _ in range(full)]
    parent = [[-1] * (n - 1) for _ in range(full)]
    for j in range(n - 1):
        dp[1 << j][j] = dist[0][j + 1]
    for mask in range(1, full):
        for j in range(n - 1):
            if not ((mask >> j) & 1) or math.isinf(dp[mask][j]):
                continue
            for k in range(n - 1):
                if (mask >> k) & 1:
                    continue
                nm = mask | (1 << k)
                c = dp[mask][j] + dist[j + 1][k + 1]
                if c < dp[nm][k]:
                    dp[nm][k] = c
                    parent[nm][k] = j
    best = math.inf
    last = -1
    for j in range(n - 1):
        c = dp[full - 1][j] + dist[j + 1][0]
        if c < best:
            best = c
            last = j
    if math.isinf(best):
        return None
    tour: list[int] = []
    mask = full - 1
    j = last
    while j != -1:
        tour.append(j + 1)
        p = parent[mask][j]
        mask ^= 1 << j
        j = p
    tour.append(0)
    tour.reverse()
    return (best, tour)


def has_hamiltonian_cycle_dp(adj: list[list[bool]]) -> bool:
    n = len(adj)
    d = [[0.0 if (i != j and adj[i][j]) else math.inf for j in range(n)] for i in range(n)]
    return traveling_salesman_exact(d) is not None


def demo() -> str:
    lines: list[str] = []
    adj = [[False] * 6 for _ in range(6)]
    for a, b in [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0), (0, 3), (1, 4)]:
        adj[a][b] = adj[b][a] = True
    lines.append(f"backtracking finds: {hamiltonian_cycle_backtracking(adj)}")
    lines.append(f"DP agrees (exists): {has_hamiltonian_cycle_dp(adj)}")
    pts = [(0.0, 0.0), (3.0, 0.0), (3.0, 4.0), (0.0, 4.0), (1.5, 2.0)]
    d = [[math.hypot(a[0] - b[0], a[1] - b[1]) for b in pts] for a in pts]
    res = traveling_salesman_exact(d)
    lines.append(f"Held-Karp optimal tour of 5 points: {(f'{res[0]:.3f}', res[1]) if res else None}")
    star = [[False] * 5 for _ in range(5)]
    for v in range(1, 5):
        star[0][v] = star[v][0] = True
    lines.append(f"star graph (no Hamiltonian cycle): {hamiltonian_cycle_backtracking(star)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute_tsp(self, d: list[list[float]]) -> float:
        n = len(d)
        best = math.inf

        def go(cur: int, used: int, cost: float) -> None:
            nonlocal best
            if used == (1 << n) - 1:
                best = min(best, cost + d[cur][0])
                return
            for v in range(1, n):
                if not ((used >> v) & 1):
                    go(v, used | (1 << v), cost + d[cur][v])

        go(0, 1, 0.0)
        return best

    def test_tsp_dp_matches_permutation_search(self) -> None:
        r = Rng(240)
        for n in range(2, 9):
            pts = [(r.f64() * 10.0, r.f64() * 10.0) for _ in range(n)]
            d = [[math.hypot(a[0] - b[0], a[1] - b[1]) for b in pts] for a in pts]
            res = traveling_salesman_exact(d)
            assert res is not None
            c, tour = res
            self.assertAlmostEqual(c, self._brute_tsp(d), places=9)
            length = sum(d[tour[i]][tour[(i + 1) % n]] for i in range(n))
            self.assertAlmostEqual(length, c, places=9)
            self.assertEqual(sorted(tour), list(range(n)))

    def test_backtracking_agrees_with_dp_on_random_graphs(self) -> None:
        r = Rng(241)
        yes = no = 0
        for _ in range(200):
            n = r.range(3, 8)
            adj = [[False] * n for _ in range(n)]
            for i in range(n):
                for j in range(i + 1, n):
                    if r.f64() < 0.4:
                        adj[i][j] = adj[j][i] = True
            bt = hamiltonian_cycle_backtracking(adj)
            self.assertEqual(bt is not None, has_hamiltonian_cycle_dp(adj))
            if bt is not None:
                yes += 1
                self.assertEqual(len(bt), n)
                self.assertEqual(sorted(bt), list(range(n)))
                self.assertTrue(all(adj[bt[i]][bt[(i + 1) % n]] for i in range(n)))
            else:
                no += 1
        self.assertGreater(yes, 10)
        self.assertGreater(no, 10)


if __name__ == "__main__":
    print(demo(), end="")
