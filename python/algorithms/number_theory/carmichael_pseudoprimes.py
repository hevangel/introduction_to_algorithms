"""Pseudoprimes and Carmichael Numbers — CLRS 31.8 (1e–4e) (1, 2, 3, 4).

Fermat's test a^(n−1) ≡ 1 (mod n) never rejects primes but also accepts composites: base-2 pseudoprimes and the Carmichael numbers that fool every coprime base (Korselt's criterion).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def is_prime_trial(n: int) -> bool:
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def fermat_passes(n: int, a: int) -> bool:
    """Fermat test to base a: does a^(n-1) ≡ 1 (mod n)?"""
    if n == 1:
        return False
    return pow(a, n - 1, n) == 1


def base2_pseudoprimes(limit: int) -> list[int]:
    """Composite numbers that pass the base-2 test (PSEUDOPRIME), up to limit."""
    return [n for n in range(3, limit + 1) if not is_prime_trial(n) and fermat_passes(n, 2)]


def is_carmichael_by_definition(n: int) -> bool:
    """By definition: composite n for which every a coprime to n passes the Fermat test."""
    return (
        n > 2
        and not is_prime_trial(n)
        and all(fermat_passes(n, a) for a in range(1, n) if math.gcd(a, n) == 1)
    )


def is_carmichael_korselt(n: int) -> bool:
    """Korselt's criterion: composite, squarefree, and (p - 1) | (n - 1) for every prime p | n."""
    if n < 4 or is_prime_trial(n):
        return False
    m = n
    p = 2
    factors = 0
    while p * p <= m:
        if m % p == 0:
            m //= p
            if m % p == 0 or (n - 1) % (p - 1) != 0:
                return False
            factors += 1
        p += 1
    if m > 1:
        if (n - 1) % (m - 1) != 0:
            return False
        factors += 1
    return factors >= 2


def demo() -> str:
    lines: list[str] = []
    lines.append(f"base-2 pseudoprimes below 1000: {base2_pseudoprimes(1000)}")
    carm = [n for n in range(2, 20_000) if is_carmichael_korselt(n)]
    lines.append(f"Carmichael numbers below 20000 (Korselt): {carm}")
    lines.append(
        f"561 passes the Fermat test for every coprime base: {is_carmichael_by_definition(561)}"
    )
    lines.append(
        f"341 = 11·31 is a base-2 pseudoprime, but base 3 exposes it: 2→{fermat_passes(341, 2)}, 3→{fermat_passes(341, 3)}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_definition_equals_korselt_below_3000(self) -> None:
        for n in range(2, 3000):
            self.assertEqual(is_carmichael_by_definition(n), is_carmichael_korselt(n), f"n = {n}")

    def test_known_values(self) -> None:
        self.assertEqual(base2_pseudoprimes(1000), [341, 561, 645])
        c = [n for n in range(2, 10_000) if is_carmichael_korselt(n)]
        self.assertEqual(c, [561, 1105, 1729, 2465, 2821, 6601, 8911])
        self.assertFalse(fermat_passes(341, 3))
        self.assertTrue(
            all(
                all(fermat_passes(p, a) for a in range(1, p))
                for p in range(2, 100)
                if is_prime_trial(p)
            )
        )


if __name__ == "__main__":
    print(demo(), end="")
