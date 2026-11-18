"""Horner's Rule — CLRS Problem 2-3 (1e–4e).

Evaluate a degree-n polynomial with n multiplications and n additions.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def horner(coeffs: list[int], x: int) -> int:
    """HORNER: y = a0 + x(a1 + x(a2 + … + x·an)); `coeffs[i]` is aᵢ."""
    y = 0
    for a in reversed(coeffs):
        y = a + x * y
    return y


def naive_eval(coeffs: list[int], x: int) -> int:
    """Naive Θ(n²)-multiplication evaluation (computes every power from scratch)."""
    y = 0
    for i, a in enumerate(coeffs):
        p = 1
        for _ in range(i):
            p *= x
        y += a * p
    return y


def running_power_eval(coeffs: list[int], x: int) -> int:
    """Θ(n) evaluation that keeps a running power of x."""
    y, p = 0, 1
    for a in coeffs:
        y += a * p
        p *= x
    return y


def demo() -> str:
    lines: list[str] = []
    c = [1, -2, 0, 3]
    for x in [-2, 0, 1, 3]:
        lines.append(
            f"P({x:>2}) horner={horner(c, x):>4} naive={naive_eval(c, x):>4} running={running_power_eval(c, x):>4}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_all_three_agree(self) -> None:
        c = [5, -3, 2, 0, 7, 1]
        for x in range(-4, 5):
            self.assertEqual(horner(c, x), naive_eval(c, x))
            self.assertEqual(horner(c, x), running_power_eval(c, x))
        self.assertEqual(horner([], 3), 0)
        self.assertEqual(horner([1, -2, 0, 3], 2), 1 - 4 + 24)


if __name__ == "__main__":
    print(demo(), end="")
