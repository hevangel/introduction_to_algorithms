"""Matrix Inversion — CLRS 28.2 (1e–4e) (1, 2, 3, 4).

Invert a matrix by solving n systems with one LUP factorisation; equivalent in difficulty to matrix multiplication.
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


def _lup_decomposition(a: Matrix) -> tuple[Matrix, list[int]] | None:
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


def _lup_solve(lu: Matrix, pi: list[int], b: list[float]) -> list[float]:
    n = len(lu)
    y = [0.0] * n
    for i in range(n):
        y[i] = b[pi[i]] - sum(lu[i][j] * y[j] for j in range(i))
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        x[i] = (y[i] - sum(lu[i][j] * x[j] for j in range(i + 1, n))) / lu[i][i]
    return x


def _mat_mul(a: Matrix, b: Matrix) -> Matrix:
    n, m, p = len(a), len(b), len(b[0])
    return [[sum(a[i][k] * b[k][j] for k in range(m)) for j in range(p)] for i in range(n)]


def invert(a: Matrix) -> Matrix | None:
    """Compute A^{-1} column by column: solve A·x_j = e_j using one LUP factorisation."""
    n = len(a)
    res = _lup_decomposition(a)
    if res is None:
        return None
    lu, pi = res
    inv = [[0.0] * n for _ in range(n)]
    for j in range(n):
        e = [0.0] * n
        e[j] = 1.0
        col = _lup_solve(lu, pi, e)
        for i in range(n):
            inv[i][j] = col[i]
    return inv


def identity(n: int) -> Matrix:
    return [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]


def identity_error(a: Matrix, b: Matrix) -> float:
    """Max-norm of A·B - I."""
    p = _mat_mul(a, b)
    n = len(a)
    return max(
        (abs(p[i][j] - (1.0 if i == j else 0.0)) for i in range(n) for j in range(n)),
        default=0.0,
    )


def demo() -> str:
    lines: list[str] = []
    a = [[4.0, 7.0], [2.0, 6.0]]
    inv = invert(a)
    assert inv is not None
    lines.append(f"A      = {a}")
    lines.append(f"A⁻¹    = {[[round(v * 1e6) / 1e6 for v in r] for r in inv]}")
    lines.append(f"‖A·A⁻¹ − I‖ = {identity_error(a, inv):.2e}")
    r = Rng(3)
    big = [[r.f64() - 0.5 for _ in range(30)] for _ in range(30)]
    big_inv = invert(big)
    assert big_inv is not None
    lines.append(f"random 30×30: error {identity_error(big, big_inv):.2e}")
    lines.append(f"singular matrix has no inverse: {invert([[1.0, 2.0], [2.0, 4.0]])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_two_by_two(self) -> None:
        inv = invert([[4.0, 7.0], [2.0, 6.0]])
        assert inv is not None
        want = [[0.6, -0.7], [-0.2, 0.4]]
        for i in range(2):
            for j in range(2):
                self.assertLess(abs(inv[i][j] - want[i][j]), 1e-12)

    def test_random_inverses(self) -> None:
        r = Rng(181)
        for n in range(1, 16):
            a = [[r.f64() * 4.0 - 2.0 for _ in range(n)] for _ in range(n)]
            inv = invert(a)
            assert inv is not None
            self.assertLess(identity_error(a, inv), 1e-6, f"n={n}")
            self.assertLess(identity_error(inv, a), 1e-6)
        self.assertEqual(invert(identity(5)), identity(5))


if __name__ == "__main__":
    print(demo(), end="")
