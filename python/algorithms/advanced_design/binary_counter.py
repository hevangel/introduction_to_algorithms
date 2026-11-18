"""Amortized Analysis: Binary Counter — CLRS 17.1–17.3 (2e–3e) / 16 (4e) / 18 (1e) (1e,2e,3e,4e).

k-bit counter INCREMENT: aggregate, accounting and potential methods give 2 amortized flips per increment.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


class BinaryCounter:
    """A k-bit binary counter (least-significant bit first)."""

    def __init__(self, k: int) -> None:
        self._bits: list[bool] = [False] * k
        self.total_cost: int = 0

    def value(self) -> int:
        return sum((1 << i) for i, b in enumerate(self._bits) if b)

    def ones(self) -> int:
        """Number of 1 bits — the potential Phi."""
        return sum(1 for b in self._bits if b)

    def increment(self) -> int:
        """INCREMENT: returns the actual cost (bits flipped)."""
        i = 0
        while i < len(self._bits) and self._bits[i]:
            self._bits[i] = False
            i += 1
        if i < len(self._bits):
            self._bits[i] = True
            i += 1
        self.total_cost += i
        return i

    def decrement(self) -> int:
        """DECREMENT (Exercise 17.1-3 setting): returns the flips; wraps at 0."""
        i = 0
        while i < len(self._bits) and not self._bits[i]:
            self._bits[i] = True
            i += 1
        if i < len(self._bits):
            self._bits[i] = False
            i += 1
        self.total_cost += i
        return i


def amortized_cost(c: BinaryCounter) -> int:
    """Potential-method amortized cost of one increment: actual + delta_Phi."""
    before = c.ones()
    actual = c.increment()
    return actual + c.ones() - before


def demo() -> str:
    lines: list[str] = []
    c = BinaryCounter(8)
    lines.append(" n | value bits | actual cost | total | Φ (ones) | amortized")
    for n in range(1, 17):
        ones_before = c.ones()
        cost = c.increment()
        amort = cost + c.ones() - ones_before
        lines.append(f"{n:>2} | {c.value():>10} | {cost:>11} | {c.total_cost:>5} | {c.ones():>8} | {amort}")
    big = BinaryCounter(32)
    for _ in range(100_000):
        big.increment()
    lines.append(f"100000 increments on a 32-bit counter cost {big.total_cost} flips (< 2n = 200000)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_amortized_cost_is_at_most_two(self) -> None:
        c = BinaryCounter(20)
        for _ in range(5000):
            self.assertLessEqual(amortized_cost(c), 2)

    def test_total_flips_formula(self) -> None:
        c = BinaryCounter(20)
        for n in range(1, 3001):
            c.increment()
            self.assertEqual(c.value(), n)
            self.assertEqual(c.total_cost, 2 * n - n.bit_count())

    def test_decrement_undoes_increment(self) -> None:
        c = BinaryCounter(10)
        for _ in range(37):
            c.increment()
        for _ in range(37):
            c.decrement()
        self.assertEqual(c.value(), 0)
        d = BinaryCounter(10)
        for _ in range(511):
            d.increment()
        before = d.total_cost
        for _ in range(10):
            d.increment()
            d.decrement()
        self.assertGreaterEqual(d.total_cost - before, 10 * 2 * 9)


if __name__ == "__main__":
    print(demo(), end="")
