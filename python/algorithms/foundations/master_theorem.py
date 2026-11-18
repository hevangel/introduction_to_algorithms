"""Master Theorem — CLRS 4.3 (1e–3e) / 4.5 (4e).

Solve T(n)=aT(n/b)+Θ(n^k·lg^p n) by comparing the watershed n^(log_b a) with the driving function.
"""

from __future__ import annotations

from enum import Enum
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


class Case(Enum):
    """Which case of the master theorem applies."""

    ONE = "One"
    TWO = "Two"
    THREE = "Three"
    GAP = "Gap"


def solve(a: float, b: float, k: float, p: float) -> tuple[Case, str]:
    """Solve `T(n) = a·T(n/b) + Θ(n^k · (lg n)^p)`, returning the case and the Θ-bound as text."""
    crit = math.log(a) / math.log(b)

    def fmt(e: float, lg: float) -> str:
        if abs(e) < 1e-9:
            pw = ""
        elif abs(e - 1.0) < 1e-9:
            pw = "n"
        elif abs(e - round(e)) < 1e-9:
            pw = f"n^{round(e)}"
        else:
            pw = f"n^{e:.3f}"

        if abs(lg) < 1e-9:
            log = ""
        elif abs(lg - 1.0) < 1e-9:
            log = "lg n"
        else:
            lg_str = f"{int(lg)}" if lg == int(lg) else f"{lg}"
            log = f"lg^{lg_str} n"

        if not pw and not log:
            return "1"
        if not pw:
            return log
        if not log:
            return pw
        return f"{pw} {log}"

    if abs(k - crit) < 1e-9:
        if p > -1.0:
            return Case.TWO, f"Θ({fmt(k, p + 1.0)})"
        if abs(p + 1.0) < 1e-9:
            return Case.TWO, f"Θ({fmt(k, 0.0)} lg lg n)"
        return Case.TWO, f"Θ({fmt(k, 0.0)})"
    if k < crit:
        return Case.ONE, f"Θ({fmt(crit, 0.0)})"
    if p >= 0.0:
        return Case.THREE, f"Θ({fmt(k, p)})"
    return Case.GAP, "master theorem does not apply"


def evaluate(a: int, b: int, k: float, n: int) -> float:
    """Numerically evaluate T(n) = a·T(⌈n/b⌉) + n^k (n ≥ 1, T(1)=1)."""
    if n <= 1:
        return 1.0
    return a * evaluate(a, b, k, (n + b - 1) // b) + (float(n) ** k)


def demo() -> str:
    lines: list[str] = []
    rows = [
        ("merge sort            T=2T(n/2)+n", 2.0, 2.0, 1.0, 0.0),
        ("binary search         T=T(n/2)+1", 1.0, 2.0, 0.0, 0.0),
        ("Strassen              T=7T(n/2)+n²", 7.0, 2.0, 2.0, 0.0),
        ("recursive matrix mult T=8T(n/2)+n²", 8.0, 2.0, 2.0, 0.0),
        ("T=3T(n/4)+n lg n", 3.0, 4.0, 1.0, 1.0),
        ("T=2T(n/2)+n²", 2.0, 2.0, 2.0, 0.0),
        ("T=2T(n/2)+n/lg n", 2.0, 2.0, 1.0, -1.0),
    ]
    for name, a, b, k, p in rows:
        c, s = solve(a, b, k, p)
        lines.append(f"{name:<38} case {c.value}: {s}")
    n = 1 << 20
    lines.append(f"\nnumeric check, T(n)=2T(n/2)+n: T(2^20)/(n lg n) = {evaluate(2, 2, 1.0, n) / (n * 20.0):.3f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_classic_recurrences(self) -> None:
        self.assertEqual(solve(2.0, 2.0, 1.0, 0.0), (Case.TWO, "Θ(n lg n)"))
        self.assertEqual(solve(1.0, 2.0, 0.0, 0.0), (Case.TWO, "Θ(lg n)"))
        self.assertEqual(solve(8.0, 2.0, 2.0, 0.0), (Case.ONE, "Θ(n^3)"))
        self.assertEqual(solve(2.0, 2.0, 2.0, 0.0), (Case.THREE, "Θ(n^2)"))
        self.assertEqual(solve(7.0, 2.0, 2.0, 0.0)[0], Case.ONE)
        self.assertEqual(solve(2.0, 2.0, 1.0, -1.0), (Case.TWO, "Θ(n lg lg n)"))
        self.assertEqual(solve(2.0, 2.0, 1.0, -2.0)[1], "Θ(n)")

    def test_numeric_agrees_with_theory(self) -> None:
        n = 1 << 16
        ratio = evaluate(2, 2, 1.0, n) / (n * 16.0)
        self.assertTrue(0.9 <= ratio < 1.2)
        r2 = evaluate(8, 2, 2.0, n) / (float(n) ** 3)
        self.assertTrue(1.0 <= r2 < 2.1)


if __name__ == "__main__":
    print(demo(), end="")
