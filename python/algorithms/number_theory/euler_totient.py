"""Euler's Totient Function — CLRS 31.3 (1e–4e) (1, 2, 3, 4).

φ(n) counts residues coprime to n: φ(n) = n·∏(1 − 1/p) over primes p | n; sieve all values up to N in O(N lg lg N); Euler's theorem aᵠ⁽ⁿ⁾ ≡ 1.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def totient(n: int) -> int:
    """phi(n) by trial-division factorisation, O(sqrt(n))."""
    result = n
    p = 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result -= result // p
        p += 1
    if n > 1:
        result -= result // n
    return result


def totient_sieve(limit: int) -> list[int]:
    """phi(1..=limit) with a sieve: O(N lg lg N)."""
    phi = list(range(limit + 1))
    for p in range(2, limit + 1):
        if phi[p] == p:
            for m in range(p, limit + 1, p):
                phi[m] -= phi[m] // p
    return phi


def demo() -> str:
    lines: list[str] = []
    phi = totient_sieve(30)
    lines.append(f"φ(1..30) = {phi[1:]}")
    for n in (36, 97, 1000, 123_456_789):
        lines.append(f"φ({n}) = {totient(n)}")
    n = 1000
    s = sum(totient(d) for d in range(1, n + 1) if n % d == 0)
    lines.append(f"Σ_{{d | {n}}} φ(d) = {s} = n")
    lines.append(f"Euler's theorem: 7^φ(100) mod 100 = {pow(7, totient(100), 100)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_counting_coprime_residues(self) -> None:
        phi = totient_sieve(500)
        for n in range(1, 501):
            brute = sum(1 for k in range(1, n + 1) if math.gcd(k, n) == 1)
            self.assertEqual(totient(n), brute, f"n = {n}")
            self.assertEqual(phi[n], brute)

    def test_identities(self) -> None:
        for n in range(1, 300):
            self.assertEqual(sum(totient(d) for d in range(1, n + 1) if n % d == 0), n)
            for a in range(1, max(n, 2)):
                if math.gcd(a, n) == 1:
                    self.assertEqual(pow(a, totient(n), n), 1 % n)
        self.assertEqual(totient(97), 96)


if __name__ == "__main__":
    print(demo(), end="")
