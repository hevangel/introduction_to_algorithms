"""Tridiagonal Systems (Thomas Algorithm) — CLRS Problem 28-1 (2e–4e); Ch 31 exercises (1e) (1, 2, 3, 4).

LU decomposition of a tridiagonal matrix keeps its band structure, so a tridiagonal system A x = d is solved in O(n) time.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def solve_tridiagonal(
    sub: list[float], diag: list[float], sup: list[float], rhs: list[float]
) -> list[float] | None:
    """Solve a tridiagonal system in Theta(n). Returns None on a zero pivot."""
    n = len(diag)
    assert len(sub) + 1 == n and len(sup) + 1 == n and len(rhs) == n
    c = [0.0] * n
    d = [0.0] * n
    if abs(diag[0]) < 1e-14:
        return None
    c[0] = sup[0] / diag[0] if n > 1 else 0.0
    d[0] = rhs[0] / diag[0]
    for i in range(1, n):
        denom = diag[i] - sub[i - 1] * c[i - 1]
        if abs(denom) < 1e-14:
            return None
        if i < n - 1:
            c[i] = sup[i] / denom
        d[i] = (rhs[i] - sub[i - 1] * d[i - 1]) / denom
    x = [0.0] * n
    x[n - 1] = d[n - 1]
    for i in range(n - 2, -1, -1):
        x[i] = d[i] - c[i] * x[i + 1]
    return x


def demo() -> str:
    lines: list[str] = []
    n = 8
    x = solve_tridiagonal([-1.0] * (n - 1), [2.0] * n, [-1.0] * (n - 1), [1.0] * n)
    assert x is not None
    lines.append(f"tridiag(−1,2,−1) x = 1, n = {n}: {[round(v * 1000.0) / 1000.0 for v in x]}")
    lines.append(f"exact: x_i = i(n+1−i)/2 → {[i * (n + 1 - i) / 2.0 for i in range(1, n + 1)]}")
    big = 200_000
    sol = solve_tridiagonal([-1.0] * (big - 1), [2.0] * big, [-1.0] * (big - 1), [1.0] * big)
    assert sol is not None
    exact_mid = (big // 2) * (big + 1 - big // 2) / 2.0
    lines.append(f"n = {big} solved in O(n); middle value {sol[big // 2 - 1]:.1f} (exact {exact_mid:.1f})")
    return "\n".join(lines) + "\n"


def _dense_lup_solve(a: list[list[float]], b: list[float]) -> list[float] | None:
    n = len(a)
    m = [list(row) for row in a]
    pi = list(range(n))
    for k in range(n):
        kp = max(range(k, n), key=lambda i: abs(m[i][k]))
        if abs(m[kp][k]) < 1e-12:
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


class Tests(unittest.TestCase):
    def test_matches_dense_lup_solver_on_diagonally_dominant_systems(self) -> None:
        r = Rng(418)
        for _ in range(200):
            n = r.range(1, 10)
            sub = [r.f64() * 2.0 - 1.0 for _ in range(n - 1)]
            sup = [r.f64() * 2.0 - 1.0 for _ in range(n - 1)]
            diag = [3.0 + r.f64() for _ in range(n)]
            rhs = [r.f64() * 10.0 - 5.0 for _ in range(n)]
            a = [[0.0] * n for _ in range(n)]
            for i in range(n):
                a[i][i] = diag[i]
                if i + 1 < n:
                    a[i][i + 1] = sup[i]
                    a[i + 1][i] = sub[i]
            x = solve_tridiagonal(sub, diag, sup, rhs)
            y = _dense_lup_solve(a, rhs)
            assert x is not None and y is not None
            for i in range(n):
                self.assertLess(abs(x[i] - y[i]), 1e-8, f"{x} {y}")

    def test_poisson_matrix_has_the_closed_form_solution(self) -> None:
        n = 50
        x = solve_tridiagonal([-1.0] * (n - 1), [2.0] * n, [-1.0] * (n - 1), [1.0] * n)
        assert x is not None
        for i in range(1, n + 1):
            self.assertLess(abs(x[i - 1] - (i * (n + 1 - i)) / 2.0), 1e-8)

    def test_singular_system_detected(self) -> None:
        self.assertIsNone(solve_tridiagonal([1.0], [0.0, 1.0], [1.0], [1.0, 1.0]))


if __name__ == "__main__":
    print(demo(), end="")
