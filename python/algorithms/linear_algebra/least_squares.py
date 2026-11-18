"""Symmetric Positive-Definite Matrices & Least Squares — CLRS 28.3–28.4 (1e–4e) (1, 2, 3, 4).

Cholesky factorisation of SPD matrices and curve fitting by the normal equations AᵀA c = Aᵀy.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

Matrix = list[list[float]]
_EPS = 1e-12


def _lup_solve_system(a: Matrix, b: list[float]) -> list[float] | None:
    n = len(a)
    m = [list(row) for row in a]
    pi = list(range(n))
    for k in range(n):
        kp = max(range(k, n), key=lambda i: abs(m[i][k]))
        if abs(m[kp][k]) < _EPS:
            return None
        pi[k], pi[kp] = pi[kp], pi[k]
        m[k], m[kp] = m[kp], m[k]
        for i in range(k + 1, n):
            m[i][k] /= m[k][k]
            for j in range(k + 1, n):
                m[i][j] -= m[i][k] * m[k][j]
    y = [0.0] * n
    for i in range(n):
        y[i] = b[pi[i]] - sum(m[i][j] * y[j] for j in range(i))
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        x[i] = (y[i] - sum(m[i][j] * x[j] for j in range(i + 1, n))) / m[i][i]
    return x


def transpose(a: Matrix) -> Matrix:
    return [[a[i][j] for i in range(len(a))] for j in range(len(a[0]))]


def _mul(a: Matrix, b: Matrix) -> Matrix:
    return [
        [sum(a[i][k] * b[k][j] for k in range(len(b))) for j in range(len(b[0]))]
        for i in range(len(a))
    ]


def cholesky(a: Matrix) -> Matrix | None:
    """Cholesky factorisation A = L·L^T of a symmetric positive-definite matrix. None if not SPD."""
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


def least_squares(a: Matrix, y: list[float]) -> list[float] | None:
    """Solve the normal equations: least-squares solution c of over-determined system A·c ≈ y."""
    at = transpose(a)
    ata = _mul(at, a)
    aty = [sum(p * q for p, q in zip(row, y)) for row in at]
    return _lup_solve_system(ata, aty)


def polynomial_fit(xs: list[float], ys: list[float], degree: int) -> list[float] | None:
    """Fit a polynomial c_0 + c_1 x + ... + c_d x^d to the points by least squares."""
    a = [[x**j for j in range(degree + 1)] for x in xs]
    return least_squares(a, ys)


def demo() -> str:
    lines: list[str] = []
    xs = [-1.0, 1.0, 2.0, 3.0, 5.0]
    ys = [2.0, 1.0, 1.0, 0.0, 3.0]
    c = polynomial_fit(xs, ys, 2)
    assert c is not None
    lines.append(f"points: {list(zip(xs, ys))}")
    lines.append(f"best quadratic F(x) = {c[0]:.3f} + {c[1]:.3f}x + {c[2]:.3f}x²")
    sse = sum((c[0] + c[1] * x + c[2] * x * x - y) ** 2 for x, y in zip(xs, ys))
    lines.append(f"sum of squared errors = {sse:.3f}")
    spd = [[4.0, 12.0, -16.0], [12.0, 37.0, -43.0], [-16.0, -43.0, 98.0]]
    lines.append(f"Cholesky factor of an SPD matrix: {cholesky(spd)}")
    lines.append(f"indefinite matrix → {cholesky([[1.0, 2.0], [2.0, 1.0]])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_quadratic_fit(self) -> None:
        c = polynomial_fit([-1.0, 1.0, 2.0, 3.0, 5.0], [2.0, 1.0, 1.0, 0.0, 3.0], 2)
        assert c is not None
        self.assertLess(abs(c[0] - 1.2), 1e-9)
        self.assertLess(abs(c[1] + 0.757142857), 1e-6)
        self.assertLess(abs(c[2] - 0.214285714), 1e-6)

    def test_exact_polynomials_are_recovered(self) -> None:
        xs = [float(x) for x in range(-5, 6)]
        ys = [3.0 - 2.0 * x + 0.5 * x * x * x for x in xs]
        c = polynomial_fit(xs, ys, 3)
        assert c is not None
        for got, want in zip(c, [3.0, -2.0, 0.0, 0.5]):
            self.assertLess(abs(got - want), 1e-6)

    def test_cholesky_reconstructs_and_rejects_non_spd(self) -> None:
        a = [[4.0, 12.0, -16.0], [12.0, 37.0, -43.0], [-16.0, -43.0, 98.0]]
        l = cholesky(a)
        assert l is not None
        back = _mul(l, transpose(l))
        for i in range(3):
            for j in range(3):
                self.assertLess(abs(back[i][j] - a[i][j]), 1e-9)
        self.assertEqual(l[0][0], 2.0)
        self.assertIsNone(cholesky([[1.0, 2.0], [2.0, 1.0]]))


if __name__ == "__main__":
    print(demo(), end="")
