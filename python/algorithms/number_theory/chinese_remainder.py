"""Chinese Remainder Theorem — CLRS 31.5 (1e–4e) (1, 2, 3, 4).

Reconstruct x mod n₁n₂…n_k from residues modulo pairwise coprime moduli; also merging non-coprime congruences.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def _extended_euclid(a: int, b: int) -> tuple[int, int, int]:
    if b == 0:
        return (a, 1, 0)
    d, x, y = _extended_euclid(b, a % b)
    return (d, y, x - (a // b) * y)


def _mod_inverse(a: int, n: int) -> int | None:
    d, x, _ = _extended_euclid(a % n, n)
    return (x % n) if d == 1 else None


def crt(residues: list[int], moduli: list[int]) -> tuple[int, int] | None:
    """CRT for pairwise relatively prime moduli: returns (x, n) with x ≡ a_i (mod n_i)."""
    n = 1
    for m in moduli:
        n *= m
    x = 0
    for a, ni in zip(residues, moduli):
        mi = n // ni
        inv = _mod_inverse(mi % ni, ni)
        if inv is None:
            return None
        x += a * mi * inv
    return (x % n, n)


def merge(a1: int, n1: int, a2: int, n2: int) -> tuple[int, int] | None:
    """Merge x ≡ a1 (mod n1) and x ≡ a2 (mod n2) for arbitrary moduli -> (x, lcm)."""
    d, p, _ = _extended_euclid(n1, n2)
    if (a2 - a1) % d != 0:
        return None
    lcm = (n1 // d) * n2
    t = ((a2 - a1) // d) * p % (n2 // d)
    x = (a1 + n1 * t) % lcm
    return (x, lcm)


def crt_general(residues: list[int], moduli: list[int]) -> tuple[int, int] | None:
    x, n = 0, 1
    for a, m in zip(residues, moduli):
        merged = merge(x, n, a % m, m)
        if merged is None:
            return None
        x, n = merged
    return (x, n)


def demo() -> str:
    lines: list[str] = []
    lines.append(f"x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7) → {crt([2, 3, 2], [3, 5, 7])}")
    lines.append(f"x ≡ 1 (mod 5), x ≡ 2 (mod 7) → {crt([1, 2], [5, 7])}")
    lines.append(f"non-coprime moduli: x ≡ 3 (mod 4), x ≡ 5 (mod 6) → {crt_general([3, 5], [4, 6])}")
    lines.append(f"inconsistent: x ≡ 1 (mod 4), x ≡ 2 (mod 6) → {crt_general([1, 2], [4, 6])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sun_tzu(self) -> None:
        self.assertEqual(crt([2, 3, 2], [3, 5, 7]), (23, 105))
        self.assertEqual(crt([1, 2], [5, 7]), (16, 35))

    def test_one_to_one_correspondence(self) -> None:
        mods, n = [3, 5, 7], 105
        for x in range(n):
            res = [x % m for m in mods]
            self.assertEqual(crt(res, mods), (x, n))
            self.assertEqual(crt_general(res, mods), (x, n))

    def test_non_coprime_moduli(self) -> None:
        self.assertEqual(crt_general([3, 5], [4, 6]), (11, 12))
        self.assertIsNone(crt_general([1, 2], [4, 6]))
        for m1, m2 in [(4, 6), (6, 9), (12, 18)]:
            lcm_val = (m1 // math.gcd(m1, m2)) * m2
            for a1 in range(m1):
                for a2 in range(m2):
                    brute = next((x for x in range(lcm_val) if x % m1 == a1 and x % m2 == a2), None)
                    got = crt_general([a1, a2], [m1, m2])
                    self.assertEqual(got[0] if got is not None else None, brute)


if __name__ == "__main__":
    print(demo(), end="")
