"""Primality Testing: Fermat & Miller-Rabin — CLRS 31.8 (1e–4e) (1, 2, 3, 4).

The pseudoprimality test (fooled by Carmichael numbers) and the randomized Miller-Rabin test with error ≤ 2^−s.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def pseudoprime(n: int) -> bool:
    """PSEUDOPRIME(n): is 2^{n-1} ≡ 1 (mod n)?"""
    return n >= 2 and pow(2, n - 1, n) == 1


def witness(a: int, n: int) -> bool:
    """WITNESS(a, n): True if a proves that n is composite (n odd, > 2)."""
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


def miller_rabin(n: int, s: int, rng: Rng) -> bool:
    """MILLER-RABIN(n, s): s random bases; composite is exact, prime error <= 2^-s."""
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0:
        return False
    return all(not witness(2 + rng.next_u64() % (n - 3), n) for _ in range(s))


def is_prime(n: int) -> bool:
    """Deterministic for all 64-bit n: the first 12 primes as bases suffice."""
    if n < 2:
        return False
    bases = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for p in bases:
        if n % p == 0:
            return n == p
    return not any(witness(a, n) for a in bases)


def demo() -> str:
    lines: list[str] = []
    r = Rng(1)
    lines.append("n    | pseudoprime(2) | Miller-Rabin (s=10) | deterministic is_prime")
    for n in (97, 561, 1105, 1729, 2047, 7919, 1_000_000_007, 1_000_000_007 * 3):
        lines.append(f"{n:<12} | {str(pseudoprime(n)):<5} | {str(miller_rabin(n, 10, r)):<5} | {is_prime(n)}")
    lines.append(
        f"561 = 3·11·17 is a Carmichael number: it passes Fermat tests for every base coprime to it, yet witness(2, 561) = {witness(2, 561)}"
    )
    primes = sum(1 for n in range(2, 100) if is_prime(n))
    lines.append(f"primes below 100: {primes}")
    return "\n".join(lines) + "\n"


def _sieve(n: int) -> list[bool]:
    s = [True] * (n + 1)
    s[0] = False
    if n >= 1:
        s[1] = False
    for i in range(2, n + 1):
        if s[i]:
            j = i * i
            while j <= n:
                s[j] = False
                j += i
    return s


class Tests(unittest.TestCase):
    def test_deterministic_test_matches_sieve(self) -> None:
        s = _sieve(30_000)
        for n in range(30_001):
            self.assertEqual(is_prime(n), s[n], f"n={n}")

    def test_carmichael_numbers_fool_fermat_but_not_miller_rabin(self) -> None:
        for n in (561, 1105, 1729, 2465, 2821, 6601):
            self.assertTrue(pseudoprime(n), f"{n}")
            self.assertTrue(witness(2, n) or any(witness(a, n) for a in range(2, n)))
            self.assertFalse(is_prime(n))
        self.assertTrue(pseudoprime(341))
        self.assertTrue(witness(2, 561))

    def test_randomized_variant_and_large_primes(self) -> None:
        r = Rng(213)
        s = _sieve(5000)
        for n in range(5001):
            self.assertEqual(miller_rabin(n, 20, r), s[n], f"n={n}")
        self.assertTrue(is_prime(18_446_744_073_709_551_557))
        self.assertFalse(is_prime(18_446_744_073_709_551_556))
        self.assertTrue(is_prime(1_000_000_007))
        self.assertFalse(is_prime((1_000_000_007 * 998_244_353) | 1 | 4))


if __name__ == "__main__":
    print(demo(), end="")
