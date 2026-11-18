"""Modular Exponentiation — CLRS 31.6 (1e–4e) (1, 2, 3, 4).

a^b mod n by repeated squaring in O(β) multiplications; powers, orders and primitive roots.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def mod_pow(a: int, b: int, n: int) -> int:
    """MODULAR-EXPONENTIATION(a, b, n): scan bits of b from high to low, squaring each step."""
    if n == 1:
        return 0
    d = 1
    for i in range(b.bit_length() - 1, -1, -1):
        d = (d * d) % n
        if (b >> i) & 1:
            d = (d * a) % n
    return d


def multiplications(b: int) -> int:
    """Number of modular multiplications used: 2*(bit length of b) at most."""
    return b.bit_length() + b.bit_count()


def order(a: int, n: int) -> int | None:
    """Multiplicative order of a modulo n (smallest t > 0 with a^t ≡ 1), if gcd(a, n) = 1."""
    if math.gcd(a, n) != 1:
        return None
    x = a % n
    t = 1
    while x != 1 % n:
        x = (x * a) % n
        t += 1
    return t


def primitive_root(p: int) -> int | None:
    """Smallest primitive root modulo a prime p (generator of Z_p*)."""
    if p == 2:
        return 1
    return next((g for g in range(2, p) if order(g, p) == p - 1), None)


def demo() -> str:
    lines: list[str] = []
    lines.append(f"7^560 mod 561 = {mod_pow(7, 560, 561)}  (561 is a Carmichael number)")
    lines.append(f"2^1000000007 mod 998244353 = {mod_pow(2, 1_000_000_007, 998_244_353)}")
    lines.append(f"multiplications for an exponent with 30 bits: ≤ {multiplications((1 << 30) - 1)}")
    lines.append(f"order of 2 mod 11 = {order(2, 11)}; order of 3 mod 11 = {order(3, 11)}")
    lines.append(f"smallest primitive root mod 23 = {primitive_root(23)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_naive_power(self) -> None:
        for a in range(12):
            for b in range(12):
                for n in range(1, 40):
                    want = 1 % n
                    for _ in range(b):
                        want = (want * a) % n
                    self.assertEqual(mod_pow(a, b, n), want, f"{a}^{b} mod {n}")

    def test_fermat_and_large_moduli(self) -> None:
        r = Rng(212)
        p = 1_000_000_007
        for _ in range(50):
            a = r.range(1, 1_000_000)
            self.assertEqual(mod_pow(a, p - 1, p), 1)
        self.assertEqual(mod_pow(2, 10, 1_000_000_007), 1024)
        u64_max = (1 << 64) - 1
        self.assertLess(mod_pow(u64_max, u64_max, u64_max - 58), u64_max - 58)

    def test_orders_and_roots(self) -> None:
        self.assertEqual(order(2, 11), 10)
        self.assertEqual(order(3, 11), 5)
        self.assertIsNone(order(4, 8))
        self.assertEqual(primitive_root(23), 5)
        self.assertEqual(primitive_root(7), 3)
        self.assertEqual(multiplications(0b1011), 4 + 3)


if __name__ == "__main__":
    print(demo(), end="")
