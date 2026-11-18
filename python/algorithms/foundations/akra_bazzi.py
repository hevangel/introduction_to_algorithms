"""Akra–Bazzi Recurrence Method — CLRS 4.7 (4e).

Solves recurrences with unequal subproblem sizes: find p with Σ aᵢ·bᵢ^p = 1, then integrate.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def find_p(terms: list[tuple[float, float]]) -> float:
    """Find `p` such that Σ aᵢ·bᵢ^p = 1 for T(n) = Σ aᵢ·T(bᵢ·n) + g(n); aᵢ > 0, 0 < bᵢ < 1."""

    def f(p: float) -> float:
        return sum(a * (b**p) for a, b in terms) - 1.0

    lo, hi = -50.0, 50.0
    for _ in range(200):
        mid = (lo + hi) / 2.0
        if f(mid) > 0.0:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2.0


def solve(terms: list[tuple[float, float]], k: float, q: float) -> tuple[float, str]:
    """Asymptotic solution for g(n) = n^k · lg^q n as text."""
    p = find_p(terms)

    def fmt(e: float, lg_pow: float) -> str:
        if lg_pow == 0.0:
            return f"Θ(n^{e:.3f})"
        lg_str = f"{int(lg_pow)}" if lg_pow == int(lg_pow) else f"{lg_pow}"
        return f"Θ(n^{e:.3f} lg^{lg_str} n)"

    eps = 1e-6
    if abs(k - p) < eps:
        if q > -1.0:
            s = fmt(p, q + 1.0)
        else:
            s = fmt(p, 0.0) + " (q ≤ −1: lg lg n or constant factor)"
    elif k < p:
        s = fmt(p, 0.0)
    else:
        s = fmt(k, q)
    return p, s


def evaluate(terms: list[tuple[float, float]], n: int, memo: dict[int, float]) -> float:
    """Numeric T(n) = Σ aᵢ·T(⌊bᵢ n⌋) + n (T(n)=1 for n<2), memoised."""
    if n < 2:
        return 1.0
    if n in memo:
        return memo[n]
    v = sum(a * evaluate(terms, math.floor(b * n), memo) for a, b in terms) + float(n)
    memo[n] = v
    return v


def demo() -> str:
    lines: list[str] = []
    t1 = [(1.0, 1.0 / 3.0), (1.0, 2.0 / 3.0)]
    p, s = solve(t1, 1.0, 0.0)
    lines.append(f"T(n)=T(n/3)+T(2n/3)+n      : p={p:.4f} → {s}")
    t2 = [(2.0, 0.5)]
    p, s = solve(t2, 1.0, 0.0)
    lines.append(f"T(n)=2T(n/2)+n             : p={p:.4f} → {s}")
    t3 = [(1.0, 0.5), (1.0, 0.25)]
    p, s = solve(t3, 1.0, 0.0)
    lines.append(f"T(n)=T(n/2)+T(n/4)+n       : p={p:.4f} → {s}")
    m: dict[int, float] = {}
    n = 1 << 20
    lines.append(
        f"numeric T(2^20)/(n lg n) for the first recurrence = {evaluate(t1, n, m) / (n * 20.0):.3f}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_p_values(self) -> None:
        self.assertAlmostEqual(find_p([(1.0, 1.0 / 3.0), (1.0, 2.0 / 3.0)]), 1.0, delta=1e-9)
        self.assertAlmostEqual(find_p([(2.0, 0.5)]), 1.0, delta=1e-9)
        self.assertAlmostEqual(find_p([(7.0, 0.5)]), math.log2(7.0), delta=1e-9)

    def test_solutions(self) -> None:
        self.assertEqual(solve([(1.0, 1.0 / 3.0), (1.0, 2.0 / 3.0)], 1.0, 0.0)[1], "Θ(n^1.000 lg^1 n)")
        self.assertEqual(solve([(1.0, 0.5), (1.0, 0.25)], 1.0, 0.0)[1], "Θ(n^1.000)")

    def test_numeric_matches(self) -> None:
        t = [(1.0, 1.0 / 3.0), (1.0, 2.0 / 3.0)]
        m: dict[int, float] = {}
        n = 1 << 18
        r = evaluate(t, n, m) / (n * 18.0)
        self.assertTrue(0.5 <= r < 2.0, f"ratio {r}")


if __name__ == "__main__":
    print(demo(), end="")
