"""Pollard's Rho Integer Factorization — CLRS 31.9 (1e–3e) (1, 2, 3).

Find a factor of n in expected O(n^¼) steps by detecting a cycle of x ↦ x²+c mod n.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _witness(a: int, n: int) -> bool:
    t = 0
    u = n - 1
    while u % 2 == 0:
        u //= 2
        t += 1
    x = pow(a, u, n)
    for _ in range(t):
        y = (x * x) % n
        if y == 1 and x != 1 and x != n - 1:
            return True
        x = y
    return x != 1


def _is_prime(n: int) -> bool:
    if n < 2:
        return False
    bases = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for p in bases:
        if n % p == 0:
            return n == p
    return not any(_witness(a, n) for a in bases)


def pollard_rho(n: int, rng: Rng) -> int:
    """POLLARD-RHO(n): returns a nontrivial factor of composite n."""
    if n % 2 == 0:
        return 2
    while True:
        c = 1 + rng.next_u64() % (n - 1)
        x = 2 + rng.next_u64() % (n - 2)
        y = x
        d = 1
        while d == 1:
            x = (x * x + c) % n
            y = (y * y + c) % n
            y = (y * y + c) % n
            d = math.gcd(abs(x - y), n)
        if d != n:
            return d


def factorize(n: int, rng: Rng) -> list[int]:
    """Complete prime factorisation (sorted with multiplicity)."""
    f: list[int] = []
    for p in range(2, 100):
        while n % p == 0:
            f.append(p)
            n //= p

    def rec(m: int) -> None:
        if m == 1:
            return
        if _is_prime(m):
            f.append(m)
            return
        d = pollard_rho(m, rng)
        rec(d)
        rec(m // d)

    rec(n)
    f.sort()
    return f


def demo() -> str:
    lines: list[str] = []
    r = Rng(5)
    lines.append(f"POLLARD-RHO(1387) = {pollard_rho(1387, r)}  (1387 = 19·73)")
    for n in (8051, 10_403, 600_851_475_143, 1_000_000_016_000_000_063, 999_999_999_989 * 3):
        lines.append(f"{n} = {factorize(n, r)}")
    n = 1_000_003 * 999_983
    lines.append(f"semiprime {n}: rho finds {pollard_rho(n, r)} (≈ n^¼ = {n**0.25:.0f} iterations expected)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_finds_nontrivial_factors(self) -> None:
        r = Rng(216)
        for n in (1387, 8051, 10_403, 455_459, 1_000_003 * 999_983):
            d = pollard_rho(n, r)
            self.assertTrue(1 < d < n and n % d == 0, f"n={n} d={d}")

    def test_full_factorisation_multiplies_back(self) -> None:
        r = Rng(217)
        for _ in range(100):
            n = 2 + r.next_u64() % 1_000_000_000_000
            f = factorize(n, r)
            prod = 1
            for p in f:
                prod *= p
                self.assertTrue(_is_prime(p))
            self.assertEqual(prod, n)
        self.assertEqual(factorize(1, r), [])
        self.assertEqual(factorize(600_851_475_143, r), [71, 839, 1471, 6857])
        self.assertEqual(factorize(1 << 40, r), [2] * 40)


if __name__ == "__main__":
    print(demo(), end="")
