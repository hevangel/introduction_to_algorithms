"""Newton's Method, Hedge & Stochastic Gradient Descent — CLRS Problems 33-1, 33-2 & 33-4 (4e) (4e).

Second-order Newton root/minimization convergence, continuous Hedge multiplicative weights over bounded losses, and stochastic mini-batch gradient descent.
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


def _lup_solve(a: list[list[float]], b: list[float]) -> list[float] | None:
    """Self-contained LUP solver for Newton's method Hessian linear system."""
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


def _gd_linear_regression(
    x: list[list[float]],
    y: list[float],
    eta: float,
    iters: int,
) -> list[float]:
    """Full-batch gradient descent linear regression helper for testing SGD."""
    m = len(x)
    d = len(x[0])
    theta = [0.0] * d
    for _ in range(iters):
        resid = [sum(a * b for a, b in zip(row, theta)) - yi for row, yi in zip(x, y)]
        for j in range(d):
            g = sum(x[i][j] * resid[i] for i in range(m)) / m
            theta[j] -= eta * g
    return theta


def newton_minimize_1d(
    d1: Callable[[float], float],
    d2: Callable[[float], float],
    x: float,
    tol: float,
    max_iters: int,
) -> tuple[float, int]:
    """1D Newton's method for minimizing f (`x <- x - f'(x)/f''(x)`). Returns (x_opt, iterations)."""
    for it in range(max_iters):
        g = d1(x)
        if abs(g) < tol:
            return x, it
        h = d2(x)
        if abs(h) <= 1e-15:
            raise ValueError("zero second derivative in Newton step")
        x -= g / h
    return x, max_iters


def newton_minimize(
    grad: Callable[[list[float]], list[float]],
    hessian: Callable[[list[float]], list[list[float]]],
    x0: list[float],
    tol: float,
    max_iters: int,
) -> tuple[list[float], int]:
    """Multi-dimensional Newton's method: solves H(x) dx = -grad(f(x)) and sets x <- x + dx."""
    x = list(x0)
    for it in range(max_iters):
        g = grad(x)
        if math.sqrt(sum(v * v for v in g)) < tol:
            return x, it
        h = hessian(x)
        neg_g = [-v for v in g]
        step = _lup_solve(h, neg_g)
        if step is None:
            raise ValueError("singular Hessian in Newton step")
        for i in range(len(x)):
            x[i] += step[i]
    return x, max_iters


def hedge(losses: list[list[float]], eps: float) -> tuple[float, float, list[float]]:
    """Continuous Hedge multiplicative weights algorithm over bounded losses in [0, 1].

    Updates w[i] <- w[i] * (1 - eps)^{losses[t][i]} and returns
    (learner_expected_loss, best_expert_loss, final_probs).
    """
    if not losses or not (0.0 < eps < 1.0):
        raise ValueError("invalid losses or eps")
    n = len(losses[0])
    w = [1.0] * n
    learner_loss = 0.0
    expert_losses = [0.0] * n
    for row in losses:
        sum_w = sum(w)
        for i, l in enumerate(row):
            p = w[i] / sum_w
            learner_loss += p * l
            expert_losses[i] += l
            w[i] *= (1.0 - eps) ** l
    sum_w = sum(w)
    probs = [wi / sum_w for wi in w]
    best_expert = min(expert_losses)
    return learner_loss, best_expert, probs


def hedge_loss_bound(best_expert_loss: float, n: int, eps: float) -> float:
    return (1.0 + eps) * best_expert_loss + math.log(n) / eps


