"""Gradient Descent — CLRS 33.3 (4e) (4e).

Minimise a differentiable convex function by repeatedly stepping against the gradient; linear regression example.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest
from typing import Callable

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def gradient_descent(
    grad: Callable[[list[float]], list[float]],
    x0: list[float],
    eta: float,
    iters: int,
) -> list[float]:
    """GRADIENT-DESCENT: x_{t+1} = x_t - eta * grad(f(x_t))."""
    x = list(x0)
    for _ in range(iters):
        g = grad(x)
        for i in range(len(x)):
            x[i] -= eta * g[i]
    return x


def gradient_descent_tol(
    grad: Callable[[list[float]], list[float]],
    x0: list[float],
    eta: float,
    tol: float,
    max_iters: int,
) -> tuple[list[float], int]:
    """Gradient descent with early stopping when ||grad(f)|| < tol. Returns (x, iterations used)."""
    x = list(x0)
    for t in range(max_iters):
        g = grad(x)
        if math.sqrt(sum(v * v for v in g)) < tol:
            return x, t
        for i in range(len(x)):
            x[i] -= eta * g[i]
    return x, max_iters


def linear_regression(
    x: list[list[float]],
    y: list[float],
    eta: float,
    iters: int,
) -> list[float]:
    """Least-squares linear regression y ≈ X*theta by gradient descent on f(theta) = (1/2m)*||X*theta - y||^2."""
    m = len(x)
    d = len(x[0])

    def grad(th: list[float]) -> list[float]:
        resid = [sum(a * b for a, b in zip(row, th)) - yi for row, yi in zip(x, y)]
        return [sum(x[i][j] * resid[i] for i in range(m)) / m for j in range(d)]

    return gradient_descent(grad, [0.0] * d, eta, iters)


def _lup_solve_linear(a: list[list[float]], b: list[float]) -> list[float] | None:
    """Self-contained LUP solver for normal equations verification."""
    n = len(a)
    mat = [list(row) for row in a]
    pi = list(range(n))
    for k in range(n):
        kp = max(range(k, n), key=lambda i: abs(mat[i][k]))
        if abs(mat[kp][k]) < 1e-12:
            return None
        pi[k], pi[kp] = pi[kp], pi[k]
        mat[k], mat[kp] = mat[kp], mat[k]
        for i in range(k + 1, n):
            mat[i][k] /= mat[k][k]
            for j in range(k + 1, n):
                mat[i][j] -= mat[i][k] * mat[k][j]
    y = [0.0] * n
    for i in range(n):
        y[i] = b[pi[i]] - sum(mat[i][j] * y[j] for j in range(i))
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        x[i] = (y[i] - sum(mat[i][j] * x[j] for j in range(i + 1, n))) / mat[i][i]
    return x


def _least_squares(a: list[list[float]], y: list[float]) -> list[float] | None:
    """Solve normal equations A^T A c = A^T y."""
    m = len(a)
    d = len(a[0])
    ata = [[sum(a[k][i] * a[k][j] for k in range(m)) for j in range(d)] for i in range(d)]
    aty = [sum(a[k][i] * y[k] for k in range(m)) for i in range(d)]
    return _lup_solve_linear(ata, aty)


def demo() -> str:
    lines: list[str] = []
    grad = lambda p: [2.0 * (p[0] - 3.0), 4.0 * (p[1] + 1.0)]
    for eta in (0.01, 0.1, 0.4, 0.55):
        x = gradient_descent(grad, [0.0, 0.0], eta, 100)
        lines.append(f"η = {eta:<4}: after 100 steps x = ({x[0]:>10.4f}, {x[1]:>10.4f})")
    x, it = gradient_descent_tol(grad, [10.0, 10.0], 0.25, 1e-8, 10_000)
    lines.append(f"with a tolerance: converged to ({x[0]:.6f}, {x[1]:.6f}) in {it} iterations")
    xs = [[1.0, i / 10.0] for i in range(20)]
    ys = [2.0 + 3.0 * r[1] for r in xs]
    theta = [round(v * 1e4) / 1e4 for v in linear_regression(xs, ys, 0.3, 5000)]
    lines.append(f"linear regression on y = 2 + 3x: θ = {theta}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_converges_on_a_convex_quadratic(self) -> None:
        grad = lambda p: [2.0 * (p[0] - 3.0), 4.0 * (p[1] + 1.0)]
        x = gradient_descent(grad, [0.0, 0.0], 0.1, 500)
        self.assertLess(abs(x[0] - 3.0), 1e-6)
        self.assertLess(abs(x[1] + 1.0), 1e-6)
        x2, it = gradient_descent_tol(grad, [10.0, -10.0], 0.2, 1e-9, 100_000)
        self.assertLess(abs(x2[0] - 3.0), 1e-6)
        self.assertLess(it, 1000)

    def test_step_size_matters(self) -> None:
        grad = lambda p: [2.0 * p[0]]
        self.assertLess(abs(gradient_descent(grad, [1.0], 0.9, 200)[0]), 1e-6)
        self.assertGreater(abs(gradient_descent(grad, [1.0], 1.1, 200)[0]), 1e3)

    def test_regression_matches_normal_equations(self) -> None:
        r = Rng(263)
        xs = [[1.0, r.f64() * 4.0 - 2.0, r.f64() * 2.0] for _ in range(50)]
        ys = [1.0 - 2.0 * row[1] + 0.5 * row[2] + (r.f64() - 0.5) * 0.1 for row in xs]
        gd = linear_regression(xs, ys, 0.1, 20_000)
        exact = _least_squares(xs, ys)
        assert exact is not None
        for a, b in zip(gd, exact):
            self.assertLess(abs(a - b), 1e-3, f"{gd} vs {exact}")


if __name__ == "__main__":
    print(demo(), end="")
