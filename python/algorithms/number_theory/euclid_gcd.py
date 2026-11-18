"""Euclid's GCD Algorithm — CLRS 31.2 (1e–4e); Problem 31-1 binary gcd (1, 2, 3, 4).

gcd(a, b) = gcd(b, a mod b): O(lg b) steps (Lamé's theorem); plus binary GCD and lcm.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def gcd(a: int, b: int) -> int:
    """EUCLID(a, b), iterative."""
    while b != 0:
        a, b = b, a % b
    return a


def gcd_steps(a: int, b: int) -> int:
    """Number of division steps EUCLID performs."""
    s = 0
    while b != 0:
        a, b = b, a % b
        s += 1
    return s


def binary_gcd(a: int, b: int) -> int:
    """Problem 31-1: binary gcd using only subtraction, parity tests and shifts."""
    if a == 0:
        return b
    if b == 0:
        return a
    ab = a | b
    shift = (ab & -ab).bit_length() - 1
    a >>= (a & -a).bit_length() - 1
    while True:
        b >>= (b & -b).bit_length() - 1
        if a > b:
            a, b = b, a
        b -= a
        if b == 0:
            return a << shift


def lcm(a: int, b: int) -> int:
    if a == 0 or b == 0:
        return 0
    return (a // gcd(a, b)) * b


def gcd_many(xs: list[int]) -> int:
    """gcd of a list (Exercise 31.2-8)."""
    g = 0
    for x in xs:
        g = gcd(g, x)
    return g


def demo() -> str:
    lines: list[str] = []
    lines.append(f"EUCLID(30, 21) = {gcd(30, 21)}   binary_gcd(30, 21) = {binary_gcd(30, 21)}")
    lines.append(f"gcd(99, 78) = {gcd(99, 78)}, lcm(4, 6) = {lcm(4, 6)}, gcd of [12, 18, 30] = {gcd_many([12, 18, 30])}")
    lines.append("worst case: consecutive Fibonacci numbers")
    a, b = 1, 1
    for k in range(2, 41):
        a, b = b, a + b
        if k % 8 == 0:
            lines.append(f"  gcd(F{k + 1}, F{k}) takes {gcd_steps(b, a)} steps")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_known_values(self) -> None:
        self.assertEqual((gcd(30, 21), gcd(99, 78), gcd(0, 5), gcd(7, 0), gcd(0, 0)), (3, 3, 5, 7, 0))
        self.assertEqual(lcm(4, 6), 12)
        self.assertEqual(gcd_many([12, 18, 30]), 6)
        self.assertEqual(gcd_many([]), 0)

    def test_binary_equals_euclid(self) -> None:
        r = Rng(210)
        for _ in range(2000):
            a = r.next_u64() >> r.range(0, 60)
            b = r.next_u64() >> r.range(0, 60)
            self.assertEqual(binary_gcd(a, b), gcd(a, b))

    def test_lame_bound_on_fibonacci(self) -> None:
        a, b = 1, 2
        phi = 1.618033988749895
        for k in range(1, 60):
            a, b = b, a + b
            self.assertLessEqual(gcd_steps(b, a), math.log(b) / math.log(phi) + 2.0, f"k={k}")


if __name__ == "__main__":
    print(demo(), end="")