def sgd_linear_regression(
    x: list[list[float]],
    y: list[float],
    eta: float,
    epochs: int,
    batch_size: int,
    seed: int,
) -> list[float]:
    """Stochastic / mini-batch Gradient Descent for linear regression y ≈ X*theta."""
    m = len(x)
    d = len(x[0])
    theta = [0.0] * d
    rng = Rng(seed)
    order = list(range(m))
    bsz = min(max(batch_size, 1), m)

    for ep in range(epochs):
        rng.shuffle(order)
        lr = eta / (1.0 + 0.005 * ep)
        for start in range(0, m, bsz):
            chunk = order[start : start + bsz]
            grad = [0.0] * d
            for idx in chunk:
                pred = sum(a * b for a, b in zip(x[idx], theta))
                err = pred - y[idx]
                for j in range(d):
                    grad[j] += x[idx][j] * err
            scale = lr / len(chunk)
            for j in range(d):
                theta[j] -= scale * grad[j]
    return theta


def demo() -> str:
    lines: list[str] = []
    x_n, it_n = newton_minimize_1d(lambda x: 4.0 * x**3 - 4.0, lambda x: 12.0 * x * x, 3.0, 1e-12, 50)
    lines.append(f"Newton 1D on x^4 - 4x + 5 from x0=3: x* = {x_n:.10f} in {it_n} steps")

    r = Rng(3302)
    losses = [
        [0.1 + 0.2 * r.f64() if i == 2 else 0.3 + 0.6 * r.f64() for i in range(6)]
        for _ in range(200)
    ]
    hl, opt, probs = hedge(losses, 0.2)
    lines.append(
        f"Hedge (200 rounds, 6 experts): loss={hl:.2f}, best={opt:.2f}, "
        f"bound={hedge_loss_bound(opt, 6, 0.2):.2f}, top_p={probs[2]:.3f}"
    )

    xs = [[1.0, i / 10.0] for i in range(20)]
    ys = [2.0 + 3.0 * row[1] for row in xs]
    th = sgd_linear_regression(xs, ys, 0.25, 400, 4, 3304)
    lines.append(f"SGD linear regression y = 2 + 3x -> theta = [{th[0]:.4f}, {th[1]:.4f}]")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_newton_converges_quadratically_on_strictly_convex_functions(self) -> None:
        x, it = newton_minimize_1d(lambda x: 4.0 * x**3 - 4.0, lambda x: 12.0 * x * x, 4.0, 1e-12, 30)
        self.assertLess(abs(x - 1.0), 1e-10)
        self.assertLessEqual(it, 10)

        pt, it2 = newton_minimize(
            lambda p: [(p[0] - 3.0) + 0.2 * (p[1] + 2.0), 2.0 * (p[1] + 2.0) + 0.2 * (p[0] - 3.0)],
            lambda _: [[1.0, 0.2], [0.2, 2.0]],
            [-10.0, 15.0],
            1e-12,
            10,
        )
        self.assertLess(abs(pt[0] - 3.0), 1e-12)
        self.assertLess(abs(pt[1] + 2.0), 1e-12)
        self.assertEqual(it2, 1)

    def test_hedge_obeys_multiplicative_weights_regret_bound(self) -> None:
        r = Rng(3322)
        for _ in range(40):
            n = r.range(2, 10)
            rounds = r.range(20, 200)
            losses = [
                [0.25 * r.f64() if i == 0 else r.f64() for i in range(n)]
                for _ in range(rounds)
            ]
            for eps in (0.1, 0.25, 0.45):
                hl, opt, probs = hedge(losses, eps)
                self.assertLess(abs(sum(probs) - 1.0), 1e-9)
                self.assertLessEqual(hl, hedge_loss_bound(opt, n, eps) + 1e-9)

    def test_sgd_agrees_with_full_batch_gradient_descent(self) -> None:
        r = Rng(3304)
        xs = [[1.0, r.f64() * 3.0 - 1.5, r.f64() * 2.0 - 1.0] for _ in range(60)]
        ys = [1.5 - 2.0 * row[1] + 0.75 * row[2] for row in xs]
        gd = _gd_linear_regression(xs, ys, 0.15, 4_000)
        sgd = sgd_linear_regression(xs, ys, 0.15, 600, 8, 99)
        for a, b in zip(gd, sgd):
            self.assertLess(abs(a - b), 1e-2, f"{gd} vs {sgd}")


if __name__ == "__main__":
    print(demo(), end="")
