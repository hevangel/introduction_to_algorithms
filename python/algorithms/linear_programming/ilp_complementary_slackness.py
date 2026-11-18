"""LP Feasibility, Complementary Slackness & Integer LP — CLRS Problems 29-1..29-3 (2e–4e) (2, 3, 4).

Solve Ax ≤ b feasibility via Simplex, verify primal/dual optimality certificates via complementary slackness, and solve small integer LPs with branch-and-bound.
"""

from __future__ import annotations

from dataclasses import dataclass
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_EPS = 1e-9


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


def _dual(a: list[list[float]], b: list[float], c: list[float]) -> tuple[list[list[float]], list[float], list[float]]:
    m, n = len(b), len(c)
    at = [[-a[i][j] for i in range(m)] for j in range(n)]
    return (at, [-v for v in c], [-v for v in b])


def check_lp_feasibility(a: list[list[float]], b: list[float]) -> list[float] | None:
    """Problem 29-1: Decide feasibility of { x >= 0 : A x <= b }."""
    n = len(a[0])
    res = _simplex(a, b, [0.0] * n)
    if isinstance(res, _LpOptimal):
        return res.x
    if isinstance(res, _LpUnbounded):
        return [0.0] * n
    return None


def verify_complementary_slackness(
    a: list[list[float]],
    b: list[float],
    c: list[float],
    x: list[float],
    y: list[float],
    tol: float,
) -> bool:
    """Problem 29-2: Verify strong-duality complementary slackness certificate for primal x and dual y."""
    m = len(b)
    n = len(c)
    if any(v < -tol for v in x) or any(v < -tol for v in y):
        return False
    for i in range(m):
        ax_i = sum(a[i][j] * x[j] for j in range(n))
        if ax_i > b[i] + tol:
            return False
        slack = b[i] - ax_i
        if abs(y[i] * slack) > tol:
            return False
    for j in range(n):
        aty_j = sum(a[i][j] * y[i] for i in range(m))
        if aty_j < c[j] - tol:
            return False
        d_slack = aty_j - c[j]
        if abs(x[j] * d_slack) > tol:
            return False
    return True


def solve_primal_dual_pair(
    a: list[list[float]], b: list[float], c: list[float]
) -> tuple[float, list[float], list[float]] | None:
    """Extract dual multipliers y by solving the dual LP with Simplex."""
    pres = _simplex(a, b, c)
    if not isinstance(pres, _LpOptimal):
        return None
    da, db, dc = _dual(a, b, c)
    dres = _simplex(da, db, dc)
    if not isinstance(dres, _LpOptimal):
        return None
    return (pres.value, pres.x, dres.x)


def solve_ilp_branch_and_bound(
    a: list[list[float]], b: list[float], c: list[float]
) -> tuple[float, list[int]] | None:
    """Problem 29-3: Solve Integer Linear Program max c^T x s.t. Ax <= b, x >= 0, x in Z^n."""
    best_val = float("-inf")
    best_x: list[int] | None = None

    def rec(cur_a: list[list[float]], cur_b: list[float], depth: int) -> None:
        nonlocal best_val, best_x
        if depth > 40:
            return
        res = _simplex(cur_a, cur_b, c)
        if not isinstance(res, _LpOptimal):
            return
        value, x = res.value, res.x
        if value <= best_val + 1e-7:
            return
        frac = next(((j, v) for j, v in enumerate(x) if abs(v - round(v)) > 1e-6), None)
        if frac is None:
            best_val = value
            best_x = [int(round(v)) for v in x]
            return
        j, val = frac
        n = len(c)
        fl = float(math.floor(val))
        cl = float(math.ceil(val))

        row1 = [0.0] * n
        row1[j] = 1.0
        rec(cur_a + [row1], cur_b + [fl], depth + 1)

        row2 = [0.0] * n
        row2[j] = -1.0
        rec(cur_a + [row2], cur_b + [-cl], depth + 1)

    rec(a, b, 0)
    if best_x is None:
        return None
    return (best_val, best_x)


def demo() -> str:
    lines: list[str] = []
    a = [[1.0, 1.0, 3.0], [2.0, 2.0, 5.0], [4.0, 1.0, 2.0]]
    b = [30.0, 24.0, 36.0]
    c = [3.0, 1.0, 2.0]
    pair = solve_primal_dual_pair(a, b, c)
    assert pair is not None
    val, x, y = pair
    ok = verify_complementary_slackness(a, b, c, x, y, 1e-6)
    lines.append(f"primal/dual opt = {val:.2f}, complementary slackness holds? {ok}")

    ia = [[2.0, 2.0]]
    ib = [3.0]
    ic = [1.0, 1.0]
    lp_res = _simplex(ia, ib, ic)
    assert isinstance(lp_res, _LpOptimal)
    ilp_res = solve_ilp_branch_and_bound(ia, ib, ic)
    assert ilp_res is not None
    ilp_v, ilp_x = ilp_res
    lines.append(f"integrality gap: LP relaxation = {lp_res.value:.2f}, ILP opt = {ilp_v:.2f} at {ilp_x}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_complementary_slackness_holds_for_random_bounded_lps(self) -> None:
        r = Rng(2902)
        checked = 0
        for _ in range(60):
            m = r.range(2, 4)
            a = [[float(r.range(0, 5)) for _ in range(2)] for _ in range(m)]
            b = [float(r.range(1, 15)) for _ in range(m)]
            a.append([1.0, 0.0])
            b.append(10.0)
            a.append([0.0, 1.0])
            b.append(10.0)
            c = [float(r.range(1, 8)) for _ in range(2)]
            pair = solve_primal_dual_pair(a, b, c)
            if pair is not None:
                val, x, y = pair
                self.assertTrue(verify_complementary_slackness(a, b, c, x, y, 1e-5))
                dual_val = sum(bi * yi for bi, yi in zip(b, y))
                self.assertLess(abs(val - dual_val), 1e-5)
                bad_y = list(y)
                bad_y[0] += 100.0
                self.assertFalse(verify_complementary_slackness(a, b, c, x, bad_y, 1e-5))
                checked += 1
        self.assertGreaterEqual(checked, 20)

    def test_feasibility_and_ilp_match_grid_brute_force(self) -> None:
        self.assertIsNotNone(check_lp_feasibility([[1.0, 1.0], [-1.0, -1.0]], [5.0, -2.0]))
        self.assertIsNone(check_lp_feasibility([[1.0], [-1.0]], [1.0, -2.0]))

        a = [[2.0, 3.0], [4.0, 1.0]]
        b = [7.0, 6.0]
        c = [5.0, 4.0]
        res = solve_ilp_branch_and_bound(a, b, c)
        assert res is not None
        ilp_v, ilp_x = res
        brute_v = -(1 << 60)
        for x0 in range(7):
            for x1 in range(7):
                if 2 * x0 + 3 * x1 <= 7 and 4 * x0 + x1 <= 6:
                    brute_v = max(brute_v, 5 * x0 + 4 * x1)
        self.assertLess(abs(ilp_v - float(brute_v)), 1e-7)
        self.assertEqual(5 * ilp_x[0] + 4 * ilp_x[1], brute_v)


if __name__ == "__main__":
    print(demo(), end="")
