"""Polynomial Evaluation & Lagrange Interpolation — CLRS 30.1 (1e–4e) (1, 2, 3, 4).

Recover coefficients from n point-value pairs in Θ(n²) using Lagrange's formula; evaluate with Horner.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def evaluate(c: list[float], x: float) -> float:
    """Horner evaluation of sum c_i x^i."""
    acc = 0.0
    for a in reversed(c):
        acc = acc * x + a
    return acc


def interpolate(xs: list[float], ys: list[float]) -> list[float]:
    """Lagrange interpolation: unique polynomial of degree < n through (xs[i], ys[i]). Theta(n^2)."""
    n = len(xs)
    result = [0.0] * n
    for i in range(n):
        basis = [1.0]
        denom = 1.0
        for j in range(n):
            if j == i:
                continue
            nxt = [0.0] * (len(basis) + 1)
            for k, b in enumerate(basis):
                nxt[k + 1] += b
                nxt[k] -= b * xs[j]
            basis = nxt
            denom *= xs[i] - xs[j]
        for k, b in enumerate(basis):
            result[k] += ys[i] * b / denom
    return result


def evaluate_many(c: list[float], xs: list[float]) -> list[float]:
    """Evaluate at many points: Theta(n*m) with Horner."""
    return [evaluate(c, x) for x in xs]


def demo() -> str:
    lines: list[str] = []
    c = [2.0, -3.0, 0.0, 1.0]
    xs = [-2.0, -1.0, 0.0, 1.0, 2.0]
    ys = evaluate_many(c, xs)
    lines.append(f"P(x) = 2 − 3x + x³ sampled at {xs} → {ys}")
    rec = interpolate(xs[:4], ys[:4])
    lines.append(f"Lagrange from 4 samples: {[round(v * 1e9) / 1e9 for v in rec]}")
    lines.append("uniqueness: a degree-3 polynomial is fixed by any 4 distinct points")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_roundtrip_recovers_coefficients(self) -> None:
        r = Rng(203)
        for n in range(1, 9):
            c = [r.f64() * 10.0 - 5.0 for _ in range(n)]
            xs = [i - n / 2.0 for i in range(n)]
            back = interpolate(xs, evaluate_many(c, xs))
            for i in range(n):
                self.assertLess(abs(back[i] - c[i]), 1e-6, f"n={n} i={i}")

    def test_horner_and_line_through_two_points(self) -> None:
        self.assertEqual(evaluate([2.0, -3.0, 0.0, 1.0], 2.0), 4.0)
        line = interpolate([1.0, 3.0], [5.0, 9.0])
        self.assertLess(abs(line[0] - 3.0), 1e-12)
        self.assertLess(abs(line[1] - 2.0), 1e-12)


if __name__ == "__main__":
    print(demo(), end="")
