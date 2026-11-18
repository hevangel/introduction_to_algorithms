"""LU and LUP Decomposition — CLRS 28.1–28.2 (1e–4e; Ch 31 in 1e–2e) (1, 2, 3, 4).

Solve Ax = b by factoring PA = LU with partial pivoting, then forward/back substitution in Θ(n³).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Matrix = list[list[float]]
_EPS = 1e-12


def lu_decomposition(a: Matrix) -> tuple[Matrix, Matrix] | None:
    """LU-DECOMPOSITION (no pivoting): A = L·U with L unit lower triangular."""
    n = len(a)
    l = [[0.0] * n for _ in range(n)]
    u = [[0.0] * n for _ in range(n)]
    m = [list(row) for row in a]
    for k in range(n):
        if abs(m[k][k]) < _EPS:
            return None
        u[k][k] = m[k][k]
        l[k][k] = 1.0
        for i in range(k + 1, n):
            l[i][k] = m[i][k] / m[k][k]
            u[k][i] = m[k][i]
        for i in range(k + 1, n):
            for j in range(k + 1, n):
                m[i][j] -= l[i][k] * u[k][j]
    return (l, u)


def lup_decomposition(a: Matrix) -> tuple[Matrix, list[int]] | None:
    """LUP-DECOMPOSITION: returns (combined LU matrix in place, permutation pi) with P·A = L·U."""
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
    return (m, pi)


def split_lu(lu: Matrix) -> tuple[Matrix, Matrix]:
    """Split the in-place LU matrix into explicit L (unit diagonal) and U."""
    n = len(lu)
    l = [[0.0] * n for _ in range(n)]
    u = [[0.0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            if i > j:
                l[i][j] = lu[i][j]
            else:
                u[i][j] = lu[i][j]
        l[i][i] = 1.0
    return (l, u)


def lup_solve(lu: Matrix, pi: list[int], b: list[float]) -> list[float]:
    """LUP-SOLVE(L, U, pi, b): forward substitution for y, back substitution for x."""
    n = len(lu)
    y = [0.0] * n
    for i in range(n):
        y[i] = b[pi[i]] - sum(lu[i][j] * y[j] for j in range(i))
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        x[i] = (y[i] - sum(lu[i][j] * x[j] for j in range(i + 1, n))) / lu[i][i]
    return x


def solve(a: Matrix, b: list[float]) -> list[float] | None:
    """Solve A·x = b; None if A is singular."""
    res = lup_decomposition(a)
    if res is None:
        return None
    lu, pi = res
    return lup_solve(lu, pi, b)


def determinant(a: Matrix) -> float:
    """det(A) = (-1)^swaps * prod U_ii."""
    res = lup_decomposition(a)
    if res is None:
        return 0.0
    lu, pi = res
    n = len(pi)
    visited = [False] * n
    swaps = 0
    for i in range(n):
        if not visited[i]:
            j = i
            length = 0
            while not visited[j]:
                visited[j] = True
                j = pi[j]
                length += 1
            swaps += length - 1
    prod = 1.0
    for i in range(n):
        prod *= lu[i][i]
    return prod if swaps % 2 == 0 else -prod


def mat_mul(a: Matrix, b: Matrix) -> Matrix:
    n, m, p = len(a), len(b), len(b[0])
    return [[sum(a[i][k] * b[k][j] for k in range(m)) for j in range(p)] for i in range(n)]


def demo() -> str:
    lines: list[str] = []
    a = [[1.0, 2.0, 0.0], [3.0, 4.0, 4.0], [5.0, 6.0, 3.0]]
    b = [3.0, 7.0, 8.0]
    res = lup_decomposition(a)
    assert res is not None
    lu, pi = res
    l, u = split_lu(lu)
    lines.append(f"π = {pi}")
    lines.append(f"L = {l}")
    lines.append(f"U = {u}")
    x = lup_solve(lu, pi, b)
    lines.append(f"solution x = {[round(v * 1e6) / 1e6 for v in x]} (expected [-1.4, 2.2, 0.6])")
    lines.append(f"det(A) = {determinant(a):.4f}")
    lines.append(f"plain LU (no pivoting) works here? {lu_decomposition(a) is not None}")
    lines.append(f"singular matrix → {lup_decomposition([[1.0, 2.0], [2.0, 4.0]]) is None}")
    return "\n".join(lines) + "\n"


def _rand_matrix(n: int, r: Rng) -> Matrix:
    return [[r.f64() * 10.0 - 5.0 for _ in range(n)] for _ in range(n)]


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        a = [[1.0, 2.0, 0.0], [3.0, 4.0, 4.0], [5.0, 6.0, 3.0]]
        x = solve(a, [3.0, 7.0, 8.0])
        assert x is not None
        for got, want in zip(x, [-1.4, 2.2, 0.6]):
            self.assertLess(abs(got - want), 1e-9)

    def test_pa_equals_lu_and_residual_small(self) -> None:
        r = Rng(180)
        for n in range(1, 13):
            a = _rand_matrix(n, r)
            b = [r.f64() * 10.0 for _ in range(n)]
            res = lup_decomposition(a)
            assert res is not None
            lu, pi = res
            l, u = split_lu(lu)
            pa = [list(a[i]) for i in pi]
            prod = mat_mul(l, u)
            for i in range(n):
                for j in range(n):
                    self.assertLess(abs(pa[i][j] - prod[i][j]), 1e-8)
            x = lup_solve(lu, pi, b)
            for i in range(n):
                self.assertLess(abs(sum(p * q for p, q in zip(a[i], x)) - b[i]), 1e-7)

    def test_determinant_singular_and_lu_without_pivoting(self) -> None:
        self.assertLess(abs(determinant([[0.0, 1.0], [1.0, 0.0]]) + 1.0), 1e-12)
        self.assertLess(abs(determinant([[2.0, 0.0], [0.0, 3.0]]) - 6.0), 1e-12)
        self.assertEqual(determinant([[1.0, 2.0], [2.0, 4.0]]), 0.0)
        self.assertIsNone(lu_decomposition([[0.0, 1.0], [1.0, 0.0]]))
        res = lu_decomposition([[4.0, 3.0], [6.0, 3.0]])
        assert res is not None
        l, u = res
        self.assertEqual(mat_mul(l, u), [[4.0, 3.0], [6.0, 3.0]])


if __name__ == "__main__":
    print(demo(), end="")
