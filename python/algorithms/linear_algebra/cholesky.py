"""Cholesky Decomposition (Symmetric Positive-Definite Matrices) — CLRS 28.3–28.5 (2e–4e) (2, 3, 4).

A = L·Lᵀ with L lower triangular for symmetric positive-definite A: half the work of LU, no pivoting, and a built-in definiteness test.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Matrix = list[list[float]]


def cholesky(a: Matrix) -> Matrix | None:
    """Cholesky factor L (A = L·L^T). Returns None if A is not (numerically) SPD. Theta(n^3/3) flops."""
    n = len(a)
    l = [[0.0] * n for _ in range(n)]
    for i in range(n):
        for j in range(i + 1):
            s = sum(l[i][k] * l[j][k] for k in range(j))
            if i == j:
                d = a[i][i] - s
                if d <= 1e-12:
                    return None
                l[i][i] = math.sqrt(d)
            else:
                l[i][j] = (a[i][j] - s) / l[j][j]
    return l


def cholesky_solve(l: Matrix, b: list[float]) -> list[float]:
    """Solve A x = b with A = L·L^T by forward and back substitution."""
    n = len(l)
    y = [0.0] * n
    for i in range(n):
        y[i] = (b[i] - sum(l[i][k] * y[k] for k in range(i))) / l[i][i]
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        x[i] = (y[i] - sum(l[k][i] * x[k] for k in range(i + 1, n))) / l[i][i]
    return x


def demo() -> str:
    lines: list[str] = []
    a = [[4.0, 12.0, -16.0], [12.0, 37.0, -43.0], [-16.0, -43.0, 98.0]]
    l = cholesky(a)
    assert l is not None
    lines.append(f"A = {a}")
    lines.append(f"L = {l}")
    x = cholesky_solve(l, [1.0, 2.0, 3.0])
    lines.append(f"solution of A x = (1,2,3): {[round(v * 1e6) / 1e6 for v in x]}")
    lines.append(f"indefinite matrix [[1,2],[2,1]]: {cholesky([[1.0, 2.0], [2.0, 1.0]])}")
    return "\n".join(lines) + "\n"


def _mul(a: Matrix, b: Matrix) -> Matrix:
    n = len(a)
    return [[sum(a[i][k] * b[k][j] for k in range(n)) for j in range(n)] for i in range(n)]


def _transpose(a: Matrix) -> Matrix:
    n = len(a)
    return [[a[j][i] for j in range(n)] for i in range(n)]


def _random_spd(r: Rng, n: int) -> Matrix:
    b = [[r.f64() * 2.0 - 1.0 for _ in range(n)] for _ in range(n)]
    a = _mul(_transpose(b), b)
    for i in range(n):
        a[i][i] += 1.0
    return a


class Tests(unittest.TestCase):
    def test_reconstructs_and_solves_random_spd_systems(self) -> None:
        r = Rng(417)
        for _ in range(100):
            n = r.range(1, 8)
            a = _random_spd(r, n)
            l = cholesky(a)
            self.assertIsNotNone(l)
            assert l is not None
            back = _mul(l, _transpose(l))
            for i in range(n):
                for j in range(n):
                    self.assertLess(abs(back[i][j] - a[i][j]), 1e-8)
                    self.assertTrue(j <= i or l[i][j] == 0.0)
            b = [r.f64() * 10.0 for _ in range(n)]
            x = cholesky_solve(l, b)
            for i in range(n):
                self.assertLess(abs(sum(p * q for p, q in zip(a[i], x)) - b[i]), 1e-7)

    def test_rejects_non_positive_definite_matrices(self) -> None:
        self.assertIsNone(cholesky([[1.0, 2.0], [2.0, 1.0]]))
        self.assertIsNone(cholesky([[0.0]]))
        self.assertIsNone(cholesky([[-1.0, 0.0], [0.0, 1.0]]))

    def test_classic_example(self) -> None:
        l = cholesky([[4.0, 12.0, -16.0], [12.0, 37.0, -43.0], [-16.0, -43.0, 98.0]])
        assert l is not None
        self.assertLess(abs(l[0][0] - 2.0), 1e-12)
        self.assertLess(abs(l[1][0] - 6.0), 1e-12)
        self.assertLess(abs(l[1][1] - 1.0), 1e-12)
        self.assertLess(abs(l[2][0] + 8.0), 1e-12)
        self.assertLess(abs(l[2][1] - 5.0), 1e-12)
        self.assertLess(abs(l[2][2] - 3.0), 1e-12)


if __name__ == "__main__":
    print(demo(), end="")
