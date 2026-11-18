"""Extended Euclid & Modular Inverses — CLRS 31.2 (1e–4e) (1, 2, 3, 4).

EXTENDED-EUCLID finds x, y with ax + by = gcd(a,b) (Bézout); gives modular inverses.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def extended_euclid(a: int, b: int) -> tuple[int, int, int]:
    """EXTENDED-EUCLID(a, b) = (d, x, y) with d = gcd(a, b) = a*x + b*y."""
    if b == 0:
        return (a, 1, 0)
    d, x, y = extended_euclid(b, a % b)
    return (d, y, x - (a // b) * y)


def mod_inverse(a: int, n: int) -> int | None:
    """a^{-1} mod n if gcd(a, n) = 1."""
    d, x, _ = extended_euclid(a % n, n)
    return (x % n) if d == 1 else None


def demo() -> str:
    lines: list[str] = []
    for a, b in [(99, 78), (240, 46), (17, 5)]:
        d, x, y = extended_euclid(a, b)
        lines.append(f"EXTENDED-EUCLID({a}, {b}) = (d={d}, x={x}, y={y}):  {a}·({x}) + {b}·({y}) = {a * x + b * y}")
    lines.append(f"inverse of 3 mod 11 = {mod_inverse(3, 11)}; inverse of 6 mod 9 = {mod_inverse(6, 9)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_bezout_identity(self) -> None:
        r = Rng(211)
        for _ in range(500):
            a, b = r.range(0, 100_000), r.range(0, 100_000)
            d, x, y = extended_euclid(a, b)
            self.assertEqual(a * x + b * y, d)
            self.assertEqual(d, math.gcd(a, b))
        self.assertEqual(extended_euclid(99, 78), (3, -11, 14))

    def test_inverses(self) -> None:
        self.assertEqual(mod_inverse(3, 11), 4)
        self.assertIsNone(mod_inverse(6, 9))
        for n in (7, 26, 101, 1000):
            for a in range(1, min(n, 60)):
                inv = mod_inverse(a, n)
                if inv is not None:
                    self.assertEqual((a * inv) % n, 1)


if __name__ == "__main__":
    print(demo(), end="")
