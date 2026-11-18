"""The Simplex Algorithm & Duality — CLRS 29 (2e–4e); detailed pivoting 29.3, 29.5 (3e) (2, 3, 4).

Maximise cᵀx subject to Ax ≤ b, x ≥ 0 by pivoting between basic feasible solutions; initialisation and LP duality.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_EPS = 1e-9


@dataclass(eq=True)
class LpOptimal:
    value: float
    x: list[float]


@dataclass(eq=True)
class LpUnbounded:
    pass


@dataclass(eq=True)
class LpInfeasible:
    pass


LpResult = LpOptimal | LpUnbounded | LpInfeasible


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

    def solve(self) -> LpResult:
        m, n = self.m, self.n
        r = min(range(m), key=lambda i: self.d[i][n + 1])
        if self.d[r][n + 1] < -_EPS:
            self.pivot(r, n)
            if not self.run(1) or self.d[m + 1][n + 1] < -_EPS:
                return LpInfeasible()
            for i in range(m):
                if self.basic[i] == -1:
                    s = min(range(n + 1), key=lambda col: (self.d[i][col], self.nonbasic[col]))
                    self.pivot(i, s)
        if not self.run(2):
            return LpUnbounded()
        x = [0.0] * n
        for i in range(m):
            if 0 <= self.basic[i] < n:
                x[self.basic[i]] = self.d[i][n + 1]
        return LpOptimal(value=self.d[m][n + 1], x=x)


def simplex(a: list[list[float]], b: list[float], c: list[float]) -> LpResult:
    """Maximise c^T x subject to Ax <= b, x >= 0."""
    assert len(b) > 0
    return _Lp(a, b, c).solve()


def dual(a: list[list[float]], b: list[float], c: list[float]) -> tuple[list[list[float]], list[float], list[float]]:
    """The dual of max c^T x, Ax <= b, x >= 0 in standard maximisation form."""
    m, n = len(b), len(c)
    at = [[-a[i][j] for i in range(m)] for j in range(n)]
    return (at, [-v for v in c], [-v for v in b])


def demo() -> str:
    lines: list[str] = []
    a = [[1.0, 1.0, 3.0], [2.0, 2.0, 5.0], [4.0, 1.0, 2.0]]
    b = [30.0, 24.0, 36.0]
    c = [3.0, 1.0, 2.0]
    lines.append("primal: max 3x1 + x2 + 2x3  s.t.  x1+x2+3x3 ≤ 30, 2x1+2x2+5x3 ≤ 24, 4x1+x2+2x3 ≤ 36")
    lines.append(f"  -> {simplex(a, b, c)}")
    da, db, dc = dual(a, b, c)
    res = simplex(da, db, dc)
    if isinstance(res, LpOptimal):
        rounded = [round(v * 1e4) / 1e4 for v in res.x]
        lines.append(
            f"dual optimum (min bᵀy) = {-res.value:.4f} at y = {rounded}  — equal to the primal value (strong duality)"
        )
    lines.append(f"infeasible: x ≤ −1, x ≥ 0 → {simplex([[1.0]], [-1.0], [1.0])}")
    lines.append(f"unbounded : max x s.t. −x ≤ 1 → {simplex([[-1.0]], [1.0], [1.0])}")
    lines.append(
        f"needs initialisation (b < 0): max −x1−x2 s.t. −x1−x2 ≤ −2 → {simplex([[-1.0, -1.0]], [-2.0], [-1.0, -1.0])}"
    )
    return "\n".join(lines) + "\n"


def _feasible(a: list[list[float]], b: list[float], x: list[float]) -> bool:
    return all(v > -1e-7 for v in x) and all(
        sum(p * q for p, q in zip(row, x)) <= bi + 1e-6 for row, bi in zip(a, b)
    )


def _brute2(a: list[list[float]], b: list[float], c: list[float]) -> float | None:
    lines = [(row[0], row[1], bi) for row, bi in zip(a, b)]
    lines.append((1.0, 0.0, 0.0))
    lines.append((0.0, 1.0, 0.0))
    best: float | None = None
    for i in range(len(lines)):
        for j in range(i + 1, len(lines)):
            l1, l2 = lines[i], lines[j]
            det = l1[0] * l2[1] - l1[1] * l2[0]
            if abs(det) < 1e-12:
                continue
            x = [(l1[2] * l2[1] - l1[1] * l2[2]) / det, (l1[0] * l2[2] - l1[2] * l2[0]) / det]
            if _feasible(a, b, x):
                v = c[0] * x[0] + c[1] * x[1]
                best = v if best is None else max(best, v)
    return best


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        a = [[1.0, 1.0, 3.0], [2.0, 2.0, 5.0], [4.0, 1.0, 2.0]]
        res = simplex(a, [30.0, 24.0, 36.0], [3.0, 1.0, 2.0])
        self.assertIsInstance(res, LpOptimal)
        assert isinstance(res, LpOptimal)
        self.assertLess(abs(res.value - 28.0), 1e-7)
        for g, w in zip(res.x, [8.0, 4.0, 0.0]):
            self.assertLess(abs(g - w), 1e-7)

    def test_status_codes(self) -> None:
        self.assertEqual(simplex([[1.0]], [-1.0], [1.0]), LpInfeasible())
        self.assertEqual(simplex([[-1.0]], [1.0], [1.0]), LpUnbounded())
        res = simplex([[-1.0, -1.0]], [-2.0], [-1.0, -1.0])
        self.assertIsInstance(res, LpOptimal)
        assert isinstance(res, LpOptimal)
        self.assertLess(abs(res.value + 2.0), 1e-7)

    def test_matches_vertex_enumeration_and_duality(self) -> None:
        r = Rng(190)
        checked = 0
        for _ in range(200):
            m = r.range(1, 5)
            a = [[float(r.range(0, 8) - 2) for _ in range(2)] for _ in range(m)]
            b = [float(r.range(0, 20) - 3) for _ in range(m)]
            c = [float(r.range(0, 8) - 2) for _ in range(2)]
            a2 = [list(row) for row in a] + [[1.0, 0.0], [0.0, 1.0]]
            b2 = list(b) + [30.0, 30.0]
            res = simplex(a2, b2, c)
            want = _brute2(a2, b2, c)
            if isinstance(res, LpOptimal) and want is not None:
                self.assertLess(abs(res.value - want), 1e-6)
                self.assertTrue(_feasible(a2, b2, res.x))
                da, db, dc = dual(a2, b2, c)
                dres = simplex(da, db, dc)
                self.assertIsInstance(dres, LpOptimal)
                assert isinstance(dres, LpOptimal)
                self.assertLess(abs(res.value + dres.value), 1e-5)
                checked += 1
            elif isinstance(res, LpInfeasible) and want is None:
                pass
            else:
                self.fail(f"mismatch {res} vs {want} for A={a2} b={b2} c={c}")
        self.assertGreater(checked, 50)


if __name__ == "__main__":
    print(demo(), end="")
